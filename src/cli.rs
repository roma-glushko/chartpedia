/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};
use log::LevelFilter;
use std::path::{PathBuf};

const BANNER: &str = r"
      _                _                  _ _
     | |              | |                | (_)
  ___| |__   __ _ _ __| |_ _ __   ___  __| |_  __ _
 / __| '_ \ / _` | '__| __| '_ \ / _ \/ _` | |/ _` |
| (__| | | | (_| | |  | |_| |_) |  __/ (_| | | (_| |
 \___|_| |_|\__,_|_|   \__| .__/ \___|\__,_|_|\__,_|
                          | |
                          |_|
    🚢 A convenient way to document your Helm charts
";

#[derive(Parser)]
#[command(author, author, version)]
#[command(about = BANNER)]
pub struct Cli {
    /// Verbosity
    #[arg(
    short,
    long,
    default_value_t = LevelFilter::Info,
    value_parser = clap::builder::PossibleValuesParser::new(["TRACE", "DEBUG", "INFO", "WARN", "ERROR"])
    .map(|s| s.parse::<LevelFilter>().unwrap()),
    )]
    pub verbosity: LevelFilter,

    /// Config (if empty, .chartpedia.yaml, .chartpedia.yml, .chartpedia.json are tried to be loaded from the current working directory)
    #[arg(short, long, value_name = "CONFIG_PATH")]
    pub config_path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

fn validate_file_exists(file_path: &str) -> Result<PathBuf, String> {
    let path = PathBuf::from(file_path);

    if path.exists() {
        Ok(path)
    } else {
        Err(format!(
            "The file \"{}\" does not exist.",
            path.to_string_lossy()
        ))
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate chart values documentation
    Gen {
        /// Path to a chart values file
        #[arg(short, long, default_value="values.yaml", value_parser=validate_file_exists)]
        values: PathBuf,
        /// Path to a markdown file
        #[arg(short, long, default_value="README.md", value_parser=validate_file_exists)]
        markdown: PathBuf,
    },
    /// Check generated files based on values.
    Check {
        /// Path to a chart values file
        #[arg(short, long, default_value="values.yaml", value_parser=validate_file_exists)]
        values: PathBuf,
        /// Path to a markdown file
        #[arg(short, long, default_value="README.md", value_parser=validate_file_exists)]
        markdown: PathBuf,
        /// Fail if there are any undocumented chart values
        #[arg(short, long, action, default_value = "true")]
        no_missing: bool,
    },
}
