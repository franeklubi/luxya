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

// grammar functions down there 👇

fn expression(tokens: ParserIter) -> Expr {
	equality(tokens)
}

fn equality(tokens: ParserIter) -> Expr {
	let mut expr = comparison(tokens);

	while let Some(operator) = tokens.peek() {
		if match_token_type(
			&operator.token_type,
			&vec![TokenType::BangEqual, TokenType::EqualEqual],
		) {
			// if the peek matches we consume it
			let operator = tokens.next().unwrap();

			let right = comparison(tokens);

			expr = Expr::Binary(BinaryValue {
				left: Box::new(expr),
				operator: operator,
				right: Box::new(right),
			});
		} else {
			break;
		}
	}

	expr
}

fn comparison(tokens: ParserIter) -> Expr {
	let mut expr = term(tokens);

	while let Some(operator) = tokens.peek() {
		if match_token_type(
			&operator.token_type,
			&vec![
				TokenType::Greater,
				TokenType::GreaterEqual,
				TokenType::Less,
				TokenType::LessEqual,
			],
		) {
			// if the peek matches we consume it
			let operator = tokens.next().unwrap();

			let right = term(tokens);

			expr = Expr::Binary(BinaryValue {
				left: Box::new(expr),
				operator: operator,
				right: Box::new(right),
			});
		} else {
			break;
		}
	}

	expr
}

fn term(tokens: ParserIter) -> Expr {
	let mut expr = factor(tokens);

	while let Some(operator) = tokens.peek() {
		if match_token_type(
			&operator.token_type,
			&vec![TokenType::Minus, TokenType::Plus],
		) {
			// if the peek matches we consume it
			let operator = tokens.next().unwrap();

			let right = factor(tokens);

			expr = Expr::Binary(BinaryValue {
				left: Box::new(expr),
				operator: operator,
				right: Box::new(right),
			});
		} else {
			break;
		}
	}

	expr
}

fn factor(tokens: ParserIter) -> Expr {
	let mut expr = unary(tokens);

	while let Some(operator) = tokens.peek() {
		if match_token_type(
			&operator.token_type,
			&vec![TokenType::Slash, TokenType::Star],
		) {
			// if the peek matches we consume it
			let operator = tokens.next().unwrap();

			let right = unary(tokens);

			expr = Expr::Binary(BinaryValue {
				left: Box::new(expr),
				operator: operator,
				right: Box::new(right),
			});
		} else {
			break;
		}
	}

	expr
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
	let nastepny = tokens.next();

	match nastepny {
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
