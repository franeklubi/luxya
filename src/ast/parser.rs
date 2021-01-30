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

	while let (Some(operator), true) = (tokens.peek(), true) {
		println!("peek: {}", operator);
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
			println!("doesnt");
			break;
		}
	}

	println!("returning");
	expr
}

fn comparison(tokens: ParserIter) -> Expr {
	let _ = tokens;

	println!("comarison called");

	Expr::Literal(LiteralValue::False)
}
