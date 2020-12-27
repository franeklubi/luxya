use crate::token::{self, TokenType};
use std::{str, iter};

pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

//
fn match_to_next(
	chars: &mut iter::Peekable<str::CharIndices>,
	expected: char
) -> bool {
	match chars.peek() {
		Some((_, c)) => *c == expected,
		None => false,
	}
}

fn scan_token<'a>(
	chars: &mut iter::Peekable<str::CharIndices>,
	source: &'a String,
) -> Option<Result<token::Token<'a>, ScanError>> {
	if let Some((i, c)) = chars.next() {
		// We should be at the beginning of the next lexeme
		// println!("{}: {:?} ", i, c);
		let char_len = c.len_utf8();

		let token_type = match c {
			'(' => TokenType::LeftParen,
			')' => TokenType::RightParen,
			'{' => TokenType::LeftBrace,
			'}' => TokenType::RightBrace,
			',' => TokenType::Comma,
			'.' => TokenType::Dot,
			'-' => TokenType::Minus,
			'+' => TokenType::Plus,
			';' => TokenType::Semicolon,
			'*' => TokenType::Star,
			'!' => {
				if match_to_next(chars, '=') {
					TokenType::BangEqual
				} else {
					TokenType::Bang
				}
			},
			'=' => {
				if match_to_next(chars, '=') {
					TokenType::EqualEqual
				} else {
					TokenType::Equal
				}
			},
			'<' => {
				if match_to_next(chars, '=') {
					TokenType::LessEqual
				} else {
					TokenType::Less
				}
			},
			'>' => {
				if match_to_next(chars, '=') {
					TokenType::GreaterEqual
				} else {
					TokenType::Greater
				}
			},
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

	let mut chars = source.char_indices().peekable();

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
