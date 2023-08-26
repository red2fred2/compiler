//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

use anyhow::{Ok, Result};

pub mod lexer;

fn main() -> Result<()> {
	let string = include_str!("../test.dm").to_string() + "\n";
	lexer::lex_and_print(string.as_str())?;

	Ok(())
}
