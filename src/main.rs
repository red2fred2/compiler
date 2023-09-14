//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

use anyhow::{Ok, Result};
use clap::Parser;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub grammar); // synthesized by LALRPOP

pub mod lexer;

/// Drewno Mars language compiler
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// File to be compiled
    input_file: String,

    /// File to output token stream to
    #[arg(short, long)]
    token_file: Option<String>,

    // Parse flag
    #[arg(short, long)]
    parse: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";

    // Lexer
    if args.token_file.is_some() {
        lexer::lex(contents.as_str(), args.token_file)?;
    }

    // Parser
    if args.parse {
        let result = grammar::ProgramParser::new().parse(&contents);

        if !result.is_ok() {
            eprintln!("syntax error\nParse failed");
        }
    }

    Ok(())
}
