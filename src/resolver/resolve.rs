use super::{resolver_env::*, statements::*};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
	interpreter::{
		self,
		types::{InterpreterStmtValue, InterpreterValue, RuntimeError},
	},
};


pub fn resolve(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let scope = ResolverEnvironment::new();

	resolve_statements(statements, &scope)?;

	Ok(())
}

fn resolve_statements(
	statements: &[Stmt],
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	for stmt in statements {
		// CHECK IF RETURN - THE SAME AS IN INTERPRETER
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
			interpreter::statements::block_statement(resolve_statements, v, env)
		}
		Stmt::Expression(v) => interpreter::statements::expression_statement(
			eval_expression,
			v,
			env,
		),
		Stmt::Break(v) => interpreter::statements::break_statement(v),
		Stmt::Continue(v) => interpreter::statements::continue_statement(v),
		Stmt::Return(v) => {
			interpreter::statements::return_statement(eval_expression, v, env)
		}

		// custom resolver handlers
		Stmt::Print(v) => {
			eval_expression(&v.expression, env)?;

			Ok(InterpreterStmtValue::Noop)
		}
		Stmt::Declaration(v) => declaration_statement(v, env),
		// Stmt::If(v) => interpreter::statements::if_statement(env, v),
		// Stmt::For(v) => interpreter::statements::for_statement(env, v),
		_ => unimplemented!("statement"),
	}
}

pub fn eval_expression(
	expr: &Expr,
	_env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		// Expr::Literal(v) => literal_expression(v),
		// Expr::Grouping(v) => eval_expression(&v.expression, env),
		// Expr::Unary(v) => unary_expression(v, env),
		// Expr::Binary(v) => binary_experssion(v, env),
		// Expr::Identifier(v) => identifier_expression(v, env),
		// Expr::Assignment(v) => assignment_expression(eval_expression, v, env),
		// Expr::Call(v) => call_expression(v, env),
		// Expr::Function(v) => function_expression(v, env),
		_ => unimplemented!("expression"),
	}
}
