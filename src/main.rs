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
    let _ = ast::parse(&contents, &args);

    Ok(())
}

#[bench]
fn parser_benchmark(b: &mut Bencher) {
    let args = Args {
        input_file: "test.dm".to_string(),
        parse: false,
        unparse: None,
        named_unparse: None,
    };
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path).unwrap() + "\n";

    b.iter(|| ast::parse(&contents, &args).unwrap())
}
