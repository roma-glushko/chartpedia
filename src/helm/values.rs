/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use serde_yaml::Value;
use std::cell::RefCell;
use std::collections::HashMap;

/// HelmValues holds flatten path to a helm value (e.g. dot-separated path like image.tag) and it's value
pub struct ChartValues {
    values: RefCell<HashMap<String, Value>>,
}

impl ChartValues {
    pub fn new() -> ChartValues {
        ChartValues {
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, value_path: String, value: Value) {
        self.values.borrow_mut().insert(value_path, value);
    }
}
