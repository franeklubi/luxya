use super::helpers::assume_resolvable_expr;
use crate::{
	ast::expr::Expr,
	env::*,
	interpreter::{
		helpers::{assume_identifier, no_identifier},
		types::{InterpreterValue, RuntimeError},
	},
	resolver_unwrap_enclosing,
	resolver_unwrap_scope,
	resolver_unwrap_scope_mut,
	token::Token,
};

use std::{cell::RefCell, rc::Rc};


// Everything we need to create resolved map will have to be inside this env
#[derive(Clone)]
pub struct ResolverEnvironment {
	// true if variable, false if const
	pub env: Rc<RefCell<EnvironmentBase<ResolverEnvironment, bool>>>,

	// current nest level
	level: u32,
}

// The InterpreterValue in this implementation tells us basically nothing, as
// we won't be resolving the true values of our nodes.
//
// It's just there to satisfy EnvironmentWrapper and a couple of statement
// functions in interpreter/statements.
//
// I'll always supply Nil here
impl EnvironmentWrapper<InterpreterValue> for ResolverEnvironment {
	fn new() -> Self {
		ResolverEnvironment {
			env: Rc::new(RefCell::new(EnvironmentBase::new(None))),
			level: 0,
		}
	}

	fn fork(&self) -> Self {
		ResolverEnvironment {
			env: Rc::new(RefCell::new(EnvironmentBase::new(Some(
				self.clone(),
			)))),
			level: self.level + 1,
		}
	}

	fn read(
		&self,
		_steps: u32,
		_identifier: &Token,
	) -> Result<DeclaredValue<InterpreterValue>, RuntimeError> {
		unimplemented!("Resolver env: use read_search")
	}

	fn read_search(
		&self,
		identifier: &Token,
	) -> Result<DeclaredValue<InterpreterValue>, RuntimeError> {
		let name = assume_identifier(&identifier);

		if let Some(dv) = resolver_unwrap_scope!(self).get(name) {
			Ok(DeclaredValue {
				mutable: *dv,
				value: InterpreterValue::Nil,
			})
		} else if let Some(enclosing) = resolver_unwrap_enclosing!(self) {
			enclosing.read_search(identifier)
		} else {
			Err(no_identifier(identifier, name))
		}
	}

	fn declare(
		&self,
		name: String,
		value: DeclaredValue<InterpreterValue>,
	) -> Option<DeclaredValue<InterpreterValue>> {
		resolver_unwrap_scope_mut!(self).insert(name, value.mutable);

		None
	}

	// checks if the target is mutable
	fn assign(
		&self,
		identifier: &Token,
		_value: InterpreterValue,
	) -> Result<InterpreterValue, RuntimeError> {
		let entry = self.read_search(identifier)?;

		if !entry.mutable {
			let name = assume_identifier(identifier);

			Err(RuntimeError {
				message: format!("Cannot assign to a const `{}`", name),
				token: identifier.clone(),
			})
		} else {
			Ok(InterpreterValue::Nil)
		}
	}
}

impl ResolverEnvironment {
	pub fn resolve_nest_level(
		&self,
		resolvable_node: &Expr,
		identifier_token: &Token,
	) -> Result<(), RuntimeError> {
		self.resolve_nest_level_worker(
			self.level,
			resolvable_node,
			identifier_token,
		)
	}

	fn resolve_nest_level_worker(
		&self,
		initial_level: u32,
		resolvable_node: &Expr,
		identifier_token: &Token,
	) -> Result<(), RuntimeError> {
		let name = assume_identifier(identifier_token);

		if let Some(_dv) = resolver_unwrap_scope!(self).get(name) {
			let env_distance = assume_resolvable_expr(resolvable_node);

			env_distance.set(initial_level - self.level);

			Ok(())
		} else if let Some(enclosing) = resolver_unwrap_enclosing!(self) {
			enclosing.resolve_nest_level_worker(
				initial_level,
				resolvable_node,
				identifier_token,
			)?;

			Ok(())
		} else {
			Err(no_identifier(identifier_token, name))
		}
	}
}
