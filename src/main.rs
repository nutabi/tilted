#![cfg(feature = "cli")]

use std::process::exit;

use tilted::{self, Cli};

fn main() {
    let cli: Cli = argh::from_env();
    exit(cli.start().into())
}
