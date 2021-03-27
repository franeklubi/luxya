use super::{helpers::*, interpret::*, interpreter_env::*, types::*};
use crate::{ast::expr::*, env::*, token::*};

use std::rc::Rc;


#[inline(always)]
pub fn literal_expression(
	v: &LiteralValue,
) -> Result<InterpreterValue, RuntimeError> {
	Ok(v.clone().into())
}

#[inline(always)]
pub fn identifier_expression(
	v: &IdentifierValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	Ok(env.read(&v.name)?.value)
}

#[inline(always)]
pub fn assignment_expression(
	v: &AssignmentValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	env.assign(&v.name, eval_expression(&v.value, env)?)
}

#[inline(always)]
pub fn call_expression(
	v: &CallValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	fn confirm_arity(
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

	fn map_arguments(
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

	let callee = eval_expression(&v.calee, env)?;

	let (enclosing_env, fun) =
		if let InterpreterValue::Function { enclosing_env, fun } = callee {
			Ok((enclosing_env, fun))
		} else {
			Err(RuntimeError {
				message: format!("Cannot call {}", callee.to_human_readable()),
				token: v.closing_paren.clone(),
			})
		}?;

	let arguments = v
		.arguments
		.iter()
		.map(|a| eval_expression(a, env))
		.collect::<Result<Vec<_>, RuntimeError>>()?;

	match &*fun {
		InterpreterFunction::LoxDefined(fv) => {
			confirm_arity(
				fv.params.as_ref().map_or(0, |p| p.len()),
				arguments.len(),
				&v.closing_paren,
			)?;

			let fun_env = &enclosing_env.fork();

			if let Some(params) = &fv.params {
				map_arguments(params, &arguments, fun_env)
			}

			if let Some(statements) = &fv.body {
				let e = eval_statements(&*statements, fun_env)?;
				Ok(guard_function(e)?)
			} else {
				Ok(InterpreterValue::Nil)
			}
		}
		InterpreterFunction::Native { arity, fun } => {
			confirm_arity(*arity, arguments.len(), &v.closing_paren)?;

			Ok(fun(&v.closing_paren, &enclosing_env.fork(), &arguments)?)
		}
	}
}

#[inline(always)]
pub fn function_expression(
	v: &FunctionValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let fun = InterpreterValue::Function {
		enclosing_env: env.clone(),
		fun: Rc::new(InterpreterFunction::LoxDefined(FunctionValue {
			body: v.body.as_ref().map(|b| Rc::clone(b)),
			keyword: v.keyword.clone(),
			name: v.name.clone(),
			params: v.params.clone(),
		})),
	};

	if let Some(t) = &v.name {
		let iden = assume_identifier(t);

		env.declare(
			iden.to_string(),
			DeclaredValue {
				mutable: false,
				value: fun.clone(),
			},
		);
	}

	Ok(fun)
}
