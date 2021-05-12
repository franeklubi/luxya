use super::{parse::*, types::*};
use crate::{
	ast::{expr::*, stmt::*},
	token::*,
};


#[inline(always)]
pub fn match_token_type(t: &TokenType, expected: &[TokenType]) -> bool {
	expected.iter().any(|a| a == t)
}

#[inline(always)]
pub fn peek_matches(tokens: ParserIter, expected: &[TokenType]) -> bool {
	tokens
		.peek()
		.map_or(false, |v| match_token_type(&v.token_type, expected))
}

/// Tries peek of ParserIter against provided token types
///
/// Returns `Some(Token)` if successful and consumes the token, `None` otherwise
#[inline(always)]
pub fn match_then_consume(
	tokens: ParserIter,
	expected: &[TokenType],
) -> Option<Token> {
	tokens
		.peek()
		.map(|t| match_token_type(&t.token_type, expected))
		.and_then(|b| b.then(|| tokens.next().unwrap()))
}

#[macro_export]
macro_rules! mtc {
	($tokens:expr, $( $expected:pat )|+ $(,)?) => {{
		match $tokens.peek().map(|t| &t.token_type) {
			Some($( $expected )|+) => $tokens.next(),
			_ => None,
		}
	}};
}

#[macro_export]
macro_rules! mtcexpect {
	($tokens:ident, $( $expected:pat )|+, $message:expr $(,)?) => {{
		mtc!($tokens, $( $expected )|+).ok_or_else(|| ParseError {
			message: $message.into(),
			token: $tokens.peek().cloned(),
		})
	}};
}

#[macro_export]
macro_rules! mtcexpectone {
	($tokens:ident, $expected:expr $(,)?) => {{
		match $tokens.peek() {
			Some(Token { token_type, .. }) if token_type == &$expected => {
				// i mean we just matched that, unsafe is pretty justified
				Ok(unsafe { $tokens.next().unwrap_unchecked() })
			}
			_ => Err(ParseError {
				message: format!("Expected `{}`", $expected.human_type()),
				token: $tokens.peek().cloned(),
			}),
		}
	}};
}

#[inline(always)]
pub fn expect_semicolon(tokens: ParserIter) -> Result<Token, ParseError> {
	mtcexpectone!(tokens, TokenType::Semicolon)
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

pub fn statement_from(
	tokens: ParserIter,
	starts_with: &[TokenType],
) -> Result<Option<Stmt>, ParseError> {
	if peek_matches(tokens, starts_with) {
		statement(tokens)
	} else {
		Err(ParseError {
			message: format!(
				"Expected statement starting with {:?}",
				starts_with
			),
			token: tokens.peek().cloned(),
		})
	}
}

pub fn build_binary_expr(
	tokens: ParserIter,
	lower_precedence: impl Fn(ParserIter) -> Result<Expr, ParseError>,
	types_to_match: &[TokenType],
) -> Result<Expr, ParseError> {
	let mut expr = lower_precedence(tokens)?;

	while let Some(operator) = match_then_consume(tokens, types_to_match) {
		let right = lower_precedence(tokens)?;

		expr = Expr::Binary(BinaryValue {
			left: Box::new(expr),
			operator,
			right: Box::new(right),
		});
	}

	Ok(expr)
}
