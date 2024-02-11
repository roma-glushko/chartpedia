/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use clap::Parser;

mod cli;
mod config;
mod logging;
mod metadata;
mod parser;
mod render;

use crate::cli::Commands;
use crate::config::Config;
use logging::setup_logging;

fn main() {
    let cli = cli::Cli::parse();

    setup_logging(cli.verbosity);

    let config = Config::load(config_path);

    // .chartpedia.yaml

    match &cli.command {
        Some(Commands::Gen) => {
            println!("Gen");

            ()
        },
        Some(Commands::Lint) => {
            println!("Lint");

            ()
        }
        None => (),
        _ => todo!(),
    }
}
