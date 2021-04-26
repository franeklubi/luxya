use super::{helpers::*, statements::*, types::*};
use crate::{
	ast::{expr::*, stmt::*},
	token::*,
};

use std::{cell::Cell, rc::Rc};


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
	#[inline(always)]
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
				Some(expression(tokens)?)
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
			TokenType::Break,
			TokenType::Class,
			TokenType::Return,
			TokenType::Continue,
			TokenType::LeftBrace,
			TokenType::Semicolon,
		],
	);

	let token_type = consumed_token.as_ref().map(|ct| &ct.token_type);

	match token_type {
		Some(TokenType::If) => if_statement(tokens),
		Some(TokenType::For) => for_statement(tokens),
		Some(TokenType::Print) => print_statement(tokens),
		Some(TokenType::Class) => class_statement(tokens),
		Some(TokenType::LeftBrace) => block_statement(tokens),
		Some(TokenType::Break) => unsafe {
			break_statement(tokens, consumed_token.unwrap_unchecked())
		},
		Some(TokenType::Return) => unsafe {
			return_statement(tokens, consumed_token.unwrap_unchecked())
		},
		Some(TokenType::Continue) => unsafe {
			continue_statement(tokens, consumed_token.unwrap_unchecked())
		},

		// We allow trails of semicolons and treat them as empty statements
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
		match expr {
			Expr::Identifier(v) => Ok(Expr::Assignment(AssignmentValue {
				name: v.name,
				value: Box::new(assignment(tokens)?),
				env_distance: Cell::new(0),
			})),
			Expr::Get(v) => Ok(Expr::Set(SetValue {
				setee: v.getee,
				key: v.key,
				blame: v.blame,
				value: Box::new(assignment(tokens)?),
			})),
			_ => Err(ParseError {
				token: Some(equals),
				message: format!(
					"Invalid l-value. Cannot assign to {}",
					expr.to_human_readable()
				),
			}),
		}
	} else {
		Ok(expr)
	}
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
			return function_declaration(tokens, false);
		}

		let operator = tokens.next().unwrap();

		let right = unary(tokens)?;

		return Ok(Expr::Unary(UnaryValue {
			operator,
			right: Box::new(right),
		}));
	}

	function_declaration(tokens, false)
}

pub fn function_declaration(
	tokens: ParserIter,
	method: bool,
) -> Result<Expr, ParseError> {
	// TODO: optimize function parsing
	if method || peek_matches(tokens, &[TokenType::Fun]) {
		// TODO: optimize expect
		let fake_identifier = TokenType::Identifier("".into());

		let keyword = if method {
			// TODO: optimize expect
			expect(
				tokens,
				&[fake_identifier.clone()],
				Some("Expected method name"),
			)?
		} else {
			expect(tokens, &[TokenType::Fun], None)?
		};

		// TODO: optimize expect
		let name = if method {
			Some(keyword.clone())
		} else {
			match_then_consume(tokens, &[fake_identifier.clone()])
		};

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
				Some(Rc::new(params))
			},
		}))
	} else {
		call(tokens)
	}
}

#[inline(always)]
fn finish_call(tokens: ParserIter, calee: Expr) -> Result<Expr, ParseError> {
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

fn finish_get(tokens: ParserIter, getee: Expr) -> Result<Expr, ParseError> {
	let consumed = tokens.next();
	let consumed_token_type = consumed.as_ref().map(|c| c.token_type.clone());

	match consumed_token_type {
		Some(TokenType::Identifier(i)) => {
			// unwrap_unchecked because we just matched peek 😇
			let blame = unsafe { consumed.unwrap_unchecked() };

			Ok(Expr::Get(GetValue {
				getee: Box::new(getee),
				key: GetAccessor::DotName(i),
				blame,
			}))
		}
		Some(TokenType::LeftParen) => {
			// same here, unwrapping what we already matched
			let blame = unsafe { tokens.peek().unwrap_unchecked().clone() };

			let eval = expression(tokens)?;

			expect(tokens, &[TokenType::RightParen], None)?;

			Ok(Expr::Get(GetValue {
				getee: Box::new(getee),
				key: GetAccessor::DotEval(Box::new(eval)),
				blame,
			}))
		}
		_ => Err(ParseError {
			token: tokens.peek().cloned(),
			message: "Expected identifier or a parenthesized expression to \
			          evaluate"
				.into(),
		}),
	}
}

fn finish_sub(tokens: ParserIter, getee: Expr) -> Result<Expr, ParseError> {
	let peek_type = tokens.peek().as_ref().map(|p| p.token_type.clone());

	let accessor = match peek_type {
		Some(TokenType::Number(n)) => {
			// unwrap_unchecked because we just matched peek 😇
			let blame = unsafe { tokens.next().unwrap_unchecked() };

			Ok(Expr::Get(GetValue {
				getee: Box::new(getee),
				key: GetAccessor::SubscriptionNumber(n),
				blame,
			}))
		}
		_ => {
			// same here, unwrapping what we already matched
			let blame = unsafe { tokens.peek().unwrap_unchecked().clone() };

			let eval = expression(tokens)?;

			Ok(Expr::Get(GetValue {
				getee: Box::new(getee),
				key: GetAccessor::SubscriptionEval(Box::new(eval)),
				blame,
			}))
		}
	}?;

	expect(tokens, &[TokenType::RightSquareBracket], None)?;

	Ok(accessor)
}

fn call(tokens: ParserIter) -> Result<Expr, ParseError> {
	let mut expr = primary(tokens)?;

	while let Some(consumed) = match_then_consume(
		tokens,
		&[
			TokenType::LeftParen,
			TokenType::Dot,
			TokenType::LeftSquareBracket,
		],
	) {
		match consumed.token_type {
			TokenType::LeftParen => {
				expr = finish_call(tokens, expr)?;
			}
			TokenType::LeftSquareBracket => {
				expr = finish_sub(tokens, expr)?;
			}
			_ => {
				expr = finish_get(tokens, expr)?;
			}
		}
	}

	Ok(expr)
}

// TODO: unwrap unsafe or idk dude. amend this
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
			token_type: TokenType::This,
			..
		}) => Ok(Expr::This(ThisValue {
			blame: token.unwrap(),
			env_distance: Cell::new(0),
		})),

		Some(Token {
			token_type: TokenType::Super,
			..
		}) => {
			let dummy_expr = Expr::Literal(LiteralValue::Nil);

			let accessor = match tokens.next().map(|next| next.token_type) {
				Some(TokenType::LeftParen) => {
					let call_expr = finish_call(tokens, dummy_expr)?;

					let arguments = if let Expr::Call(cv) = call_expr {
						cv.arguments
					} else {
						unreachable!("Call is not a call? Weird 😳")
					};

					Ok(SuperAccessor::Call(arguments))
				}
				Some(TokenType::Dot) => {
					// TODO: optimize expect
					let name = expect(
						tokens,
						&[TokenType::Identifier("".into())],
						Some("Expected method name"),
					)?;

					Ok(SuperAccessor::Method(name))
				}
				_ => Err(ParseError {
					token: token.clone(),
					message: "Expected `.` or `(...args)` (constructor call) \
					          after `super`"
						.into(),
				}),
			}?;

			Ok(Expr::Super(SuperValue {
				blame: token.unwrap(),
				accessor,
				env_distance: Cell::new(0),
			}))
		}

		Some(Token {
			token_type: TokenType::LeftSquareBracket,
			..
		}) => {
			let mut values = Vec::new();

			if !peek_matches(tokens, &[TokenType::RightSquareBracket]) {
				loop {
					values.push(expression(tokens)?);

					if match_then_consume(tokens, &[TokenType::Comma]).is_none()
					{
						break;
					}
				}
			}

			expect(tokens, &[TokenType::RightSquareBracket], None)?;

			Ok(Expr::Literal(LiteralValue::List(Rc::new(values))))
		}

		Some(Token {
			token_type: TokenType::Identifier(_),
			..
		}) => Ok(Expr::Identifier(IdentifierValue {
			name: token.unwrap(),
			env_distance: Cell::new(0),
		})),

		_ => Err(ParseError {
			token,
			message: "Expected expression".into(),
		}),
	}
}
