//! # Drewno Mars language compiler
//! Here is the [language Specification](https://compilers.cool/language/)
#![feature(test)]

extern crate test;

#[allow(unused)]
use test::Bencher;

use anyhow::{anyhow, Result};
use clap::Parser;

pub mod ast;
pub mod intermediate_code;
pub mod source_position;

/// Drewno Mars language compiler
#[allow(non_snake_case)]
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

    /// Generate 3ac intermediate code
    #[arg(short, long)]
    ac3_IR_generation: Option<String>,
}

fn main() -> Result<()> {
    // Get arguments
    let args = Args::parse();

    // Read file
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";
    source_position::set_document(&contents);

    // Build AST
    let ast = ast::build(&contents, &args);

    let Ok(ast) = ast else {
        return Err(anyhow!("Failed to generate AST"));
    };

    intermediate_code::generate(ast, &args);

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
        ac3_IR_generation: Some("ir.3ac".to_string()),
    };
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path).unwrap() + "\n";
    source_position::set_document(&contents);

    b.iter(|| ast::build(&contents, &args).unwrap())
}
