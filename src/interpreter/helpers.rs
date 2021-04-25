use super::{interpreter_env::*, types::*};
use crate::{ast::expr::FunctionValue, env::*, token::*};

use std::rc::Rc;


// A shorthand way to extract identifier's name
pub fn assume_identifier(t: &Token) -> &str {
	match &t.token_type {
		TokenType::Identifier(i) => i,
		TokenType::Super => "super",
		TokenType::This => "this",
		_ => unreachable!("Couldn't extract identifier."),
	}
}

pub fn guard_function(
	ibv: InterpreterStmtValue<InterpreterValue>,
) -> Result<InterpreterValue, RuntimeError> {
	match ibv {
		InterpreterStmtValue::Break(token) => Err(RuntimeError {
			message: "Cannot use `break` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Continue(token) => Err(RuntimeError {
			message: "Cannot use `continue` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Return { value, .. } => Ok(value),
		InterpreterStmtValue::Noop => Ok(InterpreterValue::Nil),
	}
}

#[inline(always)]
pub fn no_identifier(token: &Token, name: &str) -> RuntimeError {
	RuntimeError {
		token: token.clone(),
		message: format!("Identifier {} not defined", name),
	}
}

#[macro_export]
macro_rules! unwrap_scope {
	($wie:expr) => {{
		&$wie.0.borrow().scope
	}};
}

#[macro_export]
macro_rules! unwrap_scope_mut {
	($wie:expr) => {{
		&mut $wie.0.borrow_mut().scope
	}};
}

#[macro_export]
macro_rules! unwrap_enclosing {
	($wie:expr) => {{
		&$wie.0.borrow().enclosing
	}};
}

#[inline(always)]
pub fn confirm_arity(
	target: usize,
	value: usize,
	blame: &Token,
) -> Result<(), RuntimeError> {
	if target != value {
		Err(RuntimeError {
			message: format!(
				"{} arguments",
				if value > target {
					"Too many"
				} else {
					"Not enough"
				}
			),
			token: blame.clone(),
		})
	} else {
		Ok(())
	}
}

#[inline(always)]
pub fn map_arguments(
	parameters: &[Token],
	arguments: &[InterpreterValue],
	fun_env: &InterpreterEnvironment,
) {
	parameters.iter().zip(arguments).for_each(|(param, arg)| {
		let name = assume_identifier(param);

		fun_env.declare(
			name.to_string(),
			DeclaredValue {
				mutable: true,
				value: arg.clone(),
			},
		);
	})
}

#[inline(always)]
pub fn construct_lox_defined_function(
	fv: &FunctionValue,
	env: &InterpreterEnvironment,
) -> InterpreterValue {
	InterpreterValue::Function {
		enclosing_env: env.clone(),
		fun: Rc::new(InterpreterFunction::LoxDefined(FunctionValue {
			body: fv.body.as_ref().map(|b| Rc::clone(b)),
			keyword: fv.keyword.clone(),
			name: fv.name.clone(),
			params: fv.params.as_ref().map(|p| Rc::clone(p)),
		})),
	}
}

pub fn bind_function(
	fun: &InterpreterValue,
	instance: InterpreterValue,
) -> InterpreterValue {
	let (fun, new_env) =
		if let InterpreterValue::Function { fun, enclosing_env } = fun {
			(fun.clone(), enclosing_env.fork())
		} else {
			unreachable!("CHuju kurwa panie")
		};

	new_env.declare(
		"this".into(),
		DeclaredValue {
			mutable: false,
			value: instance,
		},
	);

	InterpreterValue::Function {
		fun,
		enclosing_env: new_env,
	}
}
