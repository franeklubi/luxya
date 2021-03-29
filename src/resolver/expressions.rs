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
	v: &AssignmentValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	resolve::resolve_expression(&v.value, env)?;

	env.resolve_nest_level(&v.value, &v.name)?;

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
