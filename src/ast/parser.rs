use crate::ast::{expr::*, stmt::*};
use crate::token::*;
use std::{iter, vec};


type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub struct ParseError {
	pub token: Option<Token>,
	pub message: String,
}

fn match_token_type(t: &TokenType, expected: &[TokenType]) -> bool {
	expected.iter().any(|a| a == t)
}

/// Tries peek of ParserIter against provided token types
///
/// Returns `Some(Token)` if successful and consumes the token, `None` otherwise
fn match_then_consume(
	tokens: ParserIter,
	expected: &[TokenType],
) -> Option<Token> {
	if let Some(token) = tokens.peek() {
		if match_token_type(&token.token_type, expected) {
			// consume the token that was matched
			Some(tokens.next().unwrap())
		} else {
			None
		}
	} else {
		None
	}
}

fn expect(
	tokens: ParserIter,
	expected: &[TokenType],
	override_message: Option<&str>,
) -> Result<Token, ParseError> {
	match_then_consume(tokens, expected).ok_or_else(|| {
		let message = {
			if let Some(m) = override_message {
				m.to_string()
			} else {
				let msg = if expected.len() > 1 {
					"Expected one of: "
				} else {
					"Expected: "
				};

				expected
					.iter()
					.fold(msg.to_string(), |acc, tt| acc + &format!("`{}`", tt))
			}
		};

		ParseError {
			message,
			token: tokens.peek().cloned(),
		}
	})
}

fn expect_semicolon(tokens: ParserIter) -> Result<Token, ParseError> {
	expect(
		tokens,
		&[TokenType::Semicolon, TokenType::Eof],
		Some("Expected `;`"),
	)
}

pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<ParseError>) {
	let mut tokens = tokens.into_iter().peekable();

	let mut statements = Vec::new();
	let mut errors = Vec::new();

	while let Some(token) = tokens.peek() {
		if token.token_type == TokenType::Eof {
			break;
		}

		match statement(&mut tokens) {
			Ok(Some(s)) => statements.push(s),

			// TODO: synchronize here or smth
			Err(s) => errors.push(s),
			_ => (),
		}
	}

	(statements, errors)
}

fn build_binary_expr(
	tokens: ParserIter,
	lower_precedence: impl Fn(ParserIter) -> Result<Expr, ParseError>,
	types_to_match: &[TokenType],
) -> Result<Expr, ParseError> {
	let mut expr = lower_precedence(tokens)?;

	while let Some(operator) = tokens.peek() {
		if !match_token_type(&operator.token_type, types_to_match) {
			break;
		}

		// if the peek matches we consume it
		let operator = tokens.next().unwrap();

		let right = lower_precedence(tokens)?;

		expr = Expr::Binary(BinaryValue {
			left: Box::new(expr),
			operator,
			right: Box::new(right),
		});
	}

	Ok(expr)
}

// call only if the token that the parser choked on is not ';'
// TODO: delete that
#[allow(dead_code)]
fn synchronize(tokens: ParserIter) {
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
				if let Some(Token {
					token_type: TokenType::Semicolon,
					..
				}) = tokens.next()
				{
					break;
				}
			}
		}
	}
}

// grammar functions down there 👇

/// Statement is the first grammar function.
///
/// It is the only function that can return Ok(None)
fn statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	fn print_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
		let stmt = Stmt::Print(PrintValue {
			expression: Box::new(expression(tokens)?),
		});

		expect_semicolon(tokens)?;

		Ok(Some(stmt))
	}

	fn expression_statement(
		tokens: ParserIter,
	) -> Result<Option<Stmt>, ParseError> {
		let stmt = Stmt::Expression(ExpressionValue {
			expression: Box::new(expression(tokens)?),
		});

		expect_semicolon(tokens)?;

		Ok(Some(stmt))
	}

	let consumed_token =
		match_then_consume(tokens, &[TokenType::Print, TokenType::Semicolon]);

	let token_type = consumed_token.map(|ct| ct.token_type);

	match token_type {
		Some(TokenType::Print) => print_statement(tokens),
		// that's an empty statement of sorts 🤔
		Some(TokenType::Semicolon) => Ok(None),
		_ => expression_statement(tokens),
	}
}

fn expression(tokens: ParserIter) -> Result<Expr, ParseError> {
	equality(tokens)
}

fn equality(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(
		tokens,
		comparison,
		&[TokenType::BangEqual, TokenType::EqualEqual],
	)
}

fn comparison(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(
		tokens,
		term,
		&[
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		],
	)
}

fn term(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, factor, &[TokenType::Minus, TokenType::Plus])
}

fn factor(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, unary, &[TokenType::Slash, TokenType::Star])
}

fn unary(tokens: ParserIter) -> Result<Expr, ParseError> {
	if let Some(operator) = tokens.peek() {
		if !match_token_type(
			&operator.token_type,
			&[TokenType::Bang, TokenType::Minus],
		) {
			return primary(tokens);
		}

		let operator = tokens.next().unwrap();

		let right = unary(tokens)?;

		return Ok(Expr::Unary(UnaryValue {
			operator,
			right: Box::new(right),
		}));
	}

	primary(tokens)
}

fn primary(tokens: ParserIter) -> Result<Expr, ParseError> {
	let token = tokens.next();

	match token {
		Some(Token {
			token_type: TokenType::False,
			..
		}) => Ok(Expr::Literal(LiteralValue::False)),

		Some(Token {
			token_type: TokenType::True,
			..
		}) => Ok(Expr::Literal(LiteralValue::True)),

		Some(Token {
			token_type: TokenType::Nil,
			..
		}) => Ok(Expr::Literal(LiteralValue::Nil)),

		Some(Token {
			token_type: TokenType::String(s),
			..
		}) => Ok(Expr::Literal(LiteralValue::String(s))),

		Some(Token {
			token_type: TokenType::Number(n),
			..
		}) => Ok(Expr::Literal(LiteralValue::Number(n))),

		Some(Token {
			token_type: TokenType::LeftParen,
			..
		}) => {
			let expr = expression(tokens)?;

			match tokens.peek() {
				Some(Token {
					token_type: TokenType::RightParen,
					..
				}) => Ok(Expr::Grouping(GroupingValue {
					expression: Box::new(expr),
				})),
				_ => Err(ParseError {
					token,
					message: "Expected ')'".into(),
				}),
			}
		}

		_ => Err(ParseError {
			token,
			message: "Expected expression".into(),
		}),
	}
}
