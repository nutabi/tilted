//! This module implements the command-line interface for [`tilted`](crate).
#![cfg(feature = "cli")]

use std::io::Write;
use crate::{Lexer, Parser};
use argh::FromArgs;

/// A non-Turing-complete interpreted programming 'language' that can do maths
/// (only).
#[derive(Debug, FromArgs)]
pub struct Cli {
    /// print the AST
    #[argh(switch, short = 'p')]
    ast: bool,

    /// enable interactive mode
    #[argh(switch, short = 'i', long = "idle")]
    interactive: bool,

    /// user input
    #[argh(positional)]
    input: Option<String>,
}

impl Cli {
    pub fn start(&self) -> u8 {
        if self.interactive {
            return self.start_interactive();
        }

        let lexer = match self.input {
            Some(ref input) => Lexer::from_source_code(input),
            None => {
                eprintln!("No input provided");
                return 1;
            }
        };

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

    fn start_interactive(&self) -> u8 {
        if let Some(ref input) = self.input {
            eprintln!("Ignoring input: {}", input);
        }

        let mut input = String::new();
        println!("Enter 'quit' to exit");

        loop {
            print!("> ");
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            if input == "quit\n" {
                break 0;
            }

            let lexer = Lexer::from_source_code(&input);
            let mut parser = Parser::from_lexer(lexer);
            let result = parser.parse();

            match result {
                Ok(node) => {
                    if self.ast {
                        println!("{}", node);
                    } else {
                        println!("{}", node.evaluate());
                    }
                }
                Err(e) => {
                    eprintln!("{}", e);
                }
            }
            input.clear();
        }
    }
}
