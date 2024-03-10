/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use clap::Parser;
use std::process;
mod cli;
mod config;
mod helm;
mod logging;
mod markdown;
mod metadata;

use crate::cli::Commands;
use crate::config::Config;

use crate::helm::parser::ValuesParser;
use crate::markdown::render::MarkdownRenderer;
use crate::metadata::parser::MetadataParser;
use logging::setup_logging;

fn main() {
    let cli = cli::Cli::parse();

    setup_logging(cli.debug);

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
            let metadata_parser = MetadataParser::new(&config);
            let values_parser = ValuesParser::new();
            let renderer = MarkdownRenderer::new(&config);

            let _ = values_parser.parse(values);

            let _ = match metadata_parser.parse(values) {
                Ok(metadata) => metadata,
                Err(err) => {
                    log::error!("Failed to parse values metadata: {}", err);

                    process::exit(1);
                }
            };

            let _ = renderer.render(&markdown);

            ()
        }
        Some(Commands::Check {
            values,
            markdown: _,
            no_missing: _,
        }) => {
            println!("Lint: {}", values.to_string_lossy());

            ()
        }
        None => (),
    }
}
