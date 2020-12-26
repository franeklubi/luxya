use std::{fmt, convert};

pub struct Scanner {
	source: String,
}

impl convert::From<String> for Scanner {
	fn from(source: String) -> Self {
		Scanner {
			source,
		}
	}
}

impl Scanner {
	pub fn scan_tokens(&self) -> Vec<Token> {
		self.source.split(' ').map(|token| {
			Token {
				token,
			}
		}).collect()
	}
}

pub struct Token<'a> {
	token: &'a str,
}

impl fmt::Display for Token<'_> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.token)
	}
}
