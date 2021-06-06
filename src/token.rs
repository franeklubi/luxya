use std::{fmt, mem, rc::Rc};


#[derive(Clone, Debug)]
#[allow(clippy::module_name_repetitions)]
pub enum TokenType {
	// Single-character tokens
	LeftParen,
	RightParen,
	LeftBrace,
	RightBrace,
	Comma,
	Dot,
	Minus,
	Plus,
	Semicolon,
	Colon,
	Slash,
	Star,
	LeftSquareBracket,
	RightSquareBracket,
	Modulo,

	// One Or Two Character Tokens
	Bang,
	BangEqual,
	Equal,
	EqualEqual,
	Greater,
	GreaterEqual,
	Less,
	LessEqual,

	// Literals
	Identifier(Rc<str>),
	String(Rc<str>),
	Number(f64),
	Char(char),

	// Keywords
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
	Break,
	Continue,
	Extends,
}

impl TokenType {
	pub fn repr(&self) -> String {
		match self {
			TokenType::String(s) => format!("{:?}", s),
			TokenType::Identifier(i) => format!("{}", i),
			TokenType::Number(n) => format!("{:?}", n),
			TokenType::Char(c) => format!("{:?}", c),
			_ => self.human_type().to_owned(),
		}
	}

	pub const fn human_type(&self) -> &str {
		match self {
			TokenType::String(_) => "string",
			TokenType::Identifier(_) => "identifier",
			TokenType::Number(_) => "number",
			TokenType::Char(_) => "char",
			TokenType::LeftParen => "(",
			TokenType::RightParen => ")",
			TokenType::LeftBrace => "{",
			TokenType::RightBrace => "}",
			TokenType::LeftSquareBracket => "[",
			TokenType::RightSquareBracket => "]",
			TokenType::Comma => ",",
			TokenType::Dot => ".",
			TokenType::Minus => "-",
			TokenType::Plus => "+",
			TokenType::Colon => ":",
			TokenType::Semicolon => ";",
			TokenType::Slash => "/",
			TokenType::Star => "*",
			TokenType::Bang => "!",
			TokenType::BangEqual => "!=",
			TokenType::Equal => "=",
			TokenType::EqualEqual => "==",
			TokenType::Greater => ">",
			TokenType::GreaterEqual => ">=",
			TokenType::Less => "<",
			TokenType::LessEqual => "<=",
			TokenType::And => "and",
			TokenType::Class => "class",
			TokenType::Else => "else",
			TokenType::False => "false",
			TokenType::Fun => "fun",
			TokenType::For => "for",
			TokenType::If => "if",
			TokenType::Nil => "nil",
			TokenType::Or => "or",
			TokenType::Print => "print",
			TokenType::Return => "return",
			TokenType::Super => "super",
			TokenType::This => "this",
			TokenType::True => "true",
			TokenType::Let => "let",
			TokenType::Const => "const",
			TokenType::Break => "break",
			TokenType::Continue => "continue",
			TokenType::Extends => "extends",
			TokenType::Modulo => "%",
		}
	}
}

#[derive(Clone)]
pub struct Token {
	pub location: Location,
	pub token_type: TokenType,
}

#[derive(Clone, Copy)]
pub struct Location {
	pub byte_offset: usize,
	pub byte_length: usize,
}

impl fmt::Display for Token {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(
			f,
			"{}\tfrom: {};\tto: {};",
			self.token_type,
			self.location.byte_offset,
			self.location.byte_offset + self.location.byte_length,
		)
	}
}

impl fmt::Display for TokenType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.repr())
	}
}

impl PartialEq for TokenType {
	fn eq(&self, other: &Self) -> bool {
		mem::discriminant(self) == mem::discriminant(other)
	}
}
