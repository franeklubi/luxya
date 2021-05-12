use super::types::*;
use crate::{mtcexpectone, token::*};


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

#[macro_export]
macro_rules! pm {
	($tokens:expr, $( $expected:pat )|+ $(,)?) => {{
		matches!(
			$tokens.peek(),
			Some(Token { token_type: $( $expected )|+, .. }),
		)
	}};
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
macro_rules! bbe {
	($tokens:ident, $lower_precedence:expr, $( $expected:pat )|+ $(,)?) => {{
		let mut expr = $lower_precedence($tokens)?;

		while let Some(operator) = mtc!($tokens, $( $expected )|+) {
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
macro_rules! mtc_stmt {
	($tokens:ident, $( $starts_with:pat )|+, $message:expr $(,)?) => {{
		if pm!($tokens, $( $starts_with )|+) {
			statement($tokens)
		} else {
			Err(ParseError {
				message: $message.into(),
				token: $tokens.peek().cloned(),
			})
		}
	}};
}
