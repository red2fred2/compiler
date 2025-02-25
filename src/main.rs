//! # Drewno Mars language compiler
//! Here is the [language Specification](https://compilers.cool/language/)
#![feature(test)]

extern crate test;

use std::{fs::File, io::Write};

#[allow(unused)]
use test::Bencher;

use anyhow::Result;
use clap::Parser;
use x64::X64Target;

pub mod ast;
pub mod source_position;
pub mod three_ac;
pub mod x64;

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

    /// Generate 3AC intermediate code
    #[arg(short, long)]
    ac3_IR_generation: Option<String>,

    /// Generate x64 assembly
    #[arg(short, long)]
    output_assembly: Option<String>,
}

fn main() -> Result<()> {
    // Get arguments
    let args = Args::parse();

    // Read file
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path)? + "\n";
    source_position::set_document(&contents);

    // Build AST
    let ast = ast::build(&contents, &args)?;

    // Output IR code
    if let Some(_output_path) = &args.ac3_IR_generation {
        let ir_code = three_ac::generate(&ast);

        let mut string = String::new();
        for line in ir_code {
            string = format!("{string}{line}");
        }
        let mut file = File::create(_output_path)?;
        file.write_all(string.as_bytes())?;
    }

    // Output assembly code
    if let Some(_output_path) = &args.output_assembly {
        let ir_code = three_ac::generate(&ast);

        let mut string = String::new();
        for line in ir_code {
            let asm = line.compile_x64();
            string = format!("{string}{asm}");
        }
        let mut file = File::create(_output_path)?;
        file.write_all(string.as_bytes())?;
    }

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
        ac3_IR_generation: None,
        output_assembly: Some("output.s".to_string()),
    };
    let path = &args.input_file;
    let contents = std::fs::read_to_string(path).unwrap() + "\n";
    source_position::set_document(&contents);

    b.iter(|| ast::build(&contents, &args).unwrap())
}
