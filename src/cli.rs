//! This module implements the command-line interface for [`tilted`](crate).
#![cfg(feature = "cli")]

use crate::{Lexer, Parser};
use argh::FromArgs;

/// A non-Turing-complete interpreted programming 'language' that can do maths
/// (only).
#[derive(Debug, FromArgs)]
pub struct Cli {
    /// print the AST
    #[argh(switch, short = 'p')]
    ast: bool,

    /// user input
    #[argh(positional)]
    input: String,
}

impl Cli {
    pub fn start(&self) -> u8 {
        let lexer = Lexer::from_source_code(&self.input);
        let mut parser = Parser::from_lexer(lexer);
        let result = parser.parse();

        match result {
            Ok(node) => {
                if self.ast {
                    println!("{}", node);
                } else {
                println!("{}", node.evaluate());
                }
                0
            }
            Err(e) => {
                eprintln!("{}", e);
                1
            }
        }
    }
}
