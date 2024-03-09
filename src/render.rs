/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use crate::config::Config;
use anyhow::Result;
use markdown_table::{Heading, MarkdownTable};
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::Write;
use std::path::PathBuf;

pub struct MarkdownRenderer {
    param_section_pattern: Regex,
}

impl MarkdownRenderer {
    pub fn new(config: &Config) -> MarkdownRenderer {
        let param_section_pattern = Regex::new(&format!(
            r"^(##+) {}",
            regex::escape(&config.regexp.params_section_title)
        ))
        .unwrap();

        MarkdownRenderer {
            param_section_pattern,
        }
    }

    /// Modify the given markdown file (e.g. README.md) to update the parameters section
    pub fn render(&self, markdown_path: &PathBuf) -> Result<()> {
        let md_file = File::open(markdown_path)?;
        let reader = io::BufReader::new(md_file);

        let mut new_content = Vec::with_capacity(100);

        let mut param_section_level: Option<String> = None;
        let mut next_section_pattern: Option<Regex> = None;
        let mut next_section_found = false;

        for (line_idx, read_res) in reader.lines().enumerate() {
            match read_res {
                Ok(line) => {
                    if let Some(section_level) = self.try_find_param_section(&line).take() {
                        log::debug!(
                            "The parameters section is found at line {} (level: {})",
                            line_idx + 1,
                            section_level
                        );

                        next_section_pattern = Some(Regex::new(&format!(
                            r"^{}\s", regex::escape(&section_level.clone())
                        ))?);

                        param_section_level = Some(section_level);

                        let param_table = self.render_params()?;

                        new_content.push(format!("{}\n", line));
                        new_content.push(param_table.to_string());

                        continue;
                    }

                    if param_section_level.is_some() && !next_section_found {
                        if let Some(section_pattern) = next_section_pattern.as_ref() {
                            if section_pattern.is_match(&line) {
                                next_section_found = true;

                                log::debug!(
                                    "The next section is found at line {}",
                                    line_idx + 1,
                                );
                            } else {
                                log::debug!("Skip line {} (the old parameters section): {}", line_idx + 1, line);
                                continue;
                            }
                        }
                    }

                    log::debug!("Keep line {}: {}", line_idx + 1, line);
                    new_content.push(line);
                }
                Err(_err) => {
                    todo!()
                }
            }
        }

        if param_section_level.is_none() {
            log::warn!(
                "The parameters section was not found in the markdown file. \
                No parameters table will be rendered"
            );

            return Ok(());
        }

        // resave the markdown file
        let mut new_md_file = File::create(markdown_path)?;

        for line in new_content {
            writeln!(new_md_file, "{}", line)?;
        }

        Ok(())
    }

    fn try_find_param_section(&self, line: &str) -> Option<String> {
        if let Some(captures) = self.param_section_pattern.captures(line) {
            return Some(captures[1].to_string());
        }

        None
    }

    fn render_params(&self) -> Result<String> {
        let mut param_table = MarkdownTable::new(
            vec![
                vec!["test".to_string(), "1".to_string(), "2".to_string()],
            ]
        );

        param_table.with_headings(
            vec![
                Heading::new("Name".to_string(), None),
                Heading::new("Description".to_string(), None),
                Heading::new("Value".to_string(), None),
            ]
        );

        param_table.as_markdown()
    }
}
