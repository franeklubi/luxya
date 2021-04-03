use super::{resolve, resolver_env::*};
use crate::{
	ast::stmt::*,
	env::*,
	interpreter::{
		helpers::assume_identifier,
		types::{InterpreterStmtValue, InterpreterValue, RuntimeError},
	},
};


#[inline(always)]
pub fn print_statement(
	v: &PrintValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	resolve::resolve_expression(&v.expression, env)?;

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn declaration_statement(
	v: &DeclarationValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	if let Some(i) = &v.initializer {
		resolve::resolve_expression(i, env)?;
	}

	env.declare(
		assume_identifier(&v.name).to_owned(),
		DeclaredValue {
			mutable: v.mutable,
			value: InterpreterValue::Nil,
		},
	);

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn if_statement(
	v: &IfValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	resolve::resolve_expression(&v.condition, env)?;
	resolve::resolve_statement(&v.then, env)?;

	if let Some(otherwise) = &v.otherwise {
		resolve::resolve_statement(otherwise, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn for_statement(
	v: &ForValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	resolve::resolve_statement(&v.body, env)?;

	if let Some(condition) = &v.condition {
		resolve::resolve_expression(condition, env)?;
	}

	if let Some(closer) = &v.closer {
		resolve::resolve_statement(closer, env)?;
	}

	Ok(InterpreterStmtValue::Noop)
}

#[inline(always)]
pub fn class_statement(
	v: &ClassValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	let iden = assume_identifier(&v.name);

	env.declare(
		iden.to_owned(),
		DeclaredValue {
			mutable: false,
			value: InterpreterValue::Nil,
		},
	);

	// TODO: methods

	Ok(InterpreterStmtValue::Noop)
}
