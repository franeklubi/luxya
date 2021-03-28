use super::resolver_env::ResolverEnvironment;
use crate::{
	ast::expr::*,
	env::*,
	interpreter::types::{InterpreterValue, RuntimeError},
};


#[inline(always)]
pub fn identifier_expression(
	expr: &Expr,
	v: &IdentifierValue,
	env: &ResolverEnvironment,
) -> Result<InterpreterValue, RuntimeError> {
	env.read(&v.name)?;

	env.resolve_nest_level(expr, &v.name)?;

	Ok(InterpreterValue::Nil)
}
