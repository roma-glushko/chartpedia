use clap::Parser;

mod cli;
mod logging;
use logging::{setup_logging};

fn main() {
    let cli = cli::Cli::parse();

    setup_logging(cli.verbosity);
}
