use super::{
	env::ResolverEnvironment,
	expressions::{
		assignment_expression,
		binary_expression,
		call_expression,
		function_expression,
		get_expression,
		identifier_expression,
		object_expression,
		set_expression,
		super_expression,
		this_expression,
	},
	statements::{
		class_statement,
		declaration_statement,
		for_statement,
		if_statement,
		print_statement,
	},
};
use crate::{
	ast::{expr::Expr, stmt::Stmt},
	env::EnvironmentWrapper,
	interpreter::{
		native_functions::NATIVE_FUNCTION_NAMES,
		statements as interpreter_stmts,
		types::{InterpreterValue, RuntimeError, StmtResult},
	},
	unwrap_scope_mut,
};


pub fn resolve(stmts: &[Stmt]) -> Result<(), RuntimeError> {
	let scope = ResolverEnvironment::new();

	// Declaring native functions
	{
		let scope_map = unwrap_scope_mut!(scope);

		for k in &NATIVE_FUNCTION_NAMES {
			scope_map.insert((*k).to_string(), true);
		}
	}

	match statements(stmts, &scope)? {
		StmtResult::Noop => Ok(()),
		StmtResult::Break(token) => Err(RuntimeError {
			message: "Cannot use `break` outside of a loop".into(),
			token,
		}),
		StmtResult::Continue(token) => Err(RuntimeError {
			message: "Cannot use `continue` outside of a loop".into(),
			token,
		}),
		StmtResult::Return { keyword, .. } => Err(RuntimeError {
			message: "Cannot use `return` outside of a function".into(),
			token: keyword,
		}),
	}
}

pub fn statements(
	statements: &[Stmt],
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	for stmt in statements {
		let res = statement(stmt, env)?;

		if !matches!(res, StmtResult::Noop) {
			return Ok(res);
		}
	}

	Ok(StmtResult::Noop)
}

pub fn statement(
	stmt: &Stmt,
	env: &ResolverEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	match stmt {
		Stmt::Block(v) => {
			interpreter_stmts::block_statement(statements, v, env)
		}
		Stmt::Expression(v) => {
			interpreter_stmts::expression_statement(expression, v, env)
		}
		Stmt::Break(v) => Ok(interpreter_stmts::break_statement(v)),
		Stmt::Continue(v) => Ok(interpreter_stmts::continue_statement(v)),
		Stmt::Return(v) => {
			interpreter_stmts::return_statement(expression, v, env)
		}

		// custom resolver statement handlers
		Stmt::Print(v) => print_statement(v, env),
		Stmt::Declaration(v) => declaration_statement(v, env),
		Stmt::If(v) => if_statement(v, env),
		Stmt::For(v) => for_statement(v, env),
		Stmt::Class(v) => class_statement(v, env),
	}
}

pub fn expression(
	expr: &Expr,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Grouping(v) => expression(&v.expression, env),
		Expr::Literal(_v) => Ok(InterpreterValue::Nil),

		// custom resolver expression handlers
		Expr::Identifier(v) => identifier_expression(expr, v, env),
		Expr::Assignment(v) => assignment_expression(expr, v, env),
		Expr::Unary(v) => expression(&v.right, env),
		Expr::Function(v) => function_expression(v, env),
		Expr::Super(v) => super_expression(expr, v, env),
		Expr::This(v) => this_expression(expr, v, env),
		Expr::Binary(v) => binary_expression(v, env),
		Expr::Object(v) => object_expression(v, env),
		Expr::Call(v) => call_expression(v, env),
		Expr::Get(v) => get_expression(v, env),
		Expr::Set(v) => set_expression(v, env),
	}
}
