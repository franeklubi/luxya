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

pub fn while_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let condition = if peek_matches(tokens, &[TokenType::LeftBrace]) {
		None
	} else {
		Some(Box::new(expression(tokens)?))
	};

	let execute = Box::new(expect_statement(tokens, &[TokenType::LeftBrace])?);

	Ok(Some(Stmt::While(WhileValue { condition, execute })))
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
	let initializer = match tokens.peek().unwrap().token_type {
		TokenType::Let | TokenType::Const => declaration(tokens)?,
		_ => {
			tokens.next();
			None
		}
	};

	// parse condition
	let condition =
		if match_then_consume(tokens, &[TokenType::Semicolon]).is_some() {
			None
		} else {
			let expr = expression(tokens)?;

			expect(tokens, &[TokenType::Semicolon], None)?;

			Some(Box::new(expr))
		};

	// parse increment
	let increment = if peek_matches(tokens, &[TokenType::LeftBrace]) {
		None
	} else {
		Some(expression(tokens)?)
	};

	// parse while body
	let mut while_body = expect_statement(tokens, &[TokenType::LeftBrace])?;

	// if increment is present, push it into the while body
	if let Some(increment) = increment {
		let bv = if let Stmt::Block(bv) = &mut while_body {
			bv
		} else {
			unreachable!()
		};

		bv.statements.push(Stmt::Expression(ExpressionValue {
			expression: Box::new(increment),
		}));
	}

	let while_stmt = Stmt::While(WhileValue {
		condition,
		execute: Box::new(while_body),
	});

	// determine if for body requires to be in a separate block
	let for_body = if let Some(initializer) = initializer {
		Stmt::Block(BlockValue {
			statements: vec![initializer, while_stmt],
		})
	} else {
		while_stmt
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
