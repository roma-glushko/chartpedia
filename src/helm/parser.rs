/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use crate::helm::values::ChartValues;
use anyhow::Result;
use serde_yaml::{Mapping, Value};
use std::fmt::Debug;
use std::path::Path;
use std::{fmt, fs};
use thiserror::Error;

#[derive(Error, Debug)]
pub struct ValuesParseError {
    message: String,
}

// Implement Display for your custom error
impl fmt::Display for ValuesParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ValuesParseError {
    pub fn new(msg: String) -> Self {
        ValuesParseError { message: msg }
    }
}

/// ValuesParser parses values.yaml file
pub struct ChartValuesParser {
    delimiter: String,
}

impl ChartValuesParser {
    pub fn new() -> ChartValuesParser {
        ChartValuesParser {
            delimiter: ".".to_string(),
        }
    }

    pub fn parse<P: AsRef<Path> + Debug + Clone>(&self, values_file: P) -> Result<ChartValues> {
        let content = fs::read_to_string(values_file.clone())?;
        let values_map: Value = serde_yaml::from_str(&content)?;

        let values = ChartValues::new();
        let curr_path = "";

        log::debug!("Processing Helm values.yaml: {:?}", values_file.clone());

        if let Value::Mapping(values_map) = values_map {
            self.process_map(curr_path, &values, &values_map)?;

            return Ok(values);
        }

        Err(ValuesParseError::new("Helm values.yaml should be a map".to_string()).into())
    }

    fn process_map(
        &self,
        parent_path: &str,
        values: &ChartValues,
        values_map: &Mapping,
    ) -> Result<()> {
        let curr_path = if parent_path.is_empty() {
            parent_path.to_string()
        } else {
            format!("{}{}", parent_path, self.delimiter)
        };

        for (key, value) in values_map {
            let path = format!(
                "{}{}",
                curr_path,
                key.as_str()
                    .ok_or(ValuesParseError::new(
                        "Failed to process values.yaml key".to_string()
                    ))
                    .unwrap(),
            );

            // If the value is also a mapping, you can recursively enumerate it
            if let Value::Mapping(ref nested_map) = value {
                self.process_map(&path, values, nested_map)?;

                continue;
            }

            log::debug!("Processing value {}: {:?}", path, value);

            values.insert(path, value.clone());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

}
