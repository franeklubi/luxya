use super::{helpers::*, statements::*, types::*};
use crate::{
	ast::{expr::*, stmt::*},
	token::*,
};

use std::rc::Rc;


pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<ParseError>) {
	let tokens: ParserIter = &mut tokens.into_iter().peekable();

	let mut statements = Vec::new();
	let mut errors = Vec::new();

	while let Some(token) = tokens.peek() {
		if token.token_type == TokenType::Eof {
			break;
		}

		match declaration(tokens) {
			Ok(Some(s)) => statements.push(s),

			Err(s) => {
				synchronize(tokens);

				errors.push(s);
			}
			_ => (),
		}
	}

	(statements, errors)
}

// grammar functions down there 👇

pub fn declaration(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	fn value_declaration(
		tokens: ParserIter,
		matched: TokenType,
	) -> Result<Option<Stmt>, ParseError> {
		// TODO: optimize expect
		let token = expect(
			tokens,
			&[TokenType::Identifier("".into())],
			Some("Expected identifier"),
		)?;

		let initializer =
			if match_then_consume(tokens, &[TokenType::Equal]).is_some() {
				Some(Box::new(expression(tokens)?))
			} else {
				None
			};

		expect_semicolon(tokens)?;

		Ok(Some(Stmt::Declaration(DeclarationValue {
			name: token,
			initializer,
			mutable: TokenType::Let == matched,
		})))
	}

	if let Some(token) =
		match_then_consume(tokens, &[TokenType::Let, TokenType::Const])
	{
		value_declaration(tokens, token.token_type)
	} else {
		statement(tokens)
	}
}

/// Statement can not fail and produce None for a statement, because it wouldn't
/// be significant (e.g. lone `;`)
pub fn statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	let consumed_token = match_then_consume(
		tokens,
		&[
			TokenType::If,
			TokenType::For,
			TokenType::Print,
			TokenType::While,
			TokenType::Break,
			TokenType::Return,
			TokenType::Continue,
			TokenType::LeftBrace,
			TokenType::Semicolon,
		],
	);

	let token_type = consumed_token.clone().map(|ct| ct.token_type);

	match token_type {
		Some(TokenType::If) => if_statement(tokens),
		Some(TokenType::For) => for_statement(tokens),
		Some(TokenType::Print) => print_statement(tokens),
		Some(TokenType::While) => while_statement(tokens),
		Some(TokenType::LeftBrace) => block_statement(tokens),
		Some(TokenType::Break) => {
			break_statement(tokens, consumed_token.unwrap())
		}
		Some(TokenType::Return) => {
			return_statement(tokens, consumed_token.unwrap())
		}
		Some(TokenType::Continue) => {
			continue_statement(tokens, consumed_token.unwrap())
		}

		// that's an empty statement so we ignore it later
		Some(TokenType::Semicolon) => Ok(None),

		_ => expression_statement(tokens),
	}
}

pub fn expression(tokens: ParserIter) -> Result<Expr, ParseError> {
	assignment(tokens)
}

fn assignment(tokens: ParserIter) -> Result<Expr, ParseError> {
	let expr = logic_or(tokens)?;

	if let Some(equals) = match_then_consume(tokens, &[TokenType::Equal]) {
		return if let Expr::Identifier(i) = expr {
			Ok(Expr::Assignment(AssignmentValue {
				name: i.name,
				value: Box::new(assignment(tokens)?),
			}))
		} else {
			Err(ParseError {
				token: Some(equals),
				message: format!(
					"Invalid l-value. Cannot assign to {}",
					expr.to_human_readable()
				),
			})
		};
	}

	Ok(expr)
}

fn logic_or(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, logic_and, &[TokenType::Or])
}

fn logic_and(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, equality, &[TokenType::And])
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

// TODO: optimize unary
fn unary(tokens: ParserIter) -> Result<Expr, ParseError> {
	if let Some(operator) = tokens.peek() {
		if !match_token_type(
			&operator.token_type,
			&[TokenType::Bang, TokenType::Minus],
		) {
			return function_declaration(tokens);
		}

		let operator = tokens.next().unwrap();

		let right = unary(tokens)?;

		return Ok(Expr::Unary(UnaryValue {
			operator,
			right: Box::new(right),
		}));
	}

	function_declaration(tokens)
}

fn function_declaration(tokens: ParserIter) -> Result<Expr, ParseError> {
	if let Some(keyword) = match_then_consume(tokens, &[TokenType::Fun]) {
		// TODO: optimize expect
		let fake_identifier = TokenType::Identifier("".into());

		// TODO: optimize expect
		let name = match_then_consume(tokens, &[fake_identifier.clone()]);

		expect(tokens, &[TokenType::LeftParen], None)?;

		let mut params = Vec::new();

		if !peek_matches(tokens, &[TokenType::RightParen]) {
			loop {
				params.push(expect(tokens, &[fake_identifier.clone()], None)?);

				if match_then_consume(tokens, &[TokenType::Comma]).is_none() {
					break;
				}
			}
		}

		expect(tokens, &[TokenType::RightParen], None)?;

		expect(tokens, &[TokenType::LeftBrace], None)?;

		let body = block_statement(tokens)?;

		let statements = if let Some(Stmt::Block(bv)) = body {
			Some(Rc::new(bv.statements))
		} else {
			None
		};

		Ok(Expr::Function(FunctionValue {
			body: statements,
			keyword,
			name,
			params: if params.is_empty() {
				None
			} else {
				Some(params)
			},
		}))
	} else {
		call(tokens)
	}
}

fn call(tokens: ParserIter) -> Result<Expr, ParseError> {
	fn finish_call(
		tokens: ParserIter,
		calee: Expr,
	) -> Result<Expr, ParseError> {
		let mut arguments = Vec::new();

		if !peek_matches(tokens, &[TokenType::RightParen]) {
			loop {
				arguments.push(expression(tokens)?);

				if match_then_consume(tokens, &[TokenType::Comma]).is_none() {
					break;
				}
			}
		}

		Ok(Expr::Call(CallValue {
			arguments,
			calee: Box::new(calee),
			closing_paren: expect(tokens, &[TokenType::RightParen], None)?,
		}))
	}

	let mut expr = primary(tokens)?;

	while match_then_consume(tokens, &[TokenType::LeftParen]).is_some() {
		expr = finish_call(tokens, expr)?;
	}

	Ok(expr)
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

			expect(tokens, &[TokenType::RightParen], None)?;

			Ok(Expr::Grouping(GroupingValue {
				expression: Box::new(expr),
			}))
		}

		Some(Token {
			token_type: TokenType::Identifier(_),
			..
		}) => Ok(Expr::Identifier(IdentifierValue {
			name: token.unwrap(),
		})),

		_ => Err(ParseError {
			token,
			message: "Expected expression".into(),
		}),
	}
}
