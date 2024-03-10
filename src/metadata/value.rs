/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use crate::metadata::section::SectionMetadata;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

// Param defines a chart values
#[derive(Debug)]
pub struct ValueMetadata {
    name: RefCell<String>,
    value_type: Option<String>,
    value: Option<String>,
    descr: RefCell<Option<String>>,
    modifiers: Vec<String>,
    section: Option<Rc<SectionMetadata>>,
    should_validate: bool,
    render_in_readme: bool,
    render_in_schema: bool,
}

impl ValueMetadata {
    pub(crate) fn new(
        name: String,
        modifiers: Vec<String>,
        descr: Option<String>,
    ) -> ValueMetadata {
        ValueMetadata {
            name: RefCell::new(name),
            value_type: None,
            value: None,
            modifiers,
            descr: RefCell::new(descr),
            section: None,
            should_validate: true,
            render_in_readme: true,
            render_in_schema: true,
        }
    }

    pub fn name(&self) -> Ref<String> {
        self.name.borrow()
    }

    pub fn descr(&self) -> Ref<Option<String>> {
        self.descr.borrow()
    }

    pub fn set_section(&mut self, section: Rc<SectionMetadata>) {
        self.section = Some(section);
    }

    pub fn skip(&mut self) {
        self.should_validate = false;
        self.render_in_readme = false;
    }

    pub fn has_skipped(&self) -> bool {
        !self.should_validate && !self.render_in_readme
    }

    pub fn set_extra(&mut self) {
        self.should_validate = false;
        self.render_in_readme = true;
    }

    pub fn has_extra(&mut self) -> bool {
        !self.should_validate && self.render_in_readme
    }
}
