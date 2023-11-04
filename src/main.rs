//! # Drewno Mars language compiler
//! Here is the [language specification].
//!
//! [language Specification](https://compilers.cool/language/)
#![feature(test)]

extern crate test;

#[allow(unused)]
use test::Bencher;

use anyhow::Result;
use clap::Parser;

mod ast;
mod source_position;

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

    /// Do type checking without output
    #[arg(short, long)]
    check_types: bool,
}

fn main() -> Result<()> {
    // Get arguments
    let args = Args::parse();

    // Read file
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";
    source_position::set_document(&contents);

    // Build AST
    let _ = ast::build(&contents, &args);

    Ok(())
}

#[bench]
fn parser_benchmark(b: &mut Bencher) {
    let args = Args {
        input_file: "test.dm".to_string(),
        parse: true,
        unparse: None,
        named_unparse: None,
        check_types: true,
    };
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path).unwrap() + "\n";
    source_position::set_document(&contents);

    b.iter(|| ast::build(&contents, &args).unwrap())
}
