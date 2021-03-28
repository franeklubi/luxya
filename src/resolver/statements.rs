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
pub fn declaration_statement(
	v: &DeclarationValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterStmtValue<InterpreterValue>, RuntimeError> {
	if let Some(i) = &v.initializer {
		resolve::eval_expression(i, env)?;
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
