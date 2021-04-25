use super::{expressions::*, resolver_env::*, statements::*};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
	interpreter::{
		native_functions::NATIVE_FUNCTION_NAMES,
		statements as interpreter_stmts,
		types::{InterpreterStmtValue, InterpreterValue, RuntimeError},
	},
	resolver_unwrap_scope_mut,
};


pub fn resolve(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let scope = ResolverEnvironment::new();

	{
		let scope_map = resolver_unwrap_scope_mut!(scope);

		NATIVE_FUNCTION_NAMES.iter().for_each(|k| {
			scope_map.insert(k.to_string(), true);
		});
	}

	// TODO: CHECK IF RETURN AND ETC - THE SAME AS IN INTERPRETER
	resolve_statements(statements, &scope)?;

	Ok(())
}

pub fn resolve_statements(
	statements: &[Stmt],
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	for stmt in statements {
		// TODO: ONLY LET NOOP PASS THROUGH HERE
		resolve_statement(&stmt, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

pub fn resolve_statement(
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

		// custom resolver statement handlers
		Stmt::Print(v) => print_statement(v, env),
		Stmt::Declaration(v) => declaration_statement(v, env),
		Stmt::If(v) => if_statement(v, env),
		Stmt::For(v) => for_statement(v, env),
		Stmt::Class(v) => class_statement(v, env),
	}
}

pub fn resolve_expression(
	expr: &Expr,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Grouping(v) => resolve_expression(&v.expression, env),
		Expr::Literal(_v) => Ok(InterpreterValue::Nil),

		// custom resolver expression handlers
		Expr::Identifier(v) => identifier_expression(expr, v, env),
		Expr::Assignment(v) => assignment_expression(expr, v, env),
		Expr::Unary(v) => resolve_expression(&v.right, env),
		Expr::Function(v) => function_expression(v, env),
		Expr::This(v) => this_expression(expr, v, env),
		Expr::Binary(v) => binary_expression(v, env),
		Expr::Call(v) => call_expression(v, env),
		Expr::Get(v) => get_expression(v, env),
		Expr::Set(v) => set_expression(v, env),
		Expr::Super(_v) => unimplemented!("super resolver"),
	}
}
