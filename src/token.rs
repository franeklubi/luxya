use std::fmt;

pub enum TokenType {
	// Single-character tokens.
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Slash,
	Star,

	// One Or Two Character Tokens.
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals.
	Identifier(String),
	String(String),
	Number(f64),

	// Keywords.
	And,
	Class,
	Else,
	False,
	Fun,
	For,
	If,
	Nil,
	Or,
	Print,
	Return,
	Super,
	This,
	True,
	Let,
	Const,
	While,

	Eof,
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			TokenType::Identifier(s) => {
				write!(f, "{}", s)
			}
			TokenType::String(s) => {
				write!(f, "{:?}", s)
			}
			TokenType::Number(n) => {
				write!(f, "{:?}", n)
			}
			_ => write!(f, "Token"),
		}
	}
}

pub struct Token {
	pub byte_offset: usize,
	pub byte_length: usize,
	pub token_type: TokenType,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{};\tfrom: {};\tto: {};",
			self.token_type,
			self.byte_offset,
			self.byte_offset + self.byte_length,
		)
	}
}
