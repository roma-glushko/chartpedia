/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CommentsConfig {
    pub format: String,
}

impl Default for CommentsConfig {
    fn default() -> Self {
        CommentsConfig {
            format: "##".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TagsConfig {
    pub param: String,
    pub section: String,
    #[serde(rename = "descriptionStart")]
    pub description_start: String,
    #[serde(rename = "descriptionEnd")]
    pub description_end: String,
    pub skip: String,
    pub extra: String,
}

impl Default for TagsConfig {
    fn default() -> Self {
        TagsConfig {
            param: "@param".to_string(),
            section: "@section".to_string(),
            description_start: "@descriptionStart".to_string(),
            description_end: "@descriptionEnd".to_string(),
            skip: "@skip".to_string(),
            extra: "@extra".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ModifiersConfig {
    pub array: String,
    pub object: String,
    pub string: String,
    pub nullable: String,
    pub default: String,
}

impl Default for ModifiersConfig {
    fn default() -> Self {
        ModifiersConfig {
            array: "array".to_string(),
            object: "object".to_string(),
            string: "string".to_string(),
            nullable: "nullable".to_string(),
            default: "default".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct RegexpConfig {
    #[serde(rename = "paramsSectionTitle")]
    pub params_section_title: String,
}

impl Default for RegexpConfig {
    fn default() -> Self {
        RegexpConfig {
            params_section_title: "Parameters".to_string(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    pub comments: CommentsConfig,
    pub tags: TagsConfig,
    pub modifiers: TagsConfig,
    pub regexp: RegexpConfig,
}

impl Config {
    pub fn load(config_path: String) -> Config {
        todo!()
    }
}
