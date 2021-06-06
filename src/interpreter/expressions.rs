use super::{helpers::*, interpret::*, interpreter_env::*, types::*};
use crate::{ast::expr::*, env::*, token::*};

use std::{cell::RefCell, collections::HashMap, rc::Rc};


// inlining because it's used only once, but i wanted to take it
// out of the context, to make it less cluttery
#[inline(always)]
pub fn literal_expression(
	v: &LiteralValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match v {
		LiteralValue::String(s) => Ok(InterpreterValue::String(Rc::clone(&s))),
		LiteralValue::Number(n) => Ok(InterpreterValue::Number(*n)),
		LiteralValue::True => Ok(InterpreterValue::True),
		LiteralValue::False => Ok(InterpreterValue::False),
		LiteralValue::Nil => Ok(InterpreterValue::Nil),
		LiteralValue::Char(c) => Ok(InterpreterValue::Char(*c)),
		LiteralValue::List(l) => {
			let values =
				l.iter()
					.map(|expr| eval_expression(expr, env))
					.collect::<Result<Vec<InterpreterValue>, RuntimeError>>()?;

			Ok(InterpreterValue::List(Rc::new(RefCell::new(values))))
		}
	}
}

#[inline(always)]
pub fn identifier_expression<E, T>(
	v: &IdentifierValue,
	env: &E,
) -> Result<T, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	Ok(env.read(v.env_distance.get(), &v.name)?.value)
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
	env.assign(
		v.env_distance.get(),
		&v.name,
		expr_evaluator(&v.value, env)?,
	)
}

#[inline(always)]
pub fn call_expression(
	v: &CallValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let callee = eval_expression(&v.calee, env)?;

	execute_call(&callee, &v.arguments, &v.closing_paren, env)
}

pub fn execute_call(
	callee: &InterpreterValue,
	arguments: &[Expr],
	blame: &Token,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	match callee {
		InterpreterValue::Function { fun, enclosing_env } => {
			let arguments = arguments
				.iter()
				.map(|arg| eval_expression(arg, env))
				.collect::<Result<Vec<_>, RuntimeError>>()?;

			match &**fun {
				InterpreterFunction::LoxDefined(fv) => {
					confirm_arity(
						fv.params.as_ref().map_or(0, |p| p.len()),
						arguments.len(),
						blame,
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
					confirm_arity(*arity, arguments.len(), blame)?;

					Ok(fun(blame, &enclosing_env.fork(), &arguments)?)
				}
			}
		}
		InterpreterValue::Class { constructor, .. } => {
			let instance = InterpreterValue::Instance {
				class: Some(Rc::new(callee.clone())),
				properties: Rc::new(RefCell::new(HashMap::new())),
			};

			if let Some(constructor) = &constructor {
				let constructor = bind_function(constructor, instance.clone());

				execute_call(&constructor, arguments, blame, env)?;
			}

			Ok(instance)
		}
		_ => Err(RuntimeError {
			message: format!("Cannot call {}", callee.human_type()),
			token: blame.clone(),
		}),
	}
}

#[inline(always)]
pub fn function_expression(
	v: &FunctionValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let fun = construct_lox_defined_function(v, env);

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
					TokenType::Modulo => Ok(InterpreterValue::Number(n1 % n2)),

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
					left_value.human_type(),
					right_value.human_type()
				),
				token: v.operator.clone(),
			}),
		},
	}
}

fn find_method(
	key: &str,
	class: &InterpreterValue,
	instance: &InterpreterValue,
	blame: &Token,
) -> Result<InterpreterValue, RuntimeError> {
	let (methods, superclass) = if let InterpreterValue::Class {
		methods,
		superclass,
		..
	} = &class
	{
		(methods, superclass)
	} else {
		unreachable!("Class is not a class? 🤔")
	};

	if let Some(method) = methods.get(key) {
		Ok(bind_function(method, instance.clone()))
	} else if let Some(superclass) = &superclass {
		find_method(key, superclass, instance, blame)
	} else {
		Err(RuntimeError {
			message: format!(
				"Couldnt find property nor method with key {}",
				key
			),
			token: blame.clone(),
		})
	}
}

fn get_dot(
	v: &GetValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	// auxiliary function used only once down below, that's why inlining is
	// completely justified 🥺
	#[inline(always)]
	fn get_property(
		key: &str,
		properties: &HashMap<String, InterpreterValue>,
		class: &Option<Rc<InterpreterValue>>,
		instance: &InterpreterValue,
		blame: &Token,
	) -> Result<InterpreterValue, RuntimeError> {
		if let Some(p) = properties.get(key) {
			Ok(p.clone())
		} else if let Some(class) = class {
			find_method(key, class, instance, blame)
		} else {
			Err(RuntimeError {
				message: format!("Property {} not defined", key),
				token: blame.clone(),
			})
		}
	}

	let getee = eval_expression(&v.getee, env)?;

	let (properties, class) =
		if let InterpreterValue::Instance { properties, class } = &getee {
			(properties, class)
		} else {
			return Err(RuntimeError {
				message: format!(
					"Can't access properties on {}",
					getee.human_type()
				),
				token: v.blame.clone(),
			});
		};

	let borrowed_props = properties.borrow();

	match &v.key {
		GetAccessor::DotName(iden) => {
			get_property(iden, &borrowed_props, &class, &getee, &v.blame)
		}
		GetAccessor::DotEval(expr) => {
			let key = eval_expression(expr, env)?.to_string();

			get_property(
				&key.as_str(),
				&borrowed_props,
				&class,
				&getee,
				&v.blame,
			)
		}
		_ => unreachable!("Wrong accessor in dot"),
	}
}

fn get_subscription(
	v: &GetValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let getee_val = eval_expression(&v.getee, env)?;

	match getee_val {
		InterpreterValue::String(s) => {
			let index =
				extract_subscription_index(&v.key, &v.blame, s.len(), env)?;

			unsafe {
				Ok(InterpreterValue::Char(char::from(
					*s.as_bytes().get_unchecked(index),
				)))
			}
		}
		InterpreterValue::List(l) => {
			let l_borrow = l.borrow();

			let index = extract_subscription_index(
				&v.key,
				&v.blame,
				l_borrow.len(),
				env,
			)?;

			unsafe { Ok(l_borrow.get_unchecked(index).clone()) }
		}
		_ => Err(RuntimeError {
			message: format!("Cannot index {}", getee_val.human_type()),
			token: v.blame.clone(),
		}),
	}
}

#[inline(always)]
pub fn get_expression(
	v: &GetValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	if matches!(v.key, GetAccessor::DotName(_) | GetAccessor::DotEval(_)) {
		get_dot(v, env)
	} else {
		get_subscription(v, env)
	}
}

fn set_dot(
	v: &SetValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let setee = eval_expression(&v.setee, env)?;

	let properties = if let InterpreterValue::Instance { properties, .. } =
		setee
	{
		properties
	} else {
		return Err(RuntimeError {
			message: format!("Can't set properties on {}", setee.human_type()),
			token: v.blame.clone(),
		});
	};

	let value = eval_expression(&v.value, env)?;

	let mut borrowed_props = properties.borrow_mut();

	match &v.key {
		GetAccessor::DotName(key) => {
			borrowed_props.insert(key.to_string(), value.clone());
		}
		GetAccessor::DotEval(expr) => {
			let key = eval_expression(expr, env)?.to_string();
			borrowed_props.insert(key, value.clone());
		}
		_ => unreachable!("How"),
	};

	Ok(value)
}

fn set_subscription(
	v: &SetValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let setee = eval_expression(&v.setee, env)?;

	let mut l_borrow = unwrap_list(
		&setee,
		&v.blame,
		0,
		Some(
			"Setting values by using the `[]` operator is allowed only on \
			 lists"
				.to_owned(),
		),
	)?;

	let index =
		extract_subscription_index(&v.key, &v.blame, l_borrow.len(), env)?;

	let value = eval_expression(&v.value, env)?;

	l_borrow[index] = value.clone();

	Ok(value)
}

#[inline(always)]
pub fn set_expression(
	v: &SetValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	if matches!(v.key, GetAccessor::DotName(_) | GetAccessor::DotEval(_)) {
		set_dot(v, env)
	} else {
		set_subscription(v, env)
	}
}

#[inline(always)]
pub fn this_expression<E, T>(v: &ThisValue, env: &E) -> Result<T, RuntimeError>
where
	E: EnvironmentWrapper<T>,
{
	Ok(env.read(v.env_distance.get(), &v.blame)?.value)
}

pub fn super_expression(
	v: &SuperValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let env_distance = v.env_distance.get();

	let superclass = env.read(env_distance, &v.blame)?.value;

	// resolver got us this far, so we believe it that env with bound `this` is
	// 1 env-hop closer to us
	let instance = env
		.read(
			env_distance - 1,
			&Token {
				location: Location {
					byte_offset: 0,
					byte_length: 0,
				},
				token_type: TokenType::This,
			},
		)?
		.value;

	match &v.accessor {
		SuperAccessor::Method(m) => {
			let name = assume_identifier(m);

			find_method(name, &superclass, &instance, m)
		}
		SuperAccessor::Call(args) => {
			let constructor =
				if let InterpreterValue::Class { constructor, .. } = superclass
				{
					constructor
				} else {
					unreachable!("Superclass should be a class like come on 🤦")
				};

			let constructor = constructor.ok_or_else(|| RuntimeError {
				message: "Superclass does not have a constructor".into(),
				token: v.blame.clone(),
			})?;

			let constructor = bind_function(&constructor, instance);

			execute_call(&constructor, &args, &v.blame, env)
		}
	}
}

pub fn object_expression(
	v: &ObjectValue,
	env: &InterpreterEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	let properties = v
		.properties
		.iter()
		.map(|p| {
			let value = eval_expression(&p.value, env)?;

			Ok((p.key.to_string(), value))
		})
		.collect::<Result<HashMap<String, InterpreterValue>, RuntimeError>>()?;

	Ok(InterpreterValue::Instance {
		class: None,
		properties: Rc::new(RefCell::new(properties)),
	})
}
