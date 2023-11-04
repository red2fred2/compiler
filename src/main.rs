//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)
#![feature(test)]

extern crate test;

#[allow(unused)]
use test::Bencher;

use anyhow::Result;
use clap::Parser;

pub mod ast;
pub mod source_position;

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
    let args = Args::parse();
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";
    source_position::set_document(&contents);
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
