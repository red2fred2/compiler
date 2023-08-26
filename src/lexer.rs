//! The lexer for the language

use anyhow::Result;
use logos::{Logos, Lexer};
use std::fmt::Debug;

static mut LINE: usize = 1;
static mut LINE_START: usize = 0;

#[allow(unused)]
pub struct Token {
	token: TokenEnum,
	line: usize,
	char: usize,
}

#[derive(Logos, PartialEq)]
#[logos(skip r"[ \t\f]+")]
pub enum TokenEnum {
	#[token("\n", new_line)]
	#[regex(r"//[^\n]*\n", new_line)]
	HELPER,

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

impl Debug for TokenEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HELPER => write!(f, "HELPER"),
            Self::AND => write!(f, "AND"),
            Self::BOOL => write!(f, "BOOL"),
            Self::CLASS => write!(f, "CLASS"),
            Self::ELSE => write!(f, "ELSE"),
            Self::EXIT => write!(f, "EXIT"),
            Self::FALSE => write!(f, "FALSE"),
            Self::GIVE => write!(f, "GIVE"),
            Self::IF => write!(f, "IF"),
            Self::INT => write!(f, "INT"),
            Self::MAGIC => write!(f, "MAGIC"),
            Self::OR => write!(f, "OR"),
            Self::PERFECT => write!(f, "PERFECT"),
            Self::RETURN => write!(f, "RETURN"),
            Self::TAKE => write!(f, "TAKE"),
            Self::TRUE => write!(f, "TRUE"),
            Self::VOID => write!(f, "VOID"),
            Self::WHILE => write!(f, "WHILE"),
            Self::ID(arg0) => write!(f, "ID:{arg0}"),
            Self::INTLITERAL(arg0) => write!(f, "INTLITERAL:{arg0}"),
            Self::STRINGLITERAL(arg0) => write!(f, "STRINGLITERAL:{arg0}"),
            Self::ASSIGN => write!(f, "ASSIGN"),
            Self::COMMA => write!(f, "COMMA"),
            Self::CROSS => write!(f, "CROSS"),
            Self::DASH => write!(f, "DASH"),
            Self::EQUALS => write!(f, "EQUALS"),
            Self::GREATER => write!(f, "GREATER"),
            Self::GREATEREQ => write!(f, "GREATEREQ"),
            Self::LCURLY => write!(f, "LCURLY"),
            Self::LESS => write!(f, "LESS"),
            Self::LESSEQ => write!(f, "LESSEQ"),
            Self::LPAREN => write!(f, "LPAREN"),
            Self::NOT => write!(f, "NOT"),
            Self::NOTEQUALS => write!(f, "NOTEQUALS"),
            Self::OTHER => write!(f, "OTHER"),
            Self::POSTDEC => write!(f, "POSTDEC"),
            Self::POSTINC => write!(f, "POSTINC"),
            Self::RCURLY => write!(f, "RCURLY"),
            Self::RPAREN => write!(f, "RPAREN"),
            Self::SEMICOL => write!(f, "SEMICOL"),
            Self::SLASH => write!(f, "SLASH"),
            Self::STAR => write!(f, "STAR"),
        }
    }
}

fn copy(lexer: &mut Lexer<TokenEnum>) -> Option<String> {
	lexer.slice().parse().ok()
}

fn new_line(lexer: &mut Lexer<TokenEnum>) -> Option<()> {
	unsafe {
		LINE += 1;
		LINE_START = lexer.span().end;
	}

	Some(())
}

fn lex_optional(string: &str, print: bool) -> Result<Vec<Token>> {
	let mut lexer = TokenEnum::lexer(string);
	let mut tokens = Vec::new();

	loop {
		// If this is None we're done reading, break out of the loop
		let Some(token) = lexer.next() else {break};

		// If this isn't Ok no token was matched, print out an error
		let Ok(token) = token else {
			println!("aw crap");
			break
		};

		// Ignore certain helper tokens
		if token == TokenEnum::HELPER {
			continue
		}

		// Figure out line and character position in flex style
		let char;
		let line;
		unsafe {
			char = lexer.span().start + 1 - LINE_START;
			line = LINE
		};

		// Print and push token
		if print {
			println!("{token:?} [{line},{char}]");
		}

		tokens.push(Token {token, line, char});
	}

	Ok(tokens)
}

/// Runs the lexer on a given input string
pub fn lex(string: &str) -> Result<Vec<Token>> {
	lex_optional(string, false)
}

/// Runs the lexer on a given input string and prints out tokens in bison format
pub fn lex_and_print(string: &str) -> Result<Vec<Token>> {
	lex_optional(string, true)
}
