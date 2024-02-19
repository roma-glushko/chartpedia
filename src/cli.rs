/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};
use log::LevelFilter;
use std::path::PathBuf;

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

#[derive(Subcommand)]
pub enum Commands {
    /// Generate chart values documentation
    Gen {
        /// Path to a markdown file
        #[arg(short, long, default_value="README.md")]
        md: PathBuf,
        /// Path to a chart values file
        #[arg(short, long, default_value="values.yaml")]
        values: PathBuf,
    },
    Lint {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}
