use crate::ast::expr::*;
use crate::token::*;
use std::{iter, vec};


type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub fn parse_next(tokens: ParserIter) -> Expr {
	expression(tokens)
}

fn match_token_type(t: &TokenType, against: &Vec<TokenType>) -> bool {
	against.iter().any(|a| a == t)
}

fn build_binary_expr(
	tokens: ParserIter,
	lower_precedence: impl Fn(ParserIter) -> Expr,
	types_to_match: &Vec<TokenType>,
) -> Expr {
	let mut expr = lower_precedence(tokens);

	while let Some(operator) = tokens.peek() {
		if match_token_type(&operator.token_type, types_to_match) {
			// if the peek matches we consume it
			let operator = tokens.next().unwrap();

			expr = Expr::Binary(BinaryValue {
				left: Box::new(expr),
				operator: operator,
				right: Box::new(lower_precedence(tokens)),
			});
		} else {
			break;
		}
	}

	expr
}

// grammar functions down there 👇

fn expression(tokens: ParserIter) -> Expr {
	equality(tokens)
}

fn equality(tokens: ParserIter) -> Expr {
	build_binary_expr(
		tokens,
		comparison,
		&vec![TokenType::BangEqual, TokenType::EqualEqual],
	)
}

fn comparison(tokens: ParserIter) -> Expr {
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

fn term(tokens: ParserIter) -> Expr {
	build_binary_expr(tokens, factor, &vec![TokenType::Minus, TokenType::Plus])
}

fn factor(tokens: ParserIter) -> Expr {
	build_binary_expr(tokens, unary, &vec![TokenType::Slash, TokenType::Star])
}

fn unary(tokens: ParserIter) -> Expr {
	if let Some(operator) = tokens.peek() {
		if !match_token_type(
			&operator.token_type,
			&vec![TokenType::Bang, TokenType::Minus],
		) {
			return primary(tokens);
		}

		let operator = tokens.next().unwrap();
		let right = unary(tokens);

		return Expr::Unary(UnaryValue {
			operator,
			right: Box::new(right),
		});
	}

	primary(tokens)
}

fn primary(tokens: ParserIter) -> Expr {
	match tokens.next() {
		Some(Token {
			token_type: TokenType::False,
			..
		}) => Expr::Literal(LiteralValue::False),

		Some(Token {
			token_type: TokenType::True,
			..
		}) => Expr::Literal(LiteralValue::True),

		Some(Token {
			token_type: TokenType::Nil,
			..
		}) => Expr::Literal(LiteralValue::Nil),

		Some(Token {
			token_type: TokenType::String(s),
			..
		}) => Expr::Literal(LiteralValue::String(s)),

		Some(Token {
			token_type: TokenType::Number(n),
			..
		}) => Expr::Literal(LiteralValue::Number(n)),

		Some(Token {
			token_type: TokenType::LeftParen,
			..
		}) => {
			let expr = expression(tokens);

			if let Some(Token {
				token_type: TokenType::RightParen,
				..
			}) = tokens.peek()
			{
				Expr::Grouping(GroupingValue {
					expression: Box::new(expr),
				})
			} else {
				// TODO: error handling
				Expr::Literal(LiteralValue::Nil)
			}
		}

		// TODO: error handling
		_ => Expr::Literal(LiteralValue::Nil),
	}
}
