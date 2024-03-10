/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::metadata::value::ValueMetadata;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

// Section defines a param section
#[derive(Debug)]
pub struct SectionMetadata {
    name: RefCell<String>,
    descr: RefCell<Vec<String>>,
    chart_values: RefCell<Vec<Rc<ValueMetadata>>>,
}

impl SectionMetadata {
    pub fn new(name: String) -> SectionMetadata {
        SectionMetadata {
            name: RefCell::new(name),
            descr: RefCell::new(Vec::new()),
            chart_values: RefCell::new(Vec::new()),
        }
    }

    pub fn name(&self) -> Ref<String> {
        self.name.borrow()
    }

    pub fn add_value(&self, chart_value: Rc<ValueMetadata>) {
        self.chart_values.borrow_mut().push(chart_value)
    }

    pub fn values(&self) -> Ref<Vec<Rc<ValueMetadata>>> {
        self.chart_values.borrow()
    }

    pub fn descr(&self) -> String {
        return self.descr.borrow().join("\r\n");
    }

    pub fn add_descr(&self, line: String) {
        self.descr.borrow_mut().push(line);
    }
}
