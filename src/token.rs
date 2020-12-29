use std::fmt;

// TODO: disallow that bby
#[allow(dead_code)]
pub enum TokenType<'a> {
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
	Identifier(&'a str),
	CharSlice(&'a str),
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

impl fmt::Display for TokenType<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			TokenType::Identifier(s) | TokenType::CharSlice(s) => {
				write!(f, "{:?}", s)
			}
			TokenType::Number(n) => {
				write!(f, "{:?}", n)
			}
			_ => write!(f, "TokenType"),
		}
	}
}

pub struct Token<'a> {
	pub byte_offset: usize,
	pub byte_length: usize,
	pub token: TokenType<'a>,
}

impl fmt::Display for Token<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{};\tfrom: {};\tto: {};",
			self.token,
			self.byte_offset,
			self.byte_offset + self.byte_length,
		)
	}
}
