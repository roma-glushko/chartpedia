/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::config;
use crate::metadata;
use regex::Regex;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::str::FromStr;

use crate::metadata::{Param, Section};
use config::Config;
use metadata::Metadata;

struct MetadataParser {
    config: Config,
    param_regex: Regex,
    section_regex: Regex,
    descr_start_regex: Regex,
    descr_content_regex: Regex,
    descr_end_regex: Regex,
    skip_regex: Regex,
    extra_regex: Regex,
}

impl MetadataParser {
    fn new(config: Config) -> MetadataParser {
        let param_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*{}\\s*([^\\s]+)\\s*(\\[.*?\\])?\\s*(.*)$",
            regex::escape(&*config.comments.format),
            regex::escape(&*config.tags.param)
        ))
        .unwrap();
        let section_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*{}\\s*(.*)$",
            regex::escape(&*config.comments.format),
            regex::escape(&*config.tags.section)
        ))
        .unwrap();
        let descr_start_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*{}\\s*(.*)",
            regex::escape(&*config.comments.format),
            regex::escape(&*config.tags.description_start)
        ))
        .unwrap();
        let descr_content_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*(.*)",
            regex::escape(&*config.comments.format)
        ))
        .unwrap();
        let descr_end_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*{}\\s*(.*)",
            regex::escape(&*config.comments.format),
            regex::escape(&*config.tags.description_end)
        ))
        .unwrap();
        let skip_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*{}\\s*([^\\s]+)\\s*(.*)$",
            regex::escape(&*config.comments.format),
            regex::escape(&*config.tags.skip)
        ))
        .unwrap();
        let extra_regex = Regex::new(&*format!(
            r"^\\s*{}\\s*{}\\s*([^\\s]+)\\s*(\\[.*?\\])?\\s*(.*)$",
            regex::escape(&*config.comments.format),
            regex::escape(&*config.tags.extra)
        ))
        .unwrap();

        MetadataParser {
            config,
            param_regex,
            section_regex,
            descr_start_regex,
            descr_content_regex,
            descr_end_regex,
            skip_regex,
            extra_regex,
        }
    }

    fn parse(self, values_file_path: String) -> Result<Metadata, E> {
        let values_file = File::open(values_file_path)?;
        let reader = io::BufReader::new(values_file);

        let metadata = Metadata::new();
        let mut curr_section: Option<&Section> = None;
        let mut descr_parsing = false;

        for line_res in reader.lines() {
            match line_res {
                Ok(line) => {
                    if let Some(param) = self.try_parse_param(&line) {
                        metadata.add_param(&param);

                        if let Some(section) = &curr_section {
                            section.add_param(&param)
                        }
                    }

                    if let Some(section) = self.try_parse_section(&line) {
                        metadata.add_section(&section);

                        curr_section = Some(&section)
                    }

                    if let Some(has_end) = self.has_descr_end(&line) {
                        if has_end && curr_section.is_some() && descr_parsing {
                            descr_parsing = false
                        }
                    }

                    if let Some(descr_line) = self.try_parse_descr_content(&line) {
                        match curr_section {
                            Some(section) => section.add_descr(descr_line),
                            None => todo!(),
                        }
                    }

                    if let Some(descr_start) = self.try_parse_descr_start(&line) {
                        if curr_section.is_some() && !descr_parsing {
                            descr_parsing = true;

                            if !descr_start.is_empty() {
                                if let Some(section) = curr_section {
                                    section.add_descr(descr_start);
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    todo!()
                }
            }
        }

        Ok(metadata)
    }

    fn try_parse_param(self, line: &String) -> Option<Param> {
        if let Some(captures) = self.param_regex.captures(line.as_str()) {
            let name = String::from_str(&captures[1]).unwrap();

            let modifiers = match captures[2].copy() {
                capt if !capt.is_empty() => capt
                    .trim_matches(|c| c == '[' || c == ']')
                    .split(",")
                    .map(|m| m.trim()),
                _ => vec![],
            };

            let descr = captures[3].copy();

            return Some(Param::new(name, modifiers, descr));
        }

        if let Some(captures) = self.skip_regex.captures(line.as_str()) {
            let name = String::from_str(&captures[1]).unwrap();
            let param = Param::new(name, vec![], None);

            param.skip();

            return Some(param);
        }

        if let Some(captures) = self.extra_regex.captures(line.as_str()) {
            let name = String::from_str(&captures[1]).unwrap();
            let descr = String::from_str(&captures[3]).unwrap();

            let param = Param::new(name, vec![], Some(descr));
            param.set_extra();

            return Some(param);
        }

        None
    }

    fn try_parse_section(self, line: &String) -> Option<Section> {
        if let Some(captures) = self.section_regex.captures(line.as_str()) {
            return Some(Section::new(captures[1].copy()));
        }

        None
    }

    fn has_descr_end(self, line: &String) -> Option<bool> {
        if let Some(_) = self.descr_end_regex.captures(line.as_str()) {
            return Some(true);
        }

        None
    }

    fn try_parse_descr_content(self, line: &String) -> Option<String> {
        if let Some(captures) = self.descr_content_regex.captures(line.as_str()) {
            return Some(captures[1].copy());
        }

        None
    }

    fn try_parse_descr_start(self, line: &String) -> Option<String> {
        if let Some(captures) = self.descr_start_regex.captures(line.as_str()) {
            return Some(captures[1].copy());
        }

        None
    }
}
