use crate::{ast::stmt::*, interpreter::types::RuntimeError};

use super::{env::*, resolve::*};


#[inline(always)]
pub fn block_statement(
	env: &ResolverEnvironment,
	v: &BlockValue,
) -> Result<(), RuntimeError> {
	let new_scope = env.fork();

	resolve_statements(&v.statements, &new_scope)
}
