//! The lexer for the language

use logos::{Logos, Lexer};

static mut LINE: usize = 1;
static mut LINE_START: usize = 0;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
	#[token("\n", new_line)]
	NEWLINE,

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
	#[token("false")]
	#[token("too hot")]
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

	// Integer literals
	#[regex(r"\d+", copy)]
	INTLITERAL(String),

	// String literals
	#[regex("\"[^\"]*\"", copy)]
	STRINGLITERAL(String),

	// Operators
	#[token("=")]
	ASSIGN,
	#[token(",")]
	COMMA,
	#[token("+")]
	CROSS,
	#[token("-")]
	DASH,
	#[token("==")]
	EQUALS,
	#[token(">")]
	GREATER,
	#[token(">=")]
	GREATEREQ,
	#[token("{")]
	LCURLY,
	#[token("<")]
	LESS,
	#[token("<=")]
	LESSEQ,
	#[token("(")]
	LPAREN,
	#[token("!")]
	NOT,
	#[token("!=")]
	NOTEQUALS,
	#[token(":")]
	OTHER,
	#[token("--")]
	POSTDEC,
	#[token("++")]
	POSTINC,
	#[token("}")]
	RCURLY,
	#[token(")")]
	RPAREN,
	#[token(";")]
	SEMICOL,
	#[token("/")]
	SLASH,
	#[token("*")]
	STAR,
}

// impl Debug for Token {}

fn copy(lexer: &mut Lexer<Token>) -> Option<String> {
	lexer.slice().parse().ok()
}

fn new_line(lexer: &mut Lexer<Token>) -> Option<()> {
	unsafe {
		LINE += 1;
		LINE_START = lexer.span().start + 1;
	}

	Some(())
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

		// Ignore certain helper tokens
		if token == Token::NEWLINE {
			continue
		}

		// Figure out line and character position in flex style
		let char;
		let line;
		unsafe {
			char = lexer.span().start + 1 - LINE_START;
			line = LINE
		};

		println!("{token:?} [{line},{char}]");
	}
}
