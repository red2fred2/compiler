//! # Drewno Mars language compiler
//! The [language specification is here]
//!
//! [language Specification](https://compilers.cool/language/)

pub mod lexer;

fn main() {
	lexer::lex(include_str!("../test.dm"));
}
