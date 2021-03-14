use crate::ast::{expr::*, stmt::*};
use crate::token::*;

use std::{iter, vec};


type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub struct ParseError {
	pub token: Option<Token>,
	pub message: String,
}

impl Expr {
	fn to_human_readable(&self) -> &str {
		match self {
			Expr::Assignment(_) => "an assignment",
			Expr::Binary(_) => "a binary expression",
			Expr::Grouping(_) => "a grouping",
			Expr::Literal(_) => "a literal",
			Expr::Unary(_) => "a unary expression",
			Expr::Identifier(_) => "an identifier",
			Expr::Call(_) => "a function/method call",
			Expr::Function(_) => "a function/method declaration",
		}
	}
}

fn match_token_type(t: &TokenType, expected: &[TokenType]) -> bool {
	expected.iter().any(|a| a == t)
}

fn peek_matches(tokens: ParserIter, expected: &[TokenType]) -> bool {
	tokens
		.peek()
		.map_or(false, |v| match_token_type(&v.token_type, expected))
}

/// Tries peek of ParserIter against provided token types
///
/// Returns `Some(Token)` if successful and consumes the token, `None` otherwise
fn match_then_consume(
	tokens: ParserIter,
	expected: &[TokenType],
) -> Option<Token> {
	tokens
		.peek()
		.map(|t| match_token_type(&t.token_type, expected))
		.and_then(|b| b.then(|| tokens.next().unwrap()))
}

fn expect(
	tokens: ParserIter,
	expected: &[TokenType],
	override_message: Option<&str>,
) -> Result<Token, ParseError> {
	match_then_consume(tokens, expected).ok_or_else(|| {
		let message = if let Some(m) = override_message {
			m.to_string()
		} else {
			gen_expected_msg(expected)
		};

		ParseError {
			message,
			token: tokens.peek().cloned(),
		}
	})
}

fn gen_expected_msg(expected: &[TokenType]) -> String {
	let msg = if expected.len() > 1 {
		"Expected one of: "
	} else {
		"Expected: "
	}
	.to_string();

	let enumerated: String = expected
		.iter()
		.map(|e| format!("`{}`", e))
		.collect::<Vec<String>>()
		.join(", ");

	msg + &enumerated
}

fn expect_semicolon(tokens: ParserIter) -> Result<Token, ParseError> {
	expect(tokens, &[TokenType::Semicolon], None)
}

fn build_binary_expr(
	tokens: ParserIter,
	lower_precedence: impl Fn(ParserIter) -> Result<Expr, ParseError>,
	types_to_match: &[TokenType],
) -> Result<Expr, ParseError> {
	let mut expr = lower_precedence(tokens)?;

	while let Some(operator) = match_then_consume(tokens, types_to_match) {
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
// TODO: rethink/rewrite this
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
				if TokenType::Semicolon == tokens.next().unwrap().token_type {
					break;
				}
			}
		}
	}
}

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

fn declaration(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
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

fn block_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
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

/// Statement can not fail and produce None for a statement, because it wouldn't
/// be significant (e.g. lone `;`)
fn statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
	fn unwrap_statement(
		tokens: ParserIter,
		stmt: Option<Stmt>,
		expected: &[TokenType],
		override_message: Option<&str>,
	) -> Result<Stmt, ParseError> {
		stmt.ok_or_else(|| ParseError {
			message: if let Some(msg) = override_message {
				msg.into()
			} else if expected.is_empty() {
				"Expected statement".into()
			} else {
				gen_expected_msg(expected)
			},
			token: tokens.peek().cloned(),
		})
	}

	fn expect_statement(
		tokens: ParserIter,
		starts_with: &[TokenType],
	) -> Result<Stmt, ParseError> {
		if !peek_matches(tokens, starts_with) {
			unwrap_statement(tokens, None, starts_with, None)
		} else {
			let stmt = statement(tokens)?;

			unwrap_statement(tokens, stmt, starts_with, None)
		}
	}

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

	fn if_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
		let condition = Box::new(expression(tokens)?);

		let then = Box::new(expect_statement(
			tokens,
			&[TokenType::LeftBrace, TokenType::If],
		)?);
		let otherwise =
			if match_then_consume(tokens, &[TokenType::Else]).is_some() {
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

	fn while_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
		let condition = if peek_matches(tokens, &[TokenType::LeftBrace]) {
			None
		} else {
			Some(Box::new(expression(tokens)?))
		};

		let execute =
			Box::new(expect_statement(tokens, &[TokenType::LeftBrace])?);

		Ok(Some(Stmt::While(WhileValue { condition, execute })))
	}

	fn for_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
		let expected =
			&[TokenType::Semicolon, TokenType::Let, TokenType::Const];

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

	let consumed_token = match_then_consume(
		tokens,
		&[
			TokenType::If,
			TokenType::For,
			TokenType::Print,
			TokenType::While,
			TokenType::LeftBrace,
			TokenType::Semicolon,
		],
	);

	let token_type = consumed_token.map(|ct| ct.token_type);

	match token_type {
		Some(TokenType::If) => if_statement(tokens),
		Some(TokenType::For) => for_statement(tokens),
		Some(TokenType::Print) => print_statement(tokens),
		Some(TokenType::While) => while_statement(tokens),
		Some(TokenType::LeftBrace) => block_statement(tokens),

		// that's an empty statement so we ignore it later
		Some(TokenType::Semicolon) => Ok(None),
		_ => expression_statement(tokens),
	}
}

fn expression(tokens: ParserIter) -> Result<Expr, ParseError> {
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

		Ok(Expr::Function(FunctionValue {
			body: body.map(Box::new),
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
