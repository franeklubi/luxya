use super::{helpers::*, interpret::*, interpreter_env::*, types::*};
use crate::{ast::expr::*, env::*, token::*};

use std::rc::Rc;


#[inline(always)]
pub fn literal_expression<T>(v: &LiteralValue) -> Result<T, RuntimeError>
where
	T: From<LiteralValue>,
{
	Ok(v.clone().into())
}

#[inline(always)]
pub fn identifier_expression<E, T>(
	v: &IdentifierValue,
	env: &E,
) -> Result<T, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	Ok(env.read(&v.name)?.value)
}

#[inline(always)]
pub fn assignment_expression<E, T>(
	expr_evaluator: fn(&Expr, &E) -> Result<T, RuntimeError>,
	v: &AssignmentValue,
	env: &E,
) -> Result<T, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	env.assign(&v.name, expr_evaluator(&v.value, env)?)
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

pub fn unary_expression(
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

pub fn binary_experssion(
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
