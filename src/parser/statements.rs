use super::{expressions::*, helpers::*, parse::*, types::*};
use crate::{
	ast::{expr::*, stmt::*},
	expect,
	expect_one,
	match_then_consume,
	match_then_consume_stmt,
	peek_matches,
	token::*,
};

use std::vec;


#[inline(always)]
pub fn print_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let stmt = Stmt::Print(PrintValue {
		expression: expression(tokens)?,
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

	let stmt = Stmt::Expression(ExpressionValue { expression: expr });

	if semicolon_expected {
		expect_semicolon(tokens)?;
	}

	Ok(Some(stmt))
}

pub fn if_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let condition = expression(tokens)?;

	let then = match_then_consume_stmt!(
		tokens,
		TokenType::LeftBrace,
		"Expected a block expression for `then` branch"
	)?
	.map(Box::new);

	let otherwise = if match_then_consume!(tokens, TokenType::Else).is_some() {
		match_then_consume_stmt!(
			tokens,
			TokenType::LeftBrace | TokenType::If,
			"Expected"
		)?
		.map(Box::new)
	} else {
		None
	};

	if then.is_none() && otherwise.is_none() {
		Ok(None)
	} else {
		Ok(Some(Stmt::If(IfValue {
			condition,
			then,
			otherwise,
		})))
	}
}

pub fn block_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let mut statements = Vec::new();

	while !peek_matches!(tokens, TokenType::RightBrace) {
		if let Some(d) = declaration(tokens)? {
			statements.push(d);
		}
	}

	expect_one!(tokens, TokenType::RightBrace)?;

	// as though it may not seem as an optimization, it really is a useful
	// heuristic to return an empty statement rather than block
	// with 0 statements
	//
	// for example: I use this in `if` statements to determine if I need to
	// even return them or not
	if statements.is_empty() {
		Ok(None)
	} else {
		Ok(Some(Stmt::Block(BlockValue { statements })))
	}
}

pub fn for_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	// parse declaration
	if !peek_matches!(
		tokens,
		TokenType::Semicolon | TokenType::Let | TokenType::Const
	) {
		return Err(ParseError {
			message: "Expected `let`, `const`, or `;` to omit declaration"
				.to_owned(),
			token: tokens.peek().cloned(),
		});
	}

	// parse initializer
	let initializer =
		if peek_matches!(tokens, TokenType::Let | TokenType::Const) {
			declaration(tokens)?
		} else {
			tokens.next();

			None
		};

	// parse condition
	let condition =
		if match_then_consume!(tokens, TokenType::Semicolon).is_some() {
			None
		} else {
			let expr = expression(tokens)?;

			expect_one!(tokens, TokenType::Semicolon)?;

			Some(expr)
		};

	// parse closer (the increment or whatever)
	let closer = if peek_matches!(tokens, TokenType::LeftBrace) {
		None
	} else {
		Some(expression(tokens)?)
	};


	// parse for's body. If the body is None, then we may as well
	// short-circuit it there, and return Ok(None)
	let body_stmt = match_then_consume_stmt!(
		tokens,
		TokenType::LeftBrace,
		"Expected for's body"
	)?;

	let body = if let Some(body) = body_stmt {
		body
	} else {
		return Ok(None);
	};


	let for_stmt = Stmt::For(ForValue {
		condition,
		body: Box::new(body),
		closer: closer.map(|c| {
			Box::new(Stmt::Expression(ExpressionValue { expression: c }))
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
	let expression = if !peek_matches!(tokens, TokenType::Semicolon) {
		Some(expression(tokens)?)
	} else {
		None
	};

	expect_semicolon(tokens)?;

	Ok(Some(Stmt::Return(ReturnValue {
		keyword,
		expression,
	})))
}

#[inline(always)]
pub fn break_statement(
	tokens: ParserIter,
	keyword: Token,
) -> Result<Option<Stmt>, ParseError> {
	expect_semicolon(tokens)?;

	Ok(Some(Stmt::Break(BreakValue { keyword })))
}

#[inline(always)]
pub fn continue_statement(
	tokens: ParserIter,
	keyword: Token,
) -> Result<Option<Stmt>, ParseError> {
	expect_semicolon(tokens)?;

	Ok(Some(Stmt::Continue(ContinueValue { keyword })))
}

pub fn class_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let name =
		expect!(tokens, TokenType::Identifier(_), "Expected class name")?;

	let superclass =
		if match_then_consume!(tokens, TokenType::Extends).is_some() {
			let superclass_name = expect!(
				tokens,
				TokenType::Identifier(_),
				"Expected superclass name",
			)?;

			Some(Expr::Identifier(IdentifierValue {
				name: superclass_name,
				env_distance: Default::default(),
			}))
		} else {
			None
		};

	expect_one!(tokens, TokenType::LeftBrace)?;

	let mut methods = Vec::new();

	while !peek_matches!(tokens, TokenType::RightBrace) {
		methods.push(function_declaration(tokens, true)?);
	}

	expect_one!(tokens, TokenType::RightBrace)?;

	Ok(Some(Stmt::Class(ClassValue {
		name,
		methods,
		superclass,
	})))
}
