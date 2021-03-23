use crate::{ast::stmt::*, interpreter::types::RuntimeError};

use super::{env::*, statements::*};


pub fn resolve(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let scope = ResolverEnvironment::new();

	resolve_statements(statements, &scope)
}


pub fn resolve_statements(
	statements: &[Stmt],
	env: &ResolverEnvironment,
) -> Result<(), RuntimeError> {
	for stmt in statements {
		resolve_statement(&stmt, env)?
	}

	Ok(())
}

fn resolve_statement(
	stmt: &Stmt,
	env: &ResolverEnvironment,
) -> Result<(), RuntimeError> {
	match stmt {
		Stmt::Block(v) => block_statement(env, v),
		_ => Ok(())
		// Stmt::Expression(v) => expression_statement(env, v),
		// Stmt::Print(v) => print_statement(env, v),
		// Stmt::Declaration(v) => declaration_statement(env, v),
		// Stmt::If(v) => if_statement(env, v),
		// Stmt::For(v) => for_statement(env, v),
		// Stmt::Return(v) => return_statement(env, v),
		// Stmt::Break(v) => break_statement(env, v),
		// Stmt::Continue(v) => continue_statement(env, v),
	}
}
