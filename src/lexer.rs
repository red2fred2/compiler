//! The lexer for the language

use anyhow::Result;
use logos::{Logos, Lexer};
use std::{fmt::Debug, fs::File, io::Write};

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
	#[regex(r"(//[^\n]*\n|)|\n", new_line)] HELPER,

	#[token("and")] AND,
	#[token("=")] ASSIGN,
	#[token("bool")] BOOL,
	#[token(",")] COMMA,
	#[token("class")] CLASS,
	#[token(":")] COLON,
	#[token("+")] CROSS,
	#[token("-")] DASH,
	#[token("else")] ELSE,
	#[token("==")] EQUALS,
	#[token("today I don't feel like doing any work")] EXIT,
	#[token("false")] #[token("too hot")] FALSE,
	#[token("give")] GIVE,
	#[token(">")] GREATER,
	#[token(">=")] GREATEREQ,
	#[regex(r"[\p{L}_][\p{L}_\d]*", copy)] ID(String),
	#[token("if")] IF,
	#[token("int")] INT,
	#[regex(r"\d+", int)] INTLITERAL(String),
	#[token("{")] LCURLY,
	#[token("<")] LESS,
	#[token("<=")] LESSEQ,
	#[token("(")] LPAREN,
	#[token("24Kmagic")] MAGIC,
	#[token("!")] NOT,
	#[token("!=")] NOTEQUALS,
	#[token("or")] OR,
	#[token("perfect")] PERFECT,
	#[token("--")] POSTDEC,
	#[token("++")] POSTINC,
	#[token("}")] RCURLY,
	#[token("return")] RETURN,
	#[token(")")] RPAREN,
	#[token(";")] SEMICOL,
	#[token("/")] SLASH,
	#[token("*")] STAR,
	#[regex("\"[^\"]*\"", copy)] STRINGLITERAL(String),
	#[token("take")] TAKE,
	#[token("true")] TRUE,
	#[token("void")] VOID,
	#[token("while")] WHILE,
}

impl Debug for TokenEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HELPER => write!(f, "HELPER"),
            Self::AND => write!(f, "AND"),
            Self::BOOL => write!(f, "BOOL"),
            Self::CLASS => write!(f, "CLASS"),
            Self::COLON => write!(f, "COLON"),
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

fn int(lexer: &mut Lexer<TokenEnum>) -> Option<String> {
	let number: String = lexer.slice().parse().unwrap();

	if number.parse::<i32>().is_err() {
		start_err(lexer);
		eprintln!("Integer literal overflow.");
	}

	Some(number)
}

fn new_line(lexer: &mut Lexer<TokenEnum>) -> Option<()> {
	unsafe {
		LINE += 1;
		LINE_START = lexer.span().end;
	}

	Some(())
}

fn start_err(lexer: &Lexer<'_, TokenEnum>) {
	let c1;
	let c2;
	let line;
	unsafe {
		c1 = lexer.span().start + 1 - LINE_START;
		c2 = lexer.span().end + 1 - LINE_START;
		line = LINE
	};

	eprint!("FATAL [{line},{c1}-{line},{c2}]: ");
}

fn lex_err(lexer: &Lexer<'_, TokenEnum>, string: &str) {
	let index = lexer.span().start;
	let character = string.chars().nth(index).unwrap();

	start_err(lexer);

	match character {
		'"' => eprintln!("Unterminated string literal detected"),
		_ => eprintln!("Illegal character {character}"),
	}
}

/// Runs the lexer on a given input string
pub fn lex(string: &str, file: &str) -> Result<Vec<Token>> {
	let mut lexer = TokenEnum::lexer(string);
	let mut tokens = Vec::new();

	// Open file for writing
	let mut file = File::create(file)?;

	loop {
		// If this is None we're done reading, break out of the loop
		let Some(token) = lexer.next() else {break};

		// If this isn't Ok no token was matched, print out an error
		let Ok(token) = token else {
			lex_err(&lexer, &string);

			continue
		};

		// Ignore certain helper tokens
		if token == TokenEnum::HELPER {continue}

		// Figure out line and character position in flex style
		let char;
		let line;
		unsafe {
			char = lexer.span().start + 1 - LINE_START;
			line = LINE
		};


		// Write and push token
		let out = format!("{token:?} [{line},{char}]\n");
		file.write_all(out.as_bytes())?;

		tokens.push(Token {token, line, char});
	}

	Ok(tokens)
}
