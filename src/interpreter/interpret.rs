use super::{
	env::*,
	expressions::*,
	native_functions::declare_native_functions,
	statements::*,
	types::*,
};
use crate::{
	ast::{expr::*, stmt::*},
	token::*,
};

use std::rc::Rc;


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
) -> Result<InterpreterStmtValue, RuntimeError> {
	for stmt in statements {
		let e = eval_statement(&stmt, env)?;

		if !matches!(e, InterpreterStmtValue::Noop) {
			return Ok(e);
		}
	}

	Ok(InterpreterStmtValue::Noop)
}

pub fn eval_statement(
	stmt: &Stmt,
	env: &InterpreterEnvironment,
) -> Result<InterpreterStmtValue, RuntimeError> {
	match stmt {
		Stmt::Expression(v) => expression_statement(env, v),
		Stmt::Print(v) => print_statement(env, v),
		Stmt::Declaration(v) => declaration_statement(env, v),
		Stmt::Block(v) => block_statement(env, v),
		Stmt::If(v) => if_statement(env, v),
		Stmt::For(v) => for_statement(env, v),
		Stmt::Return(v) => return_statement(env, v),
		Stmt::Break(v) => break_statement(env, v),
		Stmt::Continue(v) => continue_statement(env, v),
	}
}

pub fn eval_expression(
	expr: &Expr,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Literal(v) => literal_expression(env, v),
		Expr::Grouping(v) => eval_expression(&v.expression, env),
		Expr::Unary(v) => eval_unary(v, env),
		Expr::Binary(v) => eval_binary(v, env),
		Expr::Identifier(v) => identifier_expression(env, v),
		Expr::Assignment(v) => assignment_expression(env, v),
		Expr::Call(v) => call_expression(env, v),
		Expr::Function(v) => function_expression(env, v),
	}
}

fn eval_unary(
	v: &UnaryValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let right_value = eval_expression(&v.right, env)?;

	match (&v.operator.token_type, &right_value) {
		(TokenType::Minus, InterpreterValue::Number(n)) => {
			Ok(InterpreterValue::Number(-n))
		}
		(TokenType::Bang, InterpreterValue::True) => {
			Ok(InterpreterValue::False)
		}
		(TokenType::Bang, InterpreterValue::False) => {
			Ok(InterpreterValue::True)
		}

		_ => Err(RuntimeError {
			message: format!(
				"Cannot use `{}` on `{}`",
				v.operator.token_type, right_value
			),
			token: v.operator.clone(),
		}),
	}
}

fn eval_binary(
	v: &BinaryValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	// first, match the logical operators, so that we can have short-circuiting
	match v.operator.token_type {
		TokenType::Or => {
			return Ok(
				if eval_expression(&v.left, env)? == InterpreterValue::True {
					InterpreterValue::True
				} else {
					eval_expression(&v.right, env)?
				},
			)
		}
		TokenType::And => {
			let left_value = eval_expression(&v.left, env)?;

			return Ok(if left_value == InterpreterValue::True {
				eval_expression(&v.right, env)?
			} else {
				left_value
			});
		}
		_ => (),
	}

	// then eval_statement both sides normally
	let left_value = eval_expression(&v.left, env)?;
	let right_value = eval_expression(&v.right, env)?;

	// im sorry for this, but i found that the nested matches require
	// much simpler patterns,
	// and with this, i can achieve less comparisons overall
	match v.operator.token_type {
		TokenType::BangEqual => Ok((left_value != right_value).into()),
		TokenType::EqualEqual => Ok((left_value == right_value).into()),

		_ => match (&left_value, &right_value) {
			(InterpreterValue::Number(n1), InterpreterValue::Number(n2)) => {
				match v.operator.token_type {
					TokenType::Minus => Ok(InterpreterValue::Number(n1 - n2)),
					TokenType::Slash => Ok(InterpreterValue::Number(n1 / n2)),
					TokenType::Star => Ok(InterpreterValue::Number(n1 * n2)),
					TokenType::Plus => Ok(InterpreterValue::Number(n1 + n2)),
					TokenType::Greater => Ok((n1 > n2).into()),
					TokenType::GreaterEqual => Ok((n1 >= n2).into()),
					TokenType::Less => Ok((n1 < n2).into()),
					TokenType::LessEqual => Ok((n1 <= n2).into()),

					_ => unreachable!("Scanner did a bad job 😎."),
				}
			}
			(InterpreterValue::String(s1), InterpreterValue::String(s2)) => {
				if v.operator.token_type == TokenType::Plus {
					Ok(InterpreterValue::String(Rc::from(s1.to_string() + s2)))
				} else {
					Err(RuntimeError {
						message: format!(
							"You cannot use `{}` on two strings. Did you mean \
							 `+`?",
							v.operator.token_type
						),
						token: v.operator.clone(),
					})
				}
			}
			// error bby
			_ => Err(RuntimeError {
				message: format!(
					"Cannot use `{}` on {} and {}",
					v.operator.token_type,
					left_value.to_human_readable(),
					right_value.to_human_readable()
				),
				token: v.operator.clone(),
			}),
		},
	}
}
