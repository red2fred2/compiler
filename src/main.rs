//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

pub mod lexer;

fn main() {
	let string = include_str!("../test.dm").to_string() + "\n";
	lexer::lex(string.as_str());
}
