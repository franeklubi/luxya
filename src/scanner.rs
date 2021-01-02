use crate::token::{self, TokenType};
use std::{iter, str};


pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

fn resolve_identifier(identifier: &str) -> TokenType {
	match identifier {
		"and" => TokenType::And,
		"class" => TokenType::Class,
		"else" => TokenType::Else,
		"false" => TokenType::False,
		"for" => TokenType::For,
		"fun" => TokenType::Fun,
		"if" => TokenType::If,
		"nil" => TokenType::Nil,
		"or" => TokenType::Or,
		"print" => TokenType::Print,
		"return" => TokenType::Return,
		"super" => TokenType::Super,
		"this" => TokenType::This,
		"true" => TokenType::True,
		"let" => TokenType::Let,
		"const" => TokenType::Const,
		"while" => TokenType::While,
		_ => TokenType::Identifier(identifier),
	}
}

// match_to_peek returns true (and consumes next char)
// only if it maches the expected char
fn match_to_peek(
	chars: &mut iter::Peekable<str::CharIndices>,
	expected: char,
) -> bool {
	match chars.peek() {
		Some((_, c)) => *c == expected,
		None => false,
	}
}

// will consume chars while peek matches the predicate
//
// returns a result with the index (in bytes) of where the next char would be
// (regardless of it being there or the stream ending)
//
// returns an error with last_offset when the scanning has reached the eof
fn consume_while_peek(
	chars: &mut iter::Peekable<str::CharIndices>,
	predicate: impl Fn(&char) -> bool,
) -> Result<usize, usize> {
	let mut last_offset = 0;

	loop {
		break match chars.peek() {
			Some((i, c)) if predicate(c) => {
				last_offset = i + c.len_utf8();
				chars.next();

				continue;
			}
			Some((i, c)) => Ok(i + c.len_utf8()),
			None => Err(last_offset),
		};
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
				if match_to_peek(chars, '=') {
					TokenType::BangEqual
				} else {
					TokenType::Bang
				}
			}
			'=' => {
				if match_to_peek(chars, '=') {
					TokenType::EqualEqual
				} else {
					TokenType::Equal
				}
			}
			'<' => {
				if match_to_peek(chars, '=') {
					TokenType::LessEqual
				} else {
					TokenType::Less
				}
			}
			'>' => {
				if match_to_peek(chars, '=') {
					TokenType::GreaterEqual
				} else {
					TokenType::Greater
				}
			}
			'/' => {
				if match_to_peek(chars, '/') {
					// comment goes until the end of the line
					chars.take_while(|(_, c)| *c != '\n').for_each(drop);

					continue;
				} else {
					TokenType::Slash
				}
			}
			'"' => match consume_while_peek(chars, |c| *c != '"') {
				Ok(found_i) => {
					// consume the found peek
					chars.next();

					TokenType::CharSlice(&source[i + 1..found_i - 1])
				}
				Err(_) => {
					return Some(Err(ScanError {
						offset: i,
						message: String::from(format!(
							"Unterminated string literal"
						)),
					}));
				}
			},
			// the way we parse may be a lil bit problematic, because we
			// consume the `.` if the parsing somewhat fails
			// i mean idk, if it causes problems then TODO, but I don't think
			// it's that problematic for me rn
			c if c.is_ascii_digit() => {
				let consume_closure = |peek: &char| -> bool {
					return *peek == '.' || peek.is_ascii_digit();
				};

				match consume_while_peek(chars, consume_closure) {
					Ok(found_i) => {
						let to_parse = &source[i..found_i - 1];

						match to_parse.parse() {
							Ok(parsed) => TokenType::Number(parsed),
							Err(e) => {
								return Some(Err(ScanError {
									offset: i,
									message: format!(
										"Couldn't parse {}; {}",
										to_parse,
										e.to_string()
									),
								}));
							}
						}
					}
					Err(_) => {
						return Some(Err(ScanError {
							offset: i,
							message: String::from(format!(
								"I mean, the unterminated number hmm 🤔"
							)),
						}));
					}
				}
			}
			c if c.is_alphabetic() || c == '_' => {
				let identifier_end = match consume_while_peek(chars, |peek| {
					peek.is_alphanumeric() || *peek == '_'
				}) {
					Ok(found_i) | Err(found_i) => found_i,
				};

				resolve_identifier(&source[i..identifier_end - 1])
			}
			c if c.is_whitespace() => {
				continue;
			}
			_ => {
				return Some(Err(ScanError {
					offset: i,
					message: String::from(format!("Unexpected token {:?}", c)),
				}));
			}
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

	while let Some(res) = scan_token(&mut chars, source) {
		// We should be at the beginning of the next lexeme
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

	tokens.push(token::Token {
		token: token::TokenType::Eof,
		byte_offset: last_offset,
		byte_length: 1,
	});

	(tokens, errors)
}
