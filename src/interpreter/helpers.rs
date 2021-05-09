use super::{interpret::eval_expression, interpreter_env::*, types::*};
use crate::{
	ast::expr::{FunctionValue, GetAccessor},
	env::*,
	token::*,
};

use std::{cell::RefMut, rc::Rc};


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

#[inline(always)]
pub fn unwrap_list<'a>(
	value: &'a InterpreterValue,
	blame: &Token,
	arg_index: usize,
	override_msg: Option<String>,
) -> Result<RefMut<'a, Vec<InterpreterValue>>, RuntimeError> {
	if let InterpreterValue::List(l) = &value {
		Ok(l.borrow_mut())
	} else {
		Err(RuntimeError {
			message: override_msg.unwrap_or_else(|| {
				format!("Argument {} must be of type list", arg_index)
			}),
			token: blame.clone(),
		})
	}
}

#[inline(always)]
pub fn extract_subscription_index(
	accessor: &GetAccessor,
	blame: &Token,
	max_len: usize,
	env: &InterpreterEnvironment,
) -> Result<usize, RuntimeError> {
	let extracted_n = match &accessor {
		GetAccessor::SubscriptionNumber(n) => Ok(*n),
		GetAccessor::SubscriptionEval(expr) => {
			let eval = eval_expression(expr, env)?;

			if let InterpreterValue::Number(n) = eval {
				Ok(n)
			} else {
				Err(RuntimeError {
					message: format!(
						"Cannot use {} for indexing",
						eval.to_human_readable()
					),
					token: blame.clone(),
				})
			}
		}
		_ => unreachable!("Wrong accessor in subscription"),
	}?;

	if extracted_n.fract() != 0.0 || extracted_n < 0.0 {
		return Err(RuntimeError {
			message: format!("Cannot access element on index {}", extracted_n),
			token: blame.clone(),
		});
	}

	let index = extracted_n as usize;

	if index >= max_len {
		Err(RuntimeError {
			message: format!("Index {} out of bounds", extracted_n),
			token: blame.clone(),
		})
	} else {
		Ok(extracted_n as usize)
	}
}
