use crate::token::{self, TokenType};
use std::{str, iter};


pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

// match_to_next returns true (and consumes next char)
// only if it maches the expected char
fn match_to_next(
	chars: &mut iter::Peekable<str::CharIndices>,
	expected: char
) -> bool {
	match chars.peek() {
		Some((_, c)) => *c == expected,
		None => false,
	}
}

// will consume chars until chars don't match the predicate,
// consumes the first truthy char
//
// returns index (in bytes) of where the next char would be
// (regardless of it being there or the stream ending)
fn consume_until(
	chars: &mut iter::Peekable<str::CharIndices>,
	predicate: impl Fn(char, Option<char>) -> bool,
) -> Option<usize> {
	loop {
		break match chars.next() {
			Some((i, c)) if predicate(
				c,
				chars.peek().and_then(|ci| Some(ci.1)),
			) => {
				Some(i + c.len_utf8())
			},
			Some(_) => {
				continue;
			},
			None => {
				None
			}
		}
	}
}

// consumes the next token's chars
fn scan_token<'a>(
	chars: &mut iter::Peekable<str::CharIndices>,
	source: &'a String,
) -> Option<Result<token::Token<'a>, ScanError>> {
	while let Some((i, c)) = chars.next() {
		// We should be at the beginning of the next lexeme
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
			'/' => {
				if match_to_next(chars, '/') {
					// comment goes until the end of the line
					chars.take_while(|(_, c)| *c != '\n').for_each(drop);

					continue;
				} else {
					TokenType::Slash
				}
			},
			'"' => {
				match consume_until(chars, |c, _| c == '"') {
					Some(found_i) => {
						TokenType::CharSlice(&source[i+1..found_i-1])
					},
					None => {
						return Some(Err(ScanError {
							offset: 0,
							message: String::from(
								format!("Unterminated string literal")
							),
						}));
					}
				}
			},
			c if c.is_ascii_digit() => {
				println!("is ascii digit");
				continue;
			},
			c if c.is_whitespace() => {
				continue;
			},
			_ => {
				return Some(Err(ScanError {
					offset: i,
					message: String::from(format!("Unexpected token {:?}", c)),
				}));
			},
		};

		return Some(Ok(token::Token {
			token: token_type,
			byte_offset: i,
			// TODO: char_len does not work for longer lexemes
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
