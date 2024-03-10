/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::metadata::chart::ChartMetadata;
use anyhow::Result;
use markdown_table::{Heading, MarkdownTable};

pub struct ChartMetadataRenderer {}

impl ChartMetadataRenderer {
    pub fn new() -> ChartMetadataRenderer {
        ChartMetadataRenderer {}
    }

    pub fn render(&self, chart_metadata: &ChartMetadata, section_header: &str) -> Result<String> {
        for section in chart_metadata.sections() {}

        let mut param_table = MarkdownTable::new(vec![vec![
            "`autoscaling.enabled`".to_string(),
            "Enable autoscaling for replicas (recommended if load is variable)".to_string(),
            "`false`".to_string(),
        ]]);

        param_table.with_headings(vec![
            Heading::new("Name".to_string(), None),
            Heading::new("Description".to_string(), None),
            Heading::new("Value".to_string(), None),
        ]);

        param_table.as_markdown()
    }
}
