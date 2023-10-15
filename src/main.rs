//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

use anyhow::Result;
use clap::Parser;

pub mod ast;

/// Drewno Mars language compiler
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// File to be compiled
    input_file: String,

    /// Parse flag
    #[arg(short, long)]
    parse: bool,

    /// Unparse flag
    #[arg(short, long)]
    unparse: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";
    ast::parse(&contents, &args)?;

    Ok(())
}
