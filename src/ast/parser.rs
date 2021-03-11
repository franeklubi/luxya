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
			Expr::Assignment(_) => "An assignment",
			Expr::Binary(_) => "A binary expression",
			Expr::Grouping(_) => "A grouping",
			Expr::Literal(_) => "A literal",
			Expr::Unary(_) => "A unary expression",
			Expr::Identifier(_) => "An identifier",
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
			let msg = if expected.len() > 1 {
				"Expected one of: "
			} else {
				"Expected: "
			};

			expected
				.iter()
				.fold(msg.to_string(), |acc, tt| acc + &format!("`{}`", tt))
		};

		ParseError {
			message,
			token: tokens.peek().cloned(),
		}
	})
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

/// Statement can not fail and produce None for a statement, because it wouldn't
/// be significant (e.g. lone `;`)
fn statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
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
		let stmt = Stmt::Expression(ExpressionValue {
			expression: Box::new(expression(tokens)?),
		});

		expect_semicolon(tokens)?;

		Ok(Some(stmt))
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

	fn consume_block(tokens: ParserIter) -> Result<Stmt, ParseError> {
		fn gen_err(tokens: ParserIter) -> ParseError {
			ParseError {
				message: "If statement's branch has to be a block (`{ ... }`)"
					.to_owned(),
				token: tokens.peek().cloned(),
			}
		}

		if !peek_matches(tokens, &[TokenType::LeftBrace]) {
			Err(gen_err(tokens))
		} else {
			let stmt = statement(tokens)?;

			if let Some(stmt) = stmt {
				Ok(stmt)
			} else {
				Err(gen_err(tokens))
			}
		}
	}

	fn if_statement(tokens: ParserIter) -> Result<Option<Stmt>, ParseError> {
		let condition = Box::new(expression(tokens)?);

		let then = Box::new(consume_block(tokens)?);
		let otherwise =
			if match_then_consume(tokens, &[TokenType::Else]).is_some() {
				Some(Box::new(consume_block(tokens)?))
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
		let condition = Box::new(expression(tokens)?);

		let execute = Box::new(consume_block(tokens)?);

		Ok(Some(Stmt::While(WhileValue { condition, execute })))
	}

	let consumed_token = match_then_consume(
		tokens,
		&[
			TokenType::Print,
			TokenType::Semicolon,
			TokenType::LeftBrace,
			TokenType::If,
			TokenType::While,
		],
	);

	let token_type = consumed_token.map(|ct| ct.token_type);

	match token_type {
		// that's an empty statement of sorts 🤔
		Some(TokenType::Semicolon) => Ok(None),
		Some(TokenType::Print) => print_statement(tokens),
		Some(TokenType::LeftBrace) => block_statement(tokens),
		Some(TokenType::If) => if_statement(tokens),
		Some(TokenType::While) => while_statement(tokens),
		_ => expression_statement(tokens),
	}
}

fn expression(tokens: ParserIter) -> Result<Expr, ParseError> {
	assignment(tokens)
}

fn assignment(tokens: ParserIter) -> Result<Expr, ParseError> {
	let expr = equality(tokens)?;

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
					expr.to_human_readable().to_lowercase()
				),
			})
		};
	}

	Ok(expr)
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

fn unary(tokens: ParserIter) -> Result<Expr, ParseError> {
	if let Some(operator) = tokens.peek() {
		if !match_token_type(
			&operator.token_type,
			&[TokenType::Bang, TokenType::Minus],
		) {
			return primary(tokens);
		}

		let operator = tokens.next().unwrap();

		let right = unary(tokens)?;

		return Ok(Expr::Unary(UnaryValue {
			operator,
			right: Box::new(right),
		}));
	}

	primary(tokens)
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
