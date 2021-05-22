use super::{
	helpers::unwrap_list,
	interpreter_env::InterpreterEnvironment,
	types::*,
};
use crate::{env::*, token::*};

use std::{
	cell::RefCell,
	io::{self, Write},
	rc::Rc,
};


pub const NATIVE_FUNCTION_NAMES: [&str; 15] = [
	"str",
	"typeof",
	"number",
	"len",
	"expand",
	"push",
	"extend",
	"from_chars",
	"deep_copy",
	"is_nan",
	"floor",
	"ceil",
	"has",
	"unset",
	"read",
];

struct FunctionDefinition<'a> {
	name: &'a str,
	arity: usize,
	fun: NativeFunctionSignature,
}


fn native_str(
	_keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> InterpreterValue {
	let input = &args[0];

	if let InterpreterValue::String(_) = input {
		input.clone()
	} else {
		InterpreterValue::String(Rc::from(input.to_string()))
	}
}

fn native_typeof(
	_keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> InterpreterValue {
	InterpreterValue::String(Rc::from(args[0].human_type()))
}

fn native_number(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let input = &args[0];

	match input {
		InterpreterValue::Number(_) => Ok(input.clone()),
		InterpreterValue::String(s) => {
			Ok(InterpreterValue::Number(s.parse().or(Ok(f64::NAN))?))
		}
		InterpreterValue::Char(c) => Ok(InterpreterValue::Number(
			c.to_digit(10).map_or(f64::NAN, |d| d.into()),
		)),
		_ => Err(RuntimeError {
			message: format!("Can't parse {} to number", input.human_type()),
			token: keyword.clone(),
		}),
	}
}

fn native_len(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	match &args[0] {
		InterpreterValue::String(s) => {
			Ok(InterpreterValue::Number(s.len() as f64))
		}
		InterpreterValue::List(l) => {
			let l_borrow = l.borrow();

			Ok(InterpreterValue::Number(l_borrow.len() as f64))
		}
		_ => Err(RuntimeError {
			message: format!("Can't get length of {}", &args[0].human_type()),
			token: keyword.clone(),
		}),
	}
}

fn native_expand(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let val = &args[0];

	match val {
		InterpreterValue::String(s) => Ok(InterpreterValue::List(Rc::new(
			RefCell::new(s.chars().map(InterpreterValue::Char).collect()),
		))),
		InterpreterValue::Instance { properties, .. } => {
			let keys = properties
				.borrow()
				.keys()
				.cloned()
				.map(|k| InterpreterValue::String(k.into()))
				.collect();

			Ok(InterpreterValue::List(Rc::new(RefCell::new(keys))))
		}
		_ => Err(RuntimeError {
			message: format!("Can't extract chars out of {}", val.human_type()),
			token: keyword.clone(),
		}),
	}
}

fn native_push(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let mut l_borrow = unwrap_list(&args[0], &keyword, 0, None)?;

	l_borrow.push(args[1].clone());

	drop(l_borrow);

	Ok(args[0].clone())
}

fn native_extend(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let second_items = unwrap_list(&args[1], keyword, 1, None)?
		.iter()
		.cloned()
		.collect::<Vec<InterpreterValue>>();

	unwrap_list(&args[0], keyword, 0, None)?.extend(second_items);

	Ok(args[0].clone())
}

fn native_from_chars(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let l_borrow = unwrap_list(&args[0], keyword, 0, None)?;

	let string = l_borrow
		.iter()
		.map(|v| {
			if let InterpreterValue::Char(c) = v {
				Ok(*c)
			} else {
				Err(RuntimeError {
					message: format!(
						"Cannot convert from {} to char",
						v.human_type()
					),
					token: keyword.clone(),
				})
			}
		})
		.collect::<Result<String, RuntimeError>>()?;

	Ok(InterpreterValue::String(Rc::from(string)))
}

fn native_deep_copy(
	keyword: &Token,
	env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let value = &args[0];

	match value {
		InterpreterValue::Instance { class, properties } => {
			let cloned_properties = properties
				.borrow()
				.iter()
				.map(|p| {
					let key = p.0.clone();
					let value = native_deep_copy(keyword, env, &[p.1.clone()])?;

					Ok((key, value))
				})
				.collect::<Result<_, _>>()?;

			Ok(InterpreterValue::Instance {
				class: class.clone(),
				properties: Rc::new(RefCell::new(cloned_properties)),
			})
		}
		InterpreterValue::List(l) => {
			let cloned_list = l
				.borrow()
				.iter()
				.map(|v| native_deep_copy(keyword, env, &[v.clone()]))
				.collect::<Result<_, _>>()?;

			Ok(InterpreterValue::List(Rc::new(RefCell::new(cloned_list))))
		}
		_ => Ok(value.clone()),
	}
}

fn native_is_nan(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let value = &args[0];

	if let InterpreterValue::Number(n) = value {
		Ok(n.is_nan().into())
	} else {
		Err(RuntimeError {
			message: format!("Cannot use is_nan on {}", value.human_type()),
			token: keyword.clone(),
		})
	}
}

fn native_floor(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let value = &args[0];

	if let InterpreterValue::Number(n) = value {
		Ok(InterpreterValue::Number(n.floor()))
	} else {
		Err(RuntimeError {
			message: format!("Cannot use floor on {}", value.human_type()),
			token: keyword.clone(),
		})
	}
}

fn native_ceil(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let value = &args[0];

	if let InterpreterValue::Number(n) = value {
		Ok(InterpreterValue::Number(n.ceil()))
	} else {
		Err(RuntimeError {
			message: format!("Cannot use ceil on {}", value.human_type()),
			token: keyword.clone(),
		})
	}
}

fn native_has(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let searchee = &args[0];
	let value = &args[1];

	match (searchee, value) {
		(InterpreterValue::Instance { properties, .. }, _) => {
			let borrowed_props = properties.borrow();

			Ok(borrowed_props.contains_key(&value.to_string()).into())
		}
		(InterpreterValue::List(l), _) => {
			let l_borrow = l.borrow();

			Ok(l_borrow.iter().any(|v| v == value).into())
		}
		(InterpreterValue::String(s1), InterpreterValue::String(s2)) => {
			Ok(s1.contains(&**s2).into())
		}
		(InterpreterValue::String(s), InterpreterValue::Char(c)) => {
			Ok(s.contains(*c).into())
		}
		_ => Err(RuntimeError {
			message: format!(
				"Cannot use has with {} and {}",
				searchee.human_type(),
				value.human_type()
			),
			token: keyword.clone(),
		}),
	}
}

fn native_unset(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let map = &args[0];
	let key = &args[1];

	match (map, key) {
		(
			InterpreterValue::Instance { properties, .. },
			InterpreterValue::String(s),
		) => {
			let mut borrowed_props = properties.borrow_mut();

			Ok(borrowed_props.remove(&**s).unwrap_or(InterpreterValue::Nil))
		}
		_ => Err(RuntimeError {
			message: format!(
				"Cannot use unset with {} and {}",
				map.human_type(),
				key.human_type()
			),
			token: keyword.clone(),
		}),
	}
}

fn native_read(
	keyword: &Token,
	_env: &InterpreterEnvironment,
	args: &[InterpreterValue],
) -> Result<InterpreterValue, RuntimeError> {
	let to_print = if args[0] == InterpreterValue::Nil {
		"".to_owned()
	} else {
		args[0].to_string()
	};

	print!("{}", to_print);

	io::stdout().flush().map_err(|e| RuntimeError {
		message: e.to_string(),
		token: keyword.clone(),
	})?;

	let mut buffer = String::new();
	io::stdin()
		.read_line(&mut buffer)
		.map_err(|e| RuntimeError {
			message: e.to_string(),
			token: keyword.clone(),
		})?;

	Ok(InterpreterValue::String(buffer.into()))
}

fn declarator(env: &InterpreterEnvironment, funs: &[FunctionDefinition]) {
	funs.iter().for_each(|fd| {
		env.declare(
			fd.name.to_owned(),
			DeclaredValue {
				mutable: true,
				value: InterpreterValue::Function {
					fun: Rc::new(InterpreterFunction::Native {
						arity: fd.arity,
						fun: fd.fun,
					}),
					enclosing_env: env.clone(),
				},
			},
		);
	})
}

pub fn declare_native_functions(env: &InterpreterEnvironment) {
	declarator(
		env,
		&[
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[0],
				arity: 1,
				fun: |_k, _e, args| Ok(native_str(_k, _e, args)),
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[1],
				arity: 1,
				fun: |_k, _e, args| Ok(native_typeof(_k, _e, args)),
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[2],
				arity: 1,
				fun: native_number,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[3],
				arity: 1,
				fun: native_len,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[4],
				arity: 1,
				fun: native_expand,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[5],
				arity: 2,
				fun: native_push,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[6],
				arity: 2,
				fun: native_extend,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[7],
				arity: 1,
				fun: native_from_chars,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[8],
				arity: 1,
				fun: native_deep_copy,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[9],
				arity: 1,
				fun: native_is_nan,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[10],
				arity: 1,
				fun: native_floor,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[11],
				arity: 1,
				fun: native_ceil,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[12],
				arity: 2,
				fun: native_has,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[13],
				arity: 2,
				fun: native_unset,
			},
			FunctionDefinition {
				name: NATIVE_FUNCTION_NAMES[14],
				arity: 1,
				fun: native_read,
			},
		],
	);
}
