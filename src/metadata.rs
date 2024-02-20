/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use std::cell::RefCell;
use std::rc::Rc;

// Param defines a chart values
#[derive(Debug)]
pub struct Param {
    name: String,
    param_type: Option<String>,
    value: Option<String>,
    descr: Option<String>,
    modifiers: Vec<String>,
    section: Option<Rc<Section>>,
    should_validate: bool,
    render_in_readme: bool,
    render_in_schema: bool,
}

impl Param {
    pub(crate) fn new(name: String, modifiers: Vec<String>, descr: Option<String>) -> Param {
        Param {
            name,
            param_type: None,
            value: None,
            modifiers,
            descr,
            section: None,
            should_validate: true,
            render_in_readme: true,
            render_in_schema: true,
        }
    }

    pub fn set_section(&mut self, section: Rc<Section>) {
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

// Section defines a param section
#[derive(Debug)]
pub struct Section {
    name: String,
    descr: RefCell<Vec<String>>,
    params: RefCell<Vec<Rc<Param>>>,
}

impl Section {
    pub fn new(name: String) -> Section {
        Section {
            name,
            descr: RefCell::new(Vec::new()),
            params: RefCell::new(Vec::new()),
        }
    }

    pub fn add_param(&self, param: Rc<Param>) {
        self.params.borrow_mut().push(param)
    }

    pub fn add_descr(&self, line: String) {
        self.descr.borrow_mut().push(line);
    }
}

// Metadata defines the general metadata defined in a chart values file
#[derive(Debug)]
pub struct Metadata {
    sections: Vec<Rc<Section>>,
    params: Vec<Rc<Param>>,
}

impl Metadata {
    pub(crate) fn new() -> Metadata {
        Metadata {
            sections: Vec::new(),
            params: Vec::new(),
        }
    }

    pub fn add_section(&mut self, section: Rc<Section>) {
        self.sections.push(section)
    }

    pub fn add_param(&mut self, param: Rc<Param>) {
        self.params.push(param)
    }
}
