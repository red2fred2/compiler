//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

use anyhow::{Ok, Result};
use clap::Parser;

pub mod lexer;
pub mod parser;

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

    lexer::lex(contents.as_str(), args.token_file)?;

    Ok(())
}
