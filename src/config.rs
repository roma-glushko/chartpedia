/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CommentsConfig {
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
pub struct TagsConfig {
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
pub struct ModifiersConfig {
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
pub struct RegexpConfig {
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

#[derive(Error, Debug)]
pub struct ConfigError {
    message: String,
}

// Implement Display for your custom error
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl ConfigError {
    pub fn new(msg: String) -> Self {
        ConfigError { message: msg }
    }
}

impl Config {
    pub fn load(config_path: Option<PathBuf>) -> Result<Config> {
        match config_path {
            Some(path) => Config::load_config(path),
            None => {
                log::debug!("Config path is not specified");
                Config::load_default_paths()
            }
        }
    }

    fn load_default_paths() -> Result<Config> {
        let default_paths = vec![
            Path::new(".chartpedia.yaml"),
            Path::new(".chartpedia.yml"),
            Path::new(".chartpedia.json"),
        ];

        log::debug!("Trying to find under default path");

        for path in default_paths {
            log::debug!("- Trying to load {}", &path.to_string_lossy());

            if !path.exists() {
                continue;
            }

            return Config::load_config(path);
        }

        log::debug!("The default config is loaded");
        Ok(Config::default())
    }

    fn load_config<P: AsRef<Path>>(path: P) -> Result<Config> {
        let mut file = File::open(&path)?;
        let mut config_content = String::new();

        file.read_to_string(&mut config_content)?;

        match serde_yaml::from_str(&config_content) {
            Ok(config) => return Ok(config),
            Err(err) => log::debug!(
                "Failed to load {} as YAML {}",
                path.as_ref().to_string_lossy(),
                err
            ),
        }

        match serde_json::from_str(&config_content) {
            Ok(config) => return Ok(config),
            Err(err) => log::debug!(
                "Failed to load {} as JSON {}",
                path.as_ref().to_string_lossy(),
                err
            ),
        }

        Err(ConfigError::new("config file must be in YAML or JSON format".into()).into())
    }
}
