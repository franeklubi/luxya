use crate::{
	ast::stmt::*,
	env::*,
	interpreter::{
		self,
		types::{InterpreterStmtValue, RuntimeError},
	},
};

use super::resolver_env::*;


pub fn resolve(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let scope = ResolverEnvironment::new();

	resolve_statements(statements, &scope)?;

	Ok(())
}


pub fn resolve_statements(
	statements: &[Stmt],
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<bool>, RuntimeError> {
	for stmt in statements {
		resolve_statement(&stmt, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

fn resolve_statement(
	stmt: &Stmt,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<bool>, RuntimeError> {
	match stmt {
		Stmt::Block(v) => interpreter::statements::block_statement(resolve_statements, v, env),
		_ => Ok(InterpreterStmtValue::Noop)
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
