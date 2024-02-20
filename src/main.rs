/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use std::process;
use clap::Parser;

mod cli;
mod config;
mod logging;
mod metadata;
mod parser;
mod render;

use crate::cli::Commands;
use crate::config::Config;
use crate::parser::MetadataParser;
use logging::setup_logging;

fn main() {
    let cli = cli::Cli::parse();

    setup_logging(cli.verbosity);

    let config = match Config::load(cli.config_path) {
        Ok(config) => config,
        Err(e) => {
            log::error!("Failed to load config file: {}", e);

            process::exit(1);
        }
    };

    log::debug!("Config is loaded");

    match &cli.command {
        Some(Commands::Gen { markdown, values }) => {
            let parser = MetadataParser::new(config);

            let metadata = match parser.parse(values) {
                Ok(metadata) => metadata,
                Err(err) => {
                    log::error!("Failed to parse values metadata: {}", err);

                    process::exit(1);
                }
            };

            ()
        }
        Some(Commands::Check { values , markdown, no_missing}) => {
            println!("Lint: {}", values.to_string_lossy());

            ()
        }
        None => (),
    }
}
