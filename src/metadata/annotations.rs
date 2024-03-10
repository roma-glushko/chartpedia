/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use std::rc::Rc;
use crate::metadata::section::Section;
use crate::metadata::value::ValueMetadata;


// Metadata defines the general metadata defined in a chart values file
#[derive(Debug)]
pub struct Metadata {
    sections: Vec<Rc<Section>>,
    values: Vec<Rc<ValueMetadata>>,
}

impl Metadata {
    pub(crate) fn new() -> Metadata {
        Metadata {
            sections: Vec::new(),
            values: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: Rc<Section>) {
        self.sections.push(section)
    }

    pub fn add_value(&mut self, chat_value: Rc<ValueMetadata>) {
        self.values.push(chat_value)
    }
}
