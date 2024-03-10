/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/

use std::cell::RefCell;
use std::collections::HashMap;
use serde_yaml::Value;

/// HelmValues holds flatten path to a helm value (e.g. dot-separated path like image.tag) and it's value
pub struct HelmValues {
    values: RefCell<HashMap<String, Value>>
}

impl HelmValues {
    pub fn new() -> HelmValues {
        HelmValues {
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, value_path: String, value: Value) {
        self.values.borrow_mut().insert(value_path, value);
    }

}
