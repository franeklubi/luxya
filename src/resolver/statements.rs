use crate::{ast::stmt::*, env::*, interpreter::types::RuntimeError};

use super::{resolve::*, resolver_env::*};


#[inline(always)]
pub fn block_statement(
	env: &ResolverEnvironment,
	v: &BlockValue,
) -> Result<(), RuntimeError> {
	let new_scope = env.fork();

	resolve_statements(&v.statements, &new_scope)
}
