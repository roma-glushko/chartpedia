/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::metadata::chart::ChartMetadata;
use crate::metadata::section::SectionMetadata;
use crate::metadata::value::ValueMetadata;
use anyhow::Result;
use markdown_table::{Heading, MarkdownTable};
use std::rc::Rc;

pub struct ChartMetadataRenderer {}

impl ChartMetadataRenderer {
    pub fn new() -> ChartMetadataRenderer {
        ChartMetadataRenderer {}
    }

    pub fn render(&self, chart_metadata: &ChartMetadata, section_header: &str) -> Result<String> {
        let mut rendered_metadata = String::with_capacity(100);

        for section in chart_metadata.sections() {
            rendered_metadata.push_str(&self.render_section(section, section_header)?)
        }

        Ok(rendered_metadata)
    }

    fn render_section(&self, section: Rc<SectionMetadata>, section_header: &str) -> Result<String> {
        let mut rendered_section = String::with_capacity(20);

        rendered_section.push_str("\r\n");

        // section header
        rendered_section.push_str(&format!("{} {}\r\n\n", section_header, &section.name()));

        // section description
        let descr = section.descr();

        if !descr.is_empty() {
            rendered_section.push_str(&format!("{} \r\n\n", descr))
        }

        let values = section.values();

        if !values.is_empty() {
            rendered_section.push_str(&self.render_section_values(&values)?)
        }

        Ok(rendered_section)
    }

    fn render_section_values(&self, values: &[Rc<ValueMetadata>]) -> Result<String> {
        let table_rows = values
            .iter()
            .map(|value| {
                vec![
                    format!("`{}`", value.name()),
                    value.descr().clone().unwrap_or_default(),
                    format!("`{}`", "{}"),
                ]
            })
            .collect();

        let mut values_table = MarkdownTable::new(table_rows);

        values_table.with_headings(vec![
            Heading::new("Name".to_string(), None),
            Heading::new("Description".to_string(), None),
            Heading::new("Value".to_string(), None),
        ]);

        values_table.as_markdown()
    }
}
