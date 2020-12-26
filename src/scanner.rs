use std::convert;

use crate::token;


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
	pub fn scan_tokens(&self) -> Vec<token::Token> {
		self.source.split(' ').map(|lexeme| {
			token::Token {
				token: token::TokenType::CharSlice(lexeme.trim()),
				offset: 0,
				length: 0,
			}
		}).collect()
	}
}
