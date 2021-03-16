use super::types::*;
use crate::token::*;


pub fn match_token_type(t: &TokenType, expected: &[TokenType]) -> bool {
	expected.iter().any(|a| a == t)
}

pub fn peek_matches(tokens: ParserIter, expected: &[TokenType]) -> bool {
	tokens
		.peek()
		.map_or(false, |v| match_token_type(&v.token_type, expected))
}

/// Tries peek of ParserIter against provided token types
///
/// Returns `Some(Token)` if successful and consumes the token, `None` otherwise
pub fn match_then_consume(
	tokens: ParserIter,
	expected: &[TokenType],
) -> Option<Token> {
	tokens
		.peek()
		.map(|t| match_token_type(&t.token_type, expected))
		.and_then(|b| b.then(|| tokens.next().unwrap()))
}

pub fn expect(
	tokens: ParserIter,
	expected: &[TokenType],
	override_message: Option<&str>,
) -> Result<Token, ParseError> {
	match_then_consume(tokens, expected).ok_or_else(|| {
		let message = if let Some(m) = override_message {
			m.to_string()
		} else {
			gen_expected_msg(expected)
		};

		ParseError {
			message,
			token: tokens.peek().cloned(),
		}
	})
}

pub fn gen_expected_msg(expected: &[TokenType]) -> String {
	let msg = if expected.len() > 1 {
		"Expected one of: "
	} else {
		"Expected: "
	}
	.to_string();

	let enumerated: String = expected
		.iter()
		.map(|e| format!("`{}`", e))
		.collect::<Vec<String>>()
		.join(", ");

	msg + &enumerated
}

pub fn expect_semicolon(tokens: ParserIter) -> Result<Token, ParseError> {
	expect(tokens, &[TokenType::Semicolon], None)
}

// call only if the token that the parser choked on is not ';'
// TODO: rethink/rewrite this
pub fn synchronize(tokens: ParserIter) {
	while let Some(token) = tokens.peek() {
		match token.token_type {
			TokenType::Class
			| TokenType::Fun
			| TokenType::Let
			| TokenType::Const
			| TokenType::For
			| TokenType::If
			| TokenType::While
			| TokenType::Print
			| TokenType::Return => {
				break;
			}

			_ => {
				if TokenType::Semicolon == tokens.next().unwrap().token_type {
					break;
				}
			}
		}
	}
}
