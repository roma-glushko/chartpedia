/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use clap::builder::TypedValueParser;
use clap::{Parser, Subcommand};
use log::LevelFilter;

const BANNER: &'static str = r"
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

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {

}
