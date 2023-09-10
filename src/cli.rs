//! This module implements the command-line interface for [`tilted`](crate).
#![cfg(feature = "cli")]

use crate::{Lexer, Parser};
use std::io::Write;

use clap::Parser as ClapParser;

#[derive(Debug, ClapParser)]
#[command(author, version, about, long_about = None)]
pub struct CliParser {
    /// print the AST instead of the result
    #[arg(short = 'p', long)]
    ast: bool,

    /// enable interactive (read-eval-print-loop) mode
    #[arg(short = 'r', long = "repl")]
    interactive: bool,

    /// user input
    input: Option<String>,
}

impl CliParser {
    pub fn parse() -> Self {
        ClapParser::parse()
    }

    pub fn start(&self) -> u8 {
        // Check if the user wants to start the interactive mode.
        if self.interactive {
            self.start_interative()
        }
        // Check if the user provided any input.
        else if let Some(ref input) = self.input {
            let lexer = Lexer::from_source_code(input);
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
        // Error on no input.
        else {
            eprintln!("No input provided");
            1
        }
    }

    fn start_interative(&self) -> u8 {
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
