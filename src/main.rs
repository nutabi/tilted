#![cfg(feature = "cli")]

use std::process::exit;

use tilted::CliParser;

fn main() {
    let cli = CliParser::parse();
    exit(cli.start().into())
}
