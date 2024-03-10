/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use crate::metadata::section::SectionMetadata;
use crate::metadata::value::ValueMetadata;
use std::rc::Rc;

// Metadata defines the general metadata defined in a chart values file
#[derive(Debug)]
pub struct ChartMetadata {
    sections: Vec<Rc<SectionMetadata>>,
    values: Vec<Rc<ValueMetadata>>,
}

impl ChartMetadata {
    pub(crate) fn new() -> ChartMetadata {
        ChartMetadata {
            sections: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: Rc<SectionMetadata>) {
        self.sections.push(section)
    }

    pub fn add_value(&mut self, chat_value: Rc<ValueMetadata>) {
        self.values.push(chat_value)
    }
}
