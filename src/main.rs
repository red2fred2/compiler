//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

use anyhow::Result;
use ast::TreeNode;
use clap::Parser;

pub mod ast;
pub mod parser;
mod parser_test;

/// Drewno Mars language compiler
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
    let path = args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";

    if args.parse || args.unparse.is_some() {
        let mut ast = parser::parse(&contents, args.unparse)?;
        let children = ast[0].get_children();

        println!("{children:#?}")
    }

    Ok(())
}
