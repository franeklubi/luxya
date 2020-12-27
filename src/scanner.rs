use crate::token;
use std::str;

pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

fn scan_token<'a>(
	chars: &mut str::CharIndices,
	source: &'a String,
) -> Option<Result<token::Token<'a>, ScanError>> {
	if let Some((i, c)) = chars.next() {
		// We should be at the beginning of the next lexeme
		// println!("{}: {:?} ", i, c);
		let char_len = c.len_utf8();

		let token_type = match c {
			'(' => token::TokenType::LeftParen,
			')' => token::TokenType::RightParen,
			'{' => token::TokenType::LeftBrace,
			'}' => token::TokenType::RightBrace,
			',' => token::TokenType::Comma,
			'.' => token::TokenType::Dot,
			'-' => token::TokenType::Minus,
			'+' => token::TokenType::Plus,
			';' => token::TokenType::Semicolon,
			'*' => token::TokenType::Star,
			_ => {
				return Some(Err(ScanError {
					offset: i,
					message: String::from(format!("Unexpected token {}", c)),
				}));
			},
		};

		return Some(Ok(token::Token {
			token: token_type,
			byte_offset: i,
			byte_length: char_len,
		}));
	}

	None
}

pub fn scan_tokens(source: &String) -> (Vec<token::Token>, Vec<ScanError>) {
	let mut tokens = vec![];
	let mut errors = vec![];

	let mut chars = source.char_indices();

	// let mut current_index = 0;
	// let mut current_char = ' ';

	while let Some(res) = scan_token(&mut chars, source) {
		// We should be at the beginning of the next lexeme
		// println!("{}", res);
		match res {
			Ok(token) => tokens.push(token),
			Err(err) => {
				errors.push(err);
			}
		}
	}

	let last_offset = match tokens.last() {
		Some(token) => token.byte_offset + token.byte_length,
		None => 0,
	};

	tokens.push(
		token::Token {
			token: token::TokenType::Eof,
			byte_offset: last_offset,
			byte_length: 1,
		},
	);

	(tokens, errors)
}
