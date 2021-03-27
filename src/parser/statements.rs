use super::{helpers::*, parse::*, types::*};
use crate::{
	ast::{expr::*, stmt::*},
	token::*,
};

use std::vec;


pub fn print_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let stmt = Stmt::Print(PrintValue {
		expression: Box::new(expression(tokens)?),
	});

	expect_semicolon(tokens)?;

	Ok(Some(stmt))
}

pub fn expression_statement(
	tokens: ParserIter,
) -> Result<Option<Stmt>, ParseError> {
	let expr = expression(tokens)?;

	// expect semicolon only if the expression is not a function
	let semicolon_expected = !matches!(expr, Expr::Function(_));

	let stmt = Stmt::Expression(ExpressionValue {
		expression: Box::new(expr),
	});

	if semicolon_expected {
		expect_semicolon(tokens)?;
	}

	Ok(Some(stmt))
}

pub fn if_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let condition = Box::new(expression(tokens)?);

	let then = Box::new(expect_statement(
		tokens,
		&[TokenType::LeftBrace, TokenType::If],
	)?);
	let otherwise = if match_then_consume(tokens, &[TokenType::Else]).is_some()
	{
		Some(Box::new(expect_statement(
			tokens,
			&[TokenType::LeftBrace, TokenType::If],
		)?))
	} else {
		None
	};

	Ok(Some(Stmt::If(IfValue {
		condition,
		then,
		otherwise,
	})))
}

pub fn block_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let mut statements = Vec::new();

	while !peek_matches(tokens, &[TokenType::RightBrace]) {
		if let Some(d) = declaration(tokens)? {
			statements.push(d);
		}
	}

	expect(tokens, &[TokenType::RightBrace], None)?;

	if statements.is_empty() {
		Ok(None)
	} else {
		Ok(Some(Stmt::Block(BlockValue { statements })))
	}
}

pub fn for_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let expected = &[TokenType::Semicolon, TokenType::Let, TokenType::Const];

	if !peek_matches(tokens, expected) {
		return Err(ParseError {
			message: gen_expected_msg(expected),
			token: tokens.peek().cloned(),
		});
	}

	// parse initializer
	let initializer =
		if peek_matches(tokens, &[TokenType::Let, TokenType::Const]) {
			declaration(tokens)?
		} else {
			tokens.next();

			None
		};

	// parse condition
	let condition =
		if match_then_consume(tokens, &[TokenType::Semicolon]).is_some() {
			None
		} else {
			let expr = expression(tokens)?;

			expect(tokens, &[TokenType::Semicolon], None)?;

			Some(expr)
		};

	// parse closer (the increment or whatever)
	let closer = if peek_matches(tokens, &[TokenType::LeftBrace]) {
		None
	} else {
		Some(expression(tokens)?)
	};

	// parse while body
	let body = expect_statement(tokens, &[TokenType::LeftBrace])?;

	let for_stmt = Stmt::For(ForValue {
		condition: condition.map(Box::new),
		body: Box::new(body),
		closer: closer.map(|c| {
			Box::new(Stmt::Expression(ExpressionValue {
				expression: Box::new(c),
			}))
		}),
	});

	// determine if for body requires to be in a separate block
	// because of the initializer
	let for_body = if let Some(initializer) = initializer {
		Stmt::Block(BlockValue {
			statements: vec![initializer, for_stmt],
		})
	} else {
		for_stmt
	};

	Ok(Some(for_body))
}

pub fn return_statement(
	tokens: ParserIter,
	keyword: Token,
) -> Result<Option<Stmt>, ParseError> {
	let expression = if !peek_matches(tokens, &[TokenType::Semicolon]) {
		Some(expression(tokens)?)
	} else {
		None
	};

	expect_semicolon(tokens)?;

	Ok(Some(Stmt::Return(ReturnValue {
		expression,
		keyword,
	})))
}

pub fn break_statement(
	tokens: ParserIter,
	keyword: Token,
) -> Result<Option<Stmt>, ParseError> {
	expect_semicolon(tokens)?;

	Ok(Some(Stmt::Break(BreakValue { keyword })))
}

pub fn continue_statement(
	tokens: ParserIter,
	keyword: Token,
) -> Result<Option<Stmt>, ParseError> {
	expect_semicolon(tokens)?;

	Ok(Some(Stmt::Continue(ContinueValue { keyword })))
}
