//! The lexer for the language

use logos::{Logos, Lexer};

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	//Keywords
	#[token("and")]
	AND,
	#[token("bool")]
	BOOL,
	#[token("class")]
	CLASS,
	#[token("else")]
	ELSE,
	#[token("today I don't feel like doing any work")]
	EXIT,
	#[regex(r"false|too hot")]
	FALSE,
	#[token("give")]
	GIVE,
	#[token("if")]
	IF,
	#[token("int")]
	INT,
	#[token("24Kmagic")]
	MAGIC,
	#[token("or")]
	OR,
	#[token("perfect")]
	PERFECT,
	#[token("return")]
	RETURN,
	#[token("take")]
	TAKE,
	#[token("true")]
	TRUE,
	#[token("void")]
	VOID,
	#[token("while")]
	WHILE,

	// Identifiers
	#[regex(r"[\p{L}_][\p{L}_\d]*", copy)]
	ID(String),
}

fn copy(lexer: &mut Lexer<Token>) -> Option<String> {
	lexer.slice().parse().ok()
}

pub fn lex(string: &str) {
	let mut lexer = Token::lexer(string);

	loop {
		// If this is None we're done reading, break out of the loop
		let Some(token) = lexer.next() else {break};

		// If this isn't Ok no token was matched, print out an error
		let Ok(token) = token else {
			println!("aw crap");
			break
		};


		// let val = lexer.slice();
		println!("{token:?}");
	}
}
