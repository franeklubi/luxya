use super::{resolve, resolver_env::ResolverEnvironment};
use crate::{
	ast::expr::*,
	env::*,
	interpreter::{
		helpers::*,
		types::{InterpreterValue, RuntimeError},
	},
};


#[inline(always)]
pub fn identifier_expression(
	expr: &Expr,
	v: &IdentifierValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	env.resolve_nest_level(expr, &v.name)?;

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn assignment_expression(
	expr: &Expr,
	v: &AssignmentValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	// that takes care on the variables on the right
	resolve::resolve_expression(&v.value, env)?;

	// and this one manages the ones on the left 😎
	env.resolve_nest_level(expr, &v.name)?;

	env.assign(v.env_distance.get(), &v.name, InterpreterValue::Nil)?;

	Ok(InterpreterValue::Nil)
}

pub fn function_expression(
	v: &FunctionValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	// if the function has a name attached, declare it in scope
	if let Some(name) = &v.name {
		let iden = assume_identifier(name);

		env.declare(
			iden.to_owned(),
			DeclaredValue {
				mutable: false,
				value: InterpreterValue::Nil,
			},
		);
	}

	let new_scope = env.fork();

	// declaring dummy for each parameter
	if let Some(params) = &v.params {
		params.iter().for_each(|param| {
			let name = assume_identifier(param);

			new_scope.declare(
				name.to_owned(),
				DeclaredValue {
					mutable: true,
					value: InterpreterValue::Nil,
				},
			);
		});
	}

	// evaluating function body
	if let Some(statements) = &v.body {
		let e = resolve::resolve_statements(statements, &new_scope)?;
		Ok(guard_function(e)?)
	} else {
		Ok(InterpreterValue::Nil)
	}
}

#[inline(always)]
pub fn binary_expression(
	v: &BinaryValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	resolve::resolve_expression(&v.left, env)?;
	resolve::resolve_expression(&v.right, env)?;

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn call_expression(
	v: &CallValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	resolve::resolve_expression(&v.calee, env)?;

	for arg in &v.arguments {
		resolve::resolve_expression(arg, env)?;
	}

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn get_expression(
	v: &GetValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	resolve::resolve_expression(&v.getee, env)?;

	match &v.key {
		GetAccessor::DotEval(key) => {
			resolve::resolve_expression(key, env)?;
		}
		GetAccessor::SubscriptionEval(expr) => {
			resolve::resolve_expression(expr, env)?;
		}
		_ => (),
	}

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn set_expression(
	v: &SetValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	resolve::resolve_expression(&v.setee, env)?;
	resolve::resolve_expression(&v.value, env)?;

	if let GetAccessor::DotEval(key) = &v.key {
		resolve::resolve_expression(key, env)?;
	}

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn this_expression(
	expr: &Expr,
	v: &ThisValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	env.resolve_nest_level(expr, &v.blame)
		.map_err(|err| RuntimeError {
			token: err.token,
			message: "Cannot call `this` outside of a method".into(),
		})?;

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn super_expression(
	expr: &Expr,
	v: &SuperValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	env.resolve_nest_level(expr, &v.blame)
		.map_err(|err| RuntimeError {
			token: err.token,
			message: "Cannot call `super` outside of a child class method"
				.into(),
		})?;

	Ok(InterpreterValue::Nil)
}

#[inline(always)]
pub fn object_expression(
	v: &ObjectValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	for value in v.properties.iter().map(|p| &p.value) {
		resolve::resolve_expression(&value, env)?;
	}

	Ok(InterpreterValue::Nil)
}
