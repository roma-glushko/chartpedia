/*
* Copyright 2024, Roma Hlushko
* SPDX-License-Identifier: Apache-2.0
*/
use clap::Parser;

mod cli;
mod logging;
mod metadata;
mod parser;
mod render;
mod config;

use logging::{setup_logging};

fn main() {
    let cli = cli::Cli::parse();

    setup_logging(cli.verbosity);
}
