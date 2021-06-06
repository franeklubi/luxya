use super::{
	env::InterpreterEnvironment,
	expressions::{
		assignment_expression,
		binary_experssion,
		call_expression,
		function_expression,
		get_expression,
		identifier_expression,
		literal_expression,
		object_expression,
		set_expression,
		super_expression,
		this_expression,
		unary_expression,
	},
	native_functions,
	statements::{
		block_statement,
		break_statement,
		class_statement,
		continue_statement,
		declaration_statement,
		expression_statement,
		for_statement,
		if_statement,
		print_statement,
		return_statement,
	},
	types::{InterpreterValue, RuntimeError, StmtResult},
};
use crate::{
	ast::{expr::Expr, stmt::Stmt},
	env::EnvironmentWrapper,
};


pub fn interpret(statements: &[Stmt]) -> Result<(), RuntimeError> {
	let env = InterpreterEnvironment::new();

	native_functions::declare(&env);

	match eval_statements(statements, &env)? {
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

pub fn eval_statements(
	statements: &[Stmt],
	env: &InterpreterEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	for stmt in statements {
		let res = eval_statement(stmt, env)?;

		if !matches!(res, StmtResult::Noop) {
			return Ok(res);
		}
	}

	Ok(StmtResult::Noop)
}

fn eval_statement(
	stmt: &Stmt,
	env: &InterpreterEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	match stmt {
		Stmt::Expression(v) => expression_statement(eval_expression, v, env),
		Stmt::Print(v) => print_statement(eval_expression, v, env),
		Stmt::Declaration(v) => declaration_statement(eval_expression, v, env),
		Stmt::Block(v) => block_statement(eval_statements, v, env),
		Stmt::If(v) => if_statement(eval_expression, eval_statement, v, env),
		Stmt::For(v) => for_statement(eval_expression, eval_statement, v, env),
		Stmt::Return(v) => return_statement(eval_expression, v, env),
		Stmt::Break(v) => Ok(break_statement(v)),
		Stmt::Continue(v) => Ok(continue_statement(v)),
		Stmt::Class(v) => class_statement(v, env),
	}
}

pub fn eval_expression(
	expr: &Expr,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Literal(v) => literal_expression(v, env),
		Expr::Grouping(v) => eval_expression(&v.expression, env),
		Expr::Unary(v) => unary_expression(v, env),
		Expr::Binary(v) => binary_experssion(v, env),
		Expr::Identifier(v) => identifier_expression(v, env),
		Expr::Assignment(v) => assignment_expression(eval_expression, v, env),
		Expr::Call(v) => call_expression(v, env),
		Expr::Function(v) => Ok(function_expression(v, env)),
		Expr::Get(v) => get_expression(v, env),
		Expr::Set(v) => set_expression(v, env),
		Expr::This(v) => this_expression(v, env),
		Expr::Super(v) => super_expression(v, env),
		Expr::Object(v) => object_expression(v, env),
	}
}
