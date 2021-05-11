use super::{helpers::*, statements::*, types::*};
use crate::{
	ast::{expr::*, stmt::*},
	mtc,
	token::*,
};

use std::{cell::Cell, rc::Rc};


pub fn expression(tokens: ParserIter) -> Result<Expr, ParseError> {
	assignment(tokens)
}

fn assignment(tokens: ParserIter) -> Result<Expr, ParseError> {
	let expr = logic_or(tokens)?;

	if let Some(equals) = mtc!(tokens, TokenType::Equal) {
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
					expr.human_type()
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
	build_binary_expr(
		tokens,
		factor,
		&[TokenType::Minus, TokenType::Plus, TokenType::Modulo],
	)
}

fn factor(tokens: ParserIter) -> Result<Expr, ParseError> {
	build_binary_expr(tokens, unary, &[TokenType::Slash, TokenType::Star])
}

fn unary(tokens: ParserIter) -> Result<Expr, ParseError> {
	if matches!(
		tokens.peek().map(|t| &t.token_type),
		Some(TokenType::Bang) | Some(TokenType::Minus)
	) {
		let operator = tokens.next().unwrap();

		let right = unary(tokens)?;

		Ok(Expr::Unary(UnaryValue {
			operator,
			right: Box::new(right),
		}))
	} else {
		function_declaration(tokens, false)
	}
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

		let name = if method {
			Some(keyword.clone())
		} else {
			mtc!(tokens, TokenType::Identifier(_))
		};

		expect(tokens, &[TokenType::LeftParen], None)?;

		let mut params = Vec::new();

		while !peek_matches(tokens, &[TokenType::RightParen]) {
			params.push(expect(tokens, &[fake_identifier.clone()], None)?);

			if mtc!(tokens, TokenType::Comma).is_none() {
				break;
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

	while !peek_matches(tokens, &[TokenType::RightParen]) {
		arguments.push(expression(tokens)?);

		if mtc!(tokens, TokenType::Comma).is_none() {
			break;
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

	while let Some(consumed) = mtc!(
		tokens,
		TokenType::LeftParen | TokenType::Dot | TokenType::LeftSquareBracket
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

			while !peek_matches(tokens, &[TokenType::RightSquareBracket]) {
				values.push(expression(tokens)?);

				if mtc!(tokens, TokenType::Comma).is_none() {
					break;
				}
			}

			expect(tokens, &[TokenType::RightSquareBracket], None)?;

			Ok(Expr::Literal(LiteralValue::List(Rc::new(values))))
		}

		Some(Token {
			token_type: TokenType::Char(c),
			..
		}) => Ok(Expr::Literal(LiteralValue::Char(c))),

		Some(Token {
			token_type: TokenType::LeftBrace,
			..
		}) => {
			let mut properties: Vec<Property> = Vec::new();

			while !peek_matches(tokens, &[TokenType::RightBrace]) {
				// TODO: optimize expect
				let key_token = expect(
					tokens,
					&[
						TokenType::Identifier("".into()),
						TokenType::String("".into()),
					],
					Some("Expected property name"),
				)?;

				let key = match &key_token.token_type {
					TokenType::Identifier(s) | TokenType::String(s) => s,
					_ => unreachable!("Hi!! Welcome to my kitchen"),
				};

				let value = if mtc!(tokens, TokenType::Colon).is_some() {
					expression(tokens)?
				} else if let TokenType::Identifier(_) = key_token.token_type {
					Expr::Identifier(IdentifierValue {
						name: key_token.clone(),
						env_distance: Default::default(),
					})
				} else {
					return Err(ParseError {
						token: Some(key_token),
						message: "Cannot use short property declaration with \
						          string"
							.into(),
					});
				};

				properties.push(Property {
					key: key.clone(),
					value,
				});

				if mtc!(tokens, TokenType::Comma).is_none() {
					break;
				}
			}

			expect(tokens, &[TokenType::RightBrace], None)?;

			Ok(Expr::Object(ObjectValue {
				blame: token.unwrap(),
				properties,
			}))
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
