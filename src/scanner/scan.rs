use super::{helpers::*, types::*};
use crate::token::{self, TokenType};


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
		"break" => TokenType::Break,
		"continue" => TokenType::Continue,
		_ => TokenType::Identifier(identifier.into()),
	}
}

// consumes the next token's chars
fn scan_token(
	chars: ScannerIter,
	source: &str,
	// TODO: change that to only return result lol
) -> Option<Result<token::Token, ScanError>> {
	while let Some((i, c)) = chars.next() {
		// We should be at the beginning of the next lexeme
		let mut token_len = c.len_utf8();

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
					chars.next();

					token_len += 1;

					TokenType::BangEqual
				} else {
					TokenType::Bang
				}
			}
			'=' => {
				if match_to_peek(chars, '=') {
					chars.next();

					token_len += 1;

					TokenType::EqualEqual
				} else {
					TokenType::Equal
				}
			}
			'<' => {
				if match_to_peek(chars, '=') {
					chars.next();

					token_len += 1;

					TokenType::LessEqual
				} else {
					TokenType::Less
				}
			}
			'>' => {
				if match_to_peek(chars, '=') {
					chars.next();

					token_len += 1;

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
				Ok(identifier_end) => {
					// consume the found peek
					chars.next();

					token_len = identifier_end - i;

					TokenType::String(source[i + 1..identifier_end].into())
				}
				Err(_) => {
					return Some(Err(ScanError {
						offset: i,
						message: "Unterminated string literal".to_owned(),
					}));
				}
			},
			// the way we parse may be a lil bit problematic, because we
			// consume the `.` if the parsing somewhat fails
			// i mean idk, if it causes problems then TODO, but I don't think
			// it's that problematic for me rn
			c if c.is_ascii_digit() => {
				let consume_closure = |peek: &char| -> bool {
					*peek == '.' || peek.is_ascii_digit()
				};

				match consume_while_peek(chars, consume_closure) {
					Ok(identifier_end) => {
						let to_parse = &source[i..identifier_end];

						token_len = identifier_end - i;

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
							message: "I mean, the unterminated number hmm 🤔"
								.to_owned(),
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

				token_len = identifier_end - i;

				resolve_identifier(&source[i..identifier_end])
			}
			c if c.is_whitespace() => {
				continue;
			}
			_ => {
				return Some(Err(ScanError {
					offset: i,
					message: format!("Unexpected token {:?}", c),
				}));
			}
		};

		return Some(Ok(token::Token {
			token_type,
			byte_offset: i,
			byte_length: token_len,
		}));
	}

	None
}

pub fn scan(source: &str) -> (Vec<token::Token>, Vec<ScanError>) {
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
		token_type: token::TokenType::Eof,
		byte_offset: last_offset,
		byte_length: 1,
	});

	(tokens, errors)
}
