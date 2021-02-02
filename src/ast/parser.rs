use crate::ast::expr::*;
use crate::token::*;
use std::{iter, vec};


type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub struct ParseError {
	pub token: Option<Token>,
	pub message: String,
}

pub fn parse_next(tokens: ParserIter) -> Result<Expr, ParseError> {
	expression(tokens)
}

fn match_token_type(t: &TokenType, against: &Vec<TokenType>) -> bool {
	against.iter().any(|a| a == t)
}

fn build_binary_expr(
	tokens: ParserIter,
	lower_precedence: impl Fn(ParserIter) -> Result<Expr, ParseError>,
	types_to_match: &Vec<TokenType>,
) -> Result<Expr, ParseError> {
	let mut expr = lower_precedence(tokens)?;

	while let Some(operator) = tokens.peek() {
		if !match_token_type(&operator.token_type, types_to_match) {
			break;
		}


		// if the peek matches we consume it
		let operator = tokens.next().unwrap();

		let right = lower_precedence(tokens);

		match right {
			Ok(r) => {
				expr = Expr::Binary(BinaryValue {
					left: Box::new(expr),
					operator: operator,
					right: Box::new(r),
				});
			}
			_ => return right,
		}
	}

	Ok(expr)
}

// call only if the token that the parser choked on is not ';'
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

fn expression(tokens: ParserIter) -> Result<Expr, ParseError> {
	equality(tokens)
}

fn equality(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(
		tokens,
		comparison,
		&vec![TokenType::BangEqual, TokenType::EqualEqual],
	)
}

fn comparison(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(
		tokens,
		term,
		&vec![
			TokenType::Greater,
			TokenType::GreaterEqual,
			TokenType::Less,
			TokenType::LessEqual,
		],
	)
}

fn term(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, factor, &vec![TokenType::Minus, TokenType::Plus])
}

fn factor(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, unary, &vec![TokenType::Slash, TokenType::Star])
}

fn unary(tokens: ParserIter) -> Result<Expr, ParseError> {
	if let Some(operator) = tokens.peek() {
		if !match_token_type(
			&operator.token_type,
			&vec![TokenType::Bang, TokenType::Minus],
		) {
			return primary(tokens);
		}

		let operator = tokens.next().unwrap();

		let right = unary(tokens);

		match right {
			Ok(r) => {
				return Ok(Expr::Unary(UnaryValue {
					operator,
					right: Box::new(r),
				}));
			}
			_ => return right,
		}
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
			let expr = expression(tokens);

			match expr {
				Ok(e) => {
					if let Some(Token {
						token_type: TokenType::RightParen,
						..
					}) = tokens.peek()
					{
						Ok(Expr::Grouping(GroupingValue {
							expression: Box::new(e),
						}))
					} else {
						Err(ParseError {
							token,
							message: "Expected '('".into(),
						})
					}
				}
				_ => return expr,
			}
		}

		_ => Err(ParseError {
			token,
			message: "Expected expression".into(),
		}),
	}
}
