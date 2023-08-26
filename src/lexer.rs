//! The lexer for the language

use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	#[token("and")]
	AND
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
