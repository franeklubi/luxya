use std::fmt;

pub struct Scanner {
	source: String,
}

impl Scanner {
	pub fn from(source: String) -> Scanner {
		return Scanner {
			source: source,
		}
	}

	pub fn scan_tokens(&self) -> Vec<Token> {
		return self.source.split(' ').map(|token| {
			return Token {
				token,
			}
		}).collect();
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
