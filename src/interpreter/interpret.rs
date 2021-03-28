use super::{
	expressions::*,
	interpreter_env::*,
	native_functions::declare_native_functions,
	statements::*,
	types::*,
};
use crate::{
	ast::{expr::*, stmt::*},
	env::*,
};


pub fn interpret(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let env = InterpreterEnvironment::new();

	declare_native_functions(&env);

	match eval_statements(statements, &env)? {
		InterpreterStmtValue::Noop => Ok(()),
		InterpreterStmtValue::Break(token) => Err(RuntimeError {
			message: "Cannot use `break` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Continue(token) => Err(RuntimeError {
			message: "Cannot use `continue` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Return { keyword, .. } => Err(RuntimeError {
			message: "Cannot use `return` outside of a function".into(),
			token: keyword,
		}),
	}
}

pub fn eval_statements(
	statements: &[Stmt],
	env: &InterpreterEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	for stmt in statements {
		let e = eval_statement(&stmt, env)?;

		if !matches!(e, InterpreterStmtValue::Noop) {
			return Ok(e);
		}
	}

	Ok(InterpreterStmtValue::Noop)
}

fn eval_statement(
	stmt: &Stmt,
	env: &InterpreterEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	match stmt {
		Stmt::Expression(v) => expression_statement(eval_expression, v, env),
		Stmt::Print(v) => print_statement(eval_expression, v, env),
		Stmt::Declaration(v) => declaration_statement(eval_expression, v, env),
		Stmt::Block(v) => block_statement(eval_statements, v, env),
		Stmt::If(v) => if_statement(eval_expression, eval_statement, v, env),
		Stmt::For(v) => for_statement(eval_expression, eval_statement, v, env),
		Stmt::Return(v) => return_statement(eval_expression, v, env),
		Stmt::Break(v) => break_statement(v),
		Stmt::Continue(v) => continue_statement(v),
	}
}

pub fn eval_expression(
	expr: &Expr,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Literal(v) => literal_expression(v),
		Expr::Grouping(v) => eval_expression(&v.expression, env),
		Expr::Unary(v) => unary_expression(v, env),
		Expr::Binary(v) => binary_experssion(v, env),
		Expr::Identifier(v) => identifier_expression(v, env),
		Expr::Assignment(v) => assignment_expression(eval_expression, v, env),
		Expr::Call(v) => call_expression(v, env),
		Expr::Function(v) => function_expression(v, env),
	}
}
