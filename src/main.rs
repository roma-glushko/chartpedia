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

    let _config = Config::load(cli.config_path);

    log::debug!("Config is loaded");

    match &cli.command {
        Some(Commands::Gen { md, values }) => {
            println!("Gen: {} {}", md.to_string_lossy(), values.to_string_lossy());

            ()
        }
        Some(Commands::Lint { list: _ }) => {
            println!("Lint");

            ()
        }
        None => (),
    }
}
