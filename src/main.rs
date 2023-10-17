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
    /// File to compile
    input_file: String,

    /// Parse
    #[arg(short, long)]
    parse: bool,

    /// Unparse
    #[arg(short, long)]
    unparse: Option<String>,

    /// Named unparse
    #[arg(short, long)]
    named_unparse: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";
    ast::parse(&contents, &args)?;

    Ok(())
}
