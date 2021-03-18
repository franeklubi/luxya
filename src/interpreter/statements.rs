use super::{env::*, helpers::*, interpret::*, types::*};
use crate::ast::stmt::*;


#[inline(always)]
pub fn expression_statement(
	env: &WrappedInterpreterEnvironment,
	v: &ExpressionValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	eval_expression(&v.expression, env)?;

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn print_statement(
	env: &WrappedInterpreterEnvironment,
	v: &PrintValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	let evaluated = eval_expression(&v.expression, env)?;

	println!("{}", evaluated);

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn declaration_statement(
	env: &WrappedInterpreterEnvironment,
	v: &DeclarationValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	let value = v
		.initializer
		.as_ref()
		.map_or(Ok(InterpreterValue::Nil), |initializer| {
			eval_expression(&initializer, env)
		})?;

	env.declare(
		assume_identifier(&v.name).to_owned(),
		DeclaredValue {
			mutable: v.mutable,
			value,
		},
	);

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn block_statement(
	env: &WrappedInterpreterEnvironment,
	v: &BlockValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	let new_scope = env.fork();

	evaluate_statements(&v.statements, &new_scope)
}

#[inline(always)]
pub fn if_statement(
	env: &WrappedInterpreterEnvironment,
	v: &IfValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	if eval_expression(&v.condition, env)? == InterpreterValue::True {
		eval_statement(&v.then, env)
	} else if let Some(otherwise) = &v.otherwise {
		eval_statement(otherwise, env)
	} else {
		Ok(InterpreterStmtValue::Noop)
	}
}

#[inline(always)]
pub fn for_statement(
	env: &WrappedInterpreterEnvironment,
	v: &ForValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	// these branches look sooo sketchy, but it's an optimization for
	// condition-less loops
	if let Some(condition) = &v.condition {
		while eval_expression(condition, env)? == InterpreterValue::True {
			let e = eval_statement(&v.body, env)?;

			match e {
				InterpreterStmtValue::Break(_) => break,
				InterpreterStmtValue::Continue(_) => {
					if let Some(c) = &v.closer {
						eval_statement(c, env)?;
					}

					continue;
				}
				InterpreterStmtValue::Noop => (),
				InterpreterStmtValue::Return { .. } => {
					return Ok(e);
				}
			}

			if let Some(c) = &v.closer {
				eval_statement(c, env)?;
			}
		}
	} else {
		loop {
			let e = eval_statement(&v.body, env)?;

			match e {
				InterpreterStmtValue::Break(_) => break,
				InterpreterStmtValue::Continue(_) => {
					if let Some(c) = &v.closer {
						eval_statement(c, env)?;
					}

					continue;
				}
				InterpreterStmtValue::Noop => (),
				InterpreterStmtValue::Return { .. } => {
					return Ok(e);
				}
			}

			if let Some(c) = &v.closer {
				eval_statement(c, env)?;
			}
		}
	}

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn return_statement(
	env: &WrappedInterpreterEnvironment,
	v: &ReturnValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	Ok(InterpreterStmtValue::Return {
		value: v
			.expression
			.as_ref()
			.map_or(Ok(InterpreterValue::Nil), |e| eval_expression(e, env))?,
		keyword: v.keyword.clone(),
	})
}

#[inline(always)]
pub fn break_statement(
	_env: &WrappedInterpreterEnvironment,
	v: &BreakValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	Ok(InterpreterStmtValue::Break(v.keyword.clone()))
}

#[inline(always)]
pub fn continue_statement(
	_env: &WrappedInterpreterEnvironment,
	v: &ContinueValue,
) -> Result<InterpreterStmtValue, RuntimeError> {
	Ok(InterpreterStmtValue::Continue(v.keyword.clone()))
}
