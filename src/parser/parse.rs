use super::{expression::expression, helpers::*, statements::*, types::*};
use crate::{ast::stmt::*, token::*};


pub fn parse(tokens: Vec<Token>) -> (Vec<Stmt>, Vec<ParseError>) {
	let tokens: ParserIter = &mut tokens.into_iter().peekable();

	let mut statements = Vec::new();
	let mut errors = Vec::new();

	while tokens.peek().is_some() {
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
