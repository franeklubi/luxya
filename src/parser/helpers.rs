use super::types::*;
use crate::{expect_one, token::*};


#[inline(always)]
pub fn expect_semicolon(tokens: ParserIter) -> Result<Token, ParseError> {
	expect_one!(tokens, TokenType::Semicolon)
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

#[macro_export]
macro_rules! peek_matches {
	($tokens:expr, $( $expected:pat )|+ $(,)?) => {{
		matches!(
			$tokens.peek(),
			Some(Token { token_type: $( $expected )|+, .. }),
		)
	}};
}

#[macro_export]
macro_rules! match_then_consume {
	($tokens:expr, $( $expected:pat )|+ $(,)?) => {{
		match $tokens.peek().map(|t| &t.token_type) {
			Some($( $expected )|+) => $tokens.next(),
			_ => None,
		}
	}};
}

#[macro_export]
macro_rules! expect {
	($tokens:ident, $( $expected:pat )|+, $message:expr $(,)?) => {{
		match_then_consume!(
			$tokens,
			$( $expected )|+,
		).ok_or_else(|| ParseError {
			message: $message.into(),
			token: $tokens.peek().cloned(),
		})
	}};
}

#[macro_export]
macro_rules! expect_one {
	($tokens:ident, $expected:expr $(,)?) => {{
		// we can't use peek_matches, because we need to generate an error
		// based on the expected token type
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

#[macro_export]
macro_rules! build_binary_expr {
	($tokens:ident, $lower_precedence:expr, $( $expected:pat )|+ $(,)?) => {{
		let mut expr = $lower_precedence($tokens)?;

		while let Some(operator) = match_then_consume!(
			$tokens,
			$( $expected )|+,
		) {
			let right = $lower_precedence($tokens)?;

			expr = Expr::Binary(BinaryValue {
				left: Box::new(expr),
				operator,
				right: Box::new(right),
			});
		}

		Ok(expr)
	}};
}

#[macro_export]
macro_rules! match_then_consume_stmt {
	($tokens:ident, $( $starts_with:pat )|+, $message:expr $(,)?) => {{
		if peek_matches!($tokens, $( $starts_with )|+) {
			statement($tokens)
		} else {
			Err(ParseError {
				message: $message.into(),
				token: $tokens.peek().cloned(),
			})
		}
	}};
}
