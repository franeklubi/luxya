use super::{expressions::*, resolver_env::*, statements::*};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
	interpreter::{
		expressions as interpreter_exprs,
		statements as interpreter_stmts,
		types::{InterpreterStmtValue, InterpreterValue, RuntimeError},
	},
};


pub fn resolve(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let scope = ResolverEnvironment::new();

	// CHECK IF RETURN AND ETC - THE SAME AS IN INTERPRETER
	resolve_statements(statements, &scope)?;

	Ok(())
}

pub fn resolve_statements(
	statements: &[Stmt],
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	for stmt in statements {
		// ONLY LET NOOP PASS THROUGH HERE
		resolve_statement(&stmt, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

fn resolve_statement(
	stmt: &Stmt,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	match stmt {
		Stmt::Block(v) => {
			interpreter_stmts::block_statement(resolve_statements, v, env)
		}
		Stmt::Expression(v) => {
			interpreter_stmts::expression_statement(resolve_expression, v, env)
		}
		Stmt::Break(v) => interpreter_stmts::break_statement(v),
		Stmt::Continue(v) => interpreter_stmts::continue_statement(v),
		Stmt::Return(v) => {
			interpreter_stmts::return_statement(resolve_expression, v, env)
		}

		// custom resolver handlers
		Stmt::Print(v) => print_statement(v, env),
		Stmt::Declaration(v) => declaration_statement(v, env),
		// Stmt::If(v) => interpreter_stmts::if_statement(env, v),
		// Stmt::For(v) => interpreter_stmts::for_statement(env, v),
		_ => unimplemented!("resolve statement"),
	}
}

pub fn resolve_expression(
	expr: &Expr,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Literal(v) => interpreter_exprs::literal_expression(v),
		Expr::Identifier(v) => identifier_expression(expr, v, env),
		Expr::Assignment(v) => assignment_expression(v, env),
		Expr::Function(v) => function_expression(v, env),
		Expr::Grouping(_v) => unimplemented!("Grouping"),
		Expr::Unary(_v) => unimplemented!("Unary"),
		Expr::Binary(_v) => unimplemented!("Binary"),
		Expr::Call(_v) => unimplemented!("Call"),
	}
	// Expr::Grouping(v) => resolve_expression(&v.expression, env),
	// Expr::Unary(v) => unary_expression(v, env),
	// Expr::Binary(v) => binary_experssion(v, env),
	// Expr::Identifier(v) => identifier_expression(v, env),
	// Expr::Call(v) => call_expression(v, env),
}
