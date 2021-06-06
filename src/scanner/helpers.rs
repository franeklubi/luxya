use super::types::{ConsumptionResult, ScanError, ScannerIter};
use crate::token::TokenType;


/// will consume chars while peek matches the predicate
///
/// returns a struct with the last offset (in bytes) of where
/// the next char would be
/// (regardless of it being there or the iterator ending)
///
/// sets `hit_eof` when the scanning has reached eof
pub fn consume_while_peek(
	chars: ScannerIter,
	predicate: impl Fn(&char) -> bool,
) -> ConsumptionResult {
	let mut last_offset = 0;

	loop {
		break match chars.peek() {
			// peek matches the predicate, so we continue on
			Some((i, c)) if predicate(c) => {
				last_offset = i + c.len_utf8();
				chars.next();

				continue;
			}
			// char doesn't match the predicate, so we return the result
			Some((i, _)) => ConsumptionResult {
				last_offset: *i,
				hit_eof: false,
			},
			// we hit the eof
			None => ConsumptionResult {
				last_offset,
				hit_eof: true,
			},
		};
	}
}

pub fn expect_char(
	chars: ScannerIter,
	after: char,
	offset: usize,
	override_message: Option<&str>,
) -> Result<char, ScanError> {
	if let Some(c) = chars.next() {
		Ok(c.1)
	} else if let Some(msg) = override_message {
		Err(ScanError {
			message: msg.to_owned(),
			offset,
		})
	} else {
		Err(ScanError {
			message: format!("Expected char after `{}`", after),
			offset,
		})
	}
}

pub fn tokenize_identifier(identifier: &str) -> TokenType {
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
