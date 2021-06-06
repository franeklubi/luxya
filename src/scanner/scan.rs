use super::{helpers::*, types::*};
use crate::token::{self, Location, TokenType};


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
		"extends" => TokenType::Extends,
		_ => TokenType::Identifier(identifier.into()),
	}
}

// consumes the next token's chars
fn scan_token(
	chars: ScannerIter,
	source: &str,
) -> Result<Option<token::Token>, ScanError> {
	// using while, because we want to skip unimportant chars, like whitespace
	while let Some((i, c)) = chars.next() {
		// We should be at the beginning of the next lexeme
		let mut token_len = c.len_utf8();

		let token_type = match c {
			'(' => TokenType::LeftParen,
			')' => TokenType::RightParen,
			'{' => TokenType::LeftBrace,
			'}' => TokenType::RightBrace,
			'[' => TokenType::LeftSquareBracket,
			']' => TokenType::RightSquareBracket,
			',' => TokenType::Comma,
			'.' => TokenType::Dot,
			'-' => TokenType::Minus,
			'+' => TokenType::Plus,
			'%' => TokenType::Modulo,
			';' => TokenType::Semicolon,
			':' => TokenType::Colon,
			'*' => TokenType::Star,
			'!' => {
				if let Some((_, '=')) = chars.peek() {
					chars.next();

					token_len += 1;

					TokenType::BangEqual
				} else {
					TokenType::Bang
				}
			}
			'=' => {
				if let Some((_, '=')) = chars.peek() {
					chars.next();

					token_len += 1;

					TokenType::EqualEqual
				} else {
					TokenType::Equal
				}
			}
			'<' => {
				if let Some((_, '=')) = chars.peek() {
					chars.next();

					token_len += 1;

					TokenType::LessEqual
				} else {
					TokenType::Less
				}
			}
			'>' => {
				if let Some((_, '=')) = chars.peek() {
					chars.next();

					token_len += 1;

					TokenType::GreaterEqual
				} else {
					TokenType::Greater
				}
			}
			'/' => {
				if let Some((_, '/')) = chars.peek() {
					// comment goes until the end of the line
					chars.take_while(|(_, c)| *c != '\n').for_each(drop);

					continue;
				}

				TokenType::Slash
			}
			'"' => {
				let res = consume_while_peek(chars, |c| *c != '"');

				if res.hit_eof {
					return Err(ScanError {
						offset: i,
						message: "Unterminated string literal".to_owned(),
					});
				}

				chars.next();

				TokenType::String(source[i + 1..res.last_offset].into())
			}
			'\'' => {
				let c = expect_char(chars, '\'', i, None)?;

				let closer = expect_char(chars, c, i, None)?;

				if '\'' == closer {
					TokenType::Char(c)
				} else {
					// get rid of remaining chars
					let _ = consume_while_peek(chars, |c| *c != '\'');
					chars.next();

					return Err(ScanError {
						offset: i,
						message: "Expected closing ' after char".to_owned(),
					});
				}
			}
			c if c.is_ascii_digit() => {
				let number_end = consume_while_peek(chars, |c| {
					*c == '.' || c.is_ascii_digit()
				})
				.last_offset;

				let to_parse = &source[i..number_end];

				token_len = number_end - i;

				TokenType::Number(to_parse.parse().expect("Parsed number"))
			}
			c if c.is_alphabetic() || c == '_' => {
				let identifier_end = consume_while_peek(chars, |peek| {
					peek.is_alphanumeric() || *peek == '_'
				})
				.last_offset;

				token_len = identifier_end - i;

				resolve_identifier(&source[i..identifier_end])
			}
			c if c.is_whitespace() => {
				continue;
			}
			_ => {
				return Err(ScanError {
					offset: i,
					message: format!("Unexpected character {:?}", c),
				});
			}
		};

		return Ok(Some(token::Token {
			token_type,
			location: Location {
				byte_offset: i,
				byte_length: token_len,
			},
		}));
	}

	Ok(None)
}

pub fn scan(source: &str) -> (Vec<token::Token>, Vec<ScanError>) {
	let mut tokens = vec![];
	let mut errors = vec![];

	let mut chars = source.char_indices().peekable();

	while let Some(_peek) = chars.peek() {
		// We should be at the beginning of the next lexeme
		match scan_token(&mut chars, source) {
			Ok(Some(token)) => tokens.push(token),
			Ok(None) => break, // iterator is exhausted
			Err(err) => errors.push(err),
		}
	}

	(tokens, errors)
}
