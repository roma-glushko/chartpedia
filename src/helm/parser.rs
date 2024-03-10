/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::metadata;
use anyhow::Result;
use serde_yaml::Value;
use std::fmt::Display;
use std::fs;
use std::path::Path;
use std::str::FromStr;

use crate::metadata::{Param, Section};
use metadata::Metadata;

/// ValuesParser parses values.yaml file
pub struct ValuesParser {}

impl ValuesParser {
    pub fn new() -> ValuesParser {
        ValuesParser {}
    }

    pub fn parse<P: AsRef<Path>>(&self, values_file: P) -> Result<()> {
        let content = fs::read_to_string(values_file)?;
        let values: Value = serde_yaml::from_str(&content)?;

        if let Value::Mapping(values_map) = values {
            for (key, value) in values_map {
                println!("Key: {:?}, Value: {:?}", key, value);

                // If the value is also a mapping, you can recursively enumerate it
                if let Value::Mapping(sub_map) = value {
                    enumerate_mapping(sub_map);
                }
            }

            Ok(())
        }

        Err()
    }
}
