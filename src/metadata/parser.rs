/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::config;
use anyhow::Result;
use regex::Regex;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use std::rc::Rc;
use std::str::FromStr;
use std::{fmt, io};

use crate::metadata::chart::ChartMetadata;
use crate::metadata::section::SectionMetadata;
use crate::metadata::value::ValueMetadata;
use config::Config;

#[derive(Debug)]
struct ParsingError {
    message: String,
}

impl Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// Implement the std::error::Error trait for your custom error type.
impl Error for ParsingError {}

/// MetadataParser parses metadata left in values.yaml file
#[derive(Debug)]
pub struct ChartMetadataParser {
    param_regex: Regex,
    section_regex: Regex,
    descr_start_regex: Regex,
    descr_content_regex: Regex,
    descr_end_regex: Regex,
    skip_regex: Regex,
    extra_regex: Regex,
}

impl ChartMetadataParser {
    pub fn new(config: &Config) -> ChartMetadataParser {
        let param_regex = Regex::new(&format!(
            r"^\s*{}\s*{}\s*([^\s]+)\s*(\[.*?\])?\s*(.*)$",
            regex::escape(&config.comments.format),
            regex::escape(&config.tags.param)
        ))
        .unwrap();
        let section_regex = Regex::new(&format!(
            r"^\s*{}\s*{}\s*(.*)$",
            regex::escape(&config.comments.format),
            regex::escape(&config.tags.section)
        ))
        .unwrap();
        let descr_start_regex = Regex::new(&format!(
            r"^\s*{}\s*{}\s*(.*)",
            regex::escape(&config.comments.format),
            regex::escape(&config.tags.description_start)
        ))
        .unwrap();
        let descr_content_regex = Regex::new(&format!(
            r"^\s*{}\s*(.*)",
            regex::escape(&config.comments.format)
        ))
        .unwrap();
        let descr_end_regex = Regex::new(&format!(
            r"^\s*{}\s*{}\s*(.*)",
            regex::escape(&config.comments.format),
            regex::escape(&config.tags.description_end)
        ))
        .unwrap();
        let skip_regex = Regex::new(&format!(
            r"^\s*{}\s*{}\s*([^\s]+)\s*(.*)$",
            regex::escape(&config.comments.format),
            regex::escape(&config.tags.skip)
        ))
        .unwrap();
        let extra_regex = Regex::new(&format!(
            r"^\s*{}\s*{}\s*([^\s]+)\s*(\[.*?\])?\s*(.*)$",
            regex::escape(&config.comments.format),
            regex::escape(&config.tags.extra)
        ))
        .unwrap();

        ChartMetadataParser {
            param_regex,
            section_regex,
            descr_start_regex,
            descr_content_regex,
            descr_end_regex,
            skip_regex,
            extra_regex,
        }
    }

    pub fn parse<P: AsRef<Path>>(&self, values_file: P) -> Result<ChartMetadata> {
        let values_file = File::open(values_file)?;
        let reader = io::BufReader::new(&values_file);

        let mut metadata = ChartMetadata::new();
        let mut curr_section: Option<Rc<SectionMetadata>> = None;
        let mut descr_parsing = false;

        log::debug!("Parsing chart metadata in values.yaml: {:?}", values_file);

        for (line_idx, line_res) in reader.lines().enumerate() {
            match line_res {
                Ok(line) => {
                    if let Some(value) = self.try_parse_value_metadata(&line) {
                        let value_rc = Rc::new(value);

                        log::debug!(
                            "Found chart value metadata at line {}: {:?}",
                            line_idx + 1,
                            value_rc,
                        );

                        metadata.add_value(Rc::clone(&value_rc));

                        if let Some(section) = &curr_section {
                            section.add_value(Rc::clone(&value_rc))
                        }
                    }

                    if let Some(section) = self.try_parse_section_metadata(&line) {
                        let section_rc = Rc::new(section);
                        metadata.add_section(Rc::clone(&section_rc));

                        log::debug!(
                            "Found chart value section metadata at line {}: {:?}",
                            line_idx + 1,
                            section_rc,
                        );

                        curr_section = Some(Rc::clone(&section_rc))
                    }

                    if let Some(has_end) = self.has_descr_end(&line) {
                        if has_end && curr_section.is_some() && descr_parsing {
                            descr_parsing = false;
                        }
                    }

                    if curr_section.is_some() && descr_parsing {
                        if let Some(descr_line) = self.try_parse_descr_content(&line) {
                            match &curr_section {
                                Some(section) => section.add_descr(descr_line),
                                None => todo!(),
                            }
                        }
                    }

                    if curr_section.is_some() && !descr_parsing {
                        if let Some(descr_start) = self.try_parse_descr_start(&line) {
                            descr_parsing = true;

                            if !descr_start.is_empty() {
                                if let Some(section) = &curr_section {
                                    section.add_descr(descr_start);
                                }
                            }
                        }
                    }
                }
                Err(_err) => {
                    todo!()
                }
            }
        }

        Ok(metadata)
    }

    fn try_parse_value_metadata(&self, line: &str) -> Option<ValueMetadata> {
        if let Some(captures) = self.param_regex.captures(line) {
            let name = captures[1].to_string();

            let modifiers = match captures.get(2) {
                Some(mod_str) => match mod_str {
                    mod_str if !mod_str.is_empty() => mod_str
                        .as_str()
                        .trim_matches(|c| c == '[' || c == ']')
                        .split(',')
                        .map(|m| m.trim().to_string())
                        .collect(),
                    _ => vec![],
                },
                None => vec![],
            };

            let descr = captures[3].to_string();

            return Some(ValueMetadata::new(name, modifiers, Some(descr)));
        }

        if let Some(captures) = self.skip_regex.captures(line) {
            let name = captures[1].to_string();
            let mut param = ValueMetadata::new(name, vec![], None);

            param.skip();

            return Some(param);
        }

        if let Some(captures) = self.extra_regex.captures(line) {
            let name = String::from_str(&captures[1]).unwrap();
            let descr = String::from_str(&captures[3]).unwrap();

            let mut param = ValueMetadata::new(name, vec![], Some(descr));
            param.set_extra();

            return Some(param);
        }

        None
    }

    fn try_parse_section_metadata(&self, line: &str) -> Option<SectionMetadata> {
        if let Some(captures) = self.section_regex.captures(line) {
            return Some(SectionMetadata::new(captures[1].to_string()));
        }

        None
    }

    fn has_descr_end(&self, line: &str) -> Option<bool> {
        if self.descr_end_regex.captures(line).is_some() {
            return Some(true);
        }

        None
    }

    fn try_parse_descr_content(&self, line: &str) -> Option<String> {
        if let Some(captures) = self.descr_content_regex.captures(line) {
            return Some(captures[1].to_string());
        }

        None
    }

    fn try_parse_descr_start(&self, line: &str) -> Option<String> {
        if let Some(captures) = self.descr_start_regex.captures(line) {
            return Some(captures[1].to_string());
        }

        None
    }
}
