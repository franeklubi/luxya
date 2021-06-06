use super::{
	env::InterpreterEnvironment,
	helpers::{assume_identifier, construct_lox_defined_function},
	interpret::eval_expression,
	types::{InterpreterValue, RuntimeError, StmtResult},
};
use crate::{
	ast::{
		expr::Expr,
		stmt::{
			BlockValue,
			BreakValue,
			ClassValue,
			ContinueValue,
			DeclarationValue,
			ExpressionValue,
			ForValue,
			IfValue,
			PrintValue,
			ReturnValue,
			Stmt,
		},
	},
	env::{DeclaredValue, EnvironmentWrapper},
};

use std::{collections::HashMap, rc::Rc};


#[inline]
pub fn expression_statement<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<T, RuntimeError>,
	v: &ExpressionValue,
	env: &E,
) -> Result<StmtResult<T>, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	expr_evaluator(&v.expression, env)?;

	Ok(StmtResult::Noop)
}

#[inline]
pub fn print_statement<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<T, RuntimeError>,
	v: &PrintValue,
	env: &E,
) -> Result<StmtResult<T>, RuntimeError>
where
	T: std::fmt::Display,
	E: EnvironmentWrapper<T>,
{
	let evaluated = expr_evaluator(&v.expression, env)?;

	println!("{}", evaluated);

	Ok(StmtResult::Noop)
}

pub fn declaration_statement<E>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	v: &DeclarationValue,
	env: &E,
) -> Result<StmtResult<InterpreterValue>, RuntimeError>
where
	E: EnvironmentWrapper<InterpreterValue>,
{
	let value = v
		.initializer
		.as_ref()
		.map_or(Ok(InterpreterValue::Nil), |initializer| {
			expr_evaluator(initializer, env)
		})?;

	env.declare(
		assume_identifier(&v.name).to_owned(),
		DeclaredValue {
			mutable: v.mutable,
			value,
		},
	);

	Ok(StmtResult::Noop)
}

#[inline]
pub fn block_statement<E, T>(
	stmts_evaluator: fn(&[Stmt], &E) -> Result<StmtResult<T>, RuntimeError>,
	v: &BlockValue,
	env: &E,
) -> Result<StmtResult<T>, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	let new_scope = env.fork();

	stmts_evaluator(&v.statements, &new_scope)
}

#[inline]
pub fn if_statement<E>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	stmt_evaluator: fn(
		&Stmt,
		&E,
	) -> Result<StmtResult<InterpreterValue>, RuntimeError>,
	v: &IfValue,
	env: &E,
) -> Result<StmtResult<InterpreterValue>, RuntimeError>
where
	E: EnvironmentWrapper<InterpreterValue>,
{
	if expr_evaluator(&v.condition, env)? == InterpreterValue::True {
		v.then
			.as_ref()
			.map_or(Ok(StmtResult::Noop), |then| stmt_evaluator(then, env))
	} else if let Some(otherwise) = &v.otherwise {
		stmt_evaluator(otherwise, env)
	} else {
		Ok(StmtResult::Noop)
	}
}

pub fn for_statement<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	stmt_evaluator: fn(&Stmt, &E) -> Result<StmtResult<T>, RuntimeError>,
	v: &ForValue,
	env: &E,
) -> Result<StmtResult<T>, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	// these branches look sooo sketchy, but it's an optimization for
	// condition-less loops
	if let Some(condition) = &v.condition {
		while expr_evaluator(condition, env)? == InterpreterValue::True {
			let e = stmt_evaluator(&v.body, env)?;

			match e {
				StmtResult::Break(_) => break,
				StmtResult::Continue(_) => {
					if let Some(c) = &v.closer {
						stmt_evaluator(c, env)?;
					}

					continue;
				}
				StmtResult::Noop => (),
				StmtResult::Return { .. } => {
					return Ok(e);
				}
			}

			if let Some(c) = &v.closer {
				stmt_evaluator(c, env)?;
			}
		}
	} else {
		loop {
			let e = stmt_evaluator(&v.body, env)?;

			match e {
				StmtResult::Break(_) => break,
				StmtResult::Continue(_) => {
					if let Some(c) = &v.closer {
						stmt_evaluator(c, env)?;
					}

					continue;
				}
				StmtResult::Noop => (),
				StmtResult::Return { .. } => {
					return Ok(e);
				}
			}

			if let Some(c) = &v.closer {
				stmt_evaluator(c, env)?;
			}
		}
	}

	Ok(StmtResult::Noop)
}

#[inline]
pub fn return_statement<E>(
	expr_evaluator: fn(&Expr, &E) -> Result<InterpreterValue, RuntimeError>,
	v: &ReturnValue,
	env: &E,
) -> Result<StmtResult<InterpreterValue>, RuntimeError>
where
	E: EnvironmentWrapper<InterpreterValue>,
{
	Ok(StmtResult::Return {
		value: v
			.expression
			.as_ref()
			.map_or(Ok(InterpreterValue::Nil), |e| expr_evaluator(e, env))?,
		keyword: v.keyword.clone(),
	})
}

#[inline]
pub fn break_statement<T>(
	v: &BreakValue,
) -> Result<StmtResult<T>, RuntimeError> {
	Ok(StmtResult::Break(v.keyword.clone()))
}

#[inline]
pub fn continue_statement<T>(
	v: &ContinueValue,
) -> Result<StmtResult<T>, RuntimeError> {
	Ok(StmtResult::Continue(v.keyword.clone()))
}

pub fn class_statement(
	v: &ClassValue,
	env: &InterpreterEnvironment,
) -> Result<StmtResult<InterpreterValue>, RuntimeError> {
	let name = assume_identifier(&v.name);

	let (superclass, super_env) = if let Some(expr) = &v.superclass {
		let evaluated = eval_expression(expr, env)?;

		if !matches!(evaluated, InterpreterValue::Class { .. }) {
			return Err(RuntimeError {
				message: format!(
					"Cannot inherit from {}",
					evaluated.human_type()
				),
				token: v.name.clone(),
			});
		}

		let superclass = eval_expression(expr, env)?;

		let super_env = env.fork();

		super_env.declare(
			"super".into(),
			DeclaredValue {
				mutable: false,
				value: superclass.clone(),
			},
		);

		(Some(Rc::new(superclass)), Some(super_env))
	} else {
		(None, None)
	};

	let class_env = super_env.as_ref().unwrap_or(env);

	let mut methods = HashMap::new();

	let mut constructor = None;

	for method in &v.methods {
		let fv = if let Expr::Function(v) = method {
			v
		} else {
			unreachable!(
				"Method should be a function expression. Parser fucked up"
			)
		};

		let fun = construct_lox_defined_function(fv, class_env);

		let name = assume_identifier(fv.name.as_ref().expect("Method name"));

		if name == "constructor" {
			constructor = Some(Rc::new(fun));
		} else {
			methods.insert(name.to_owned(), fun);
		}
	}

	env.declare(
		name.to_owned(),
		DeclaredValue {
			mutable: false,
			value: InterpreterValue::Class {
				superclass,
				constructor,
				name: Rc::from(name),
				methods: Rc::new(methods),
			},
		},
	);

	Ok(StmtResult::Noop)
}
