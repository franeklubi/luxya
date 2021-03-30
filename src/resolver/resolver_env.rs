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

use std::{cell::RefCell, collections::HashMap, rc::Rc};


// Everything we need to create resolved map will have to be inside this env
#[derive(Clone)]
pub struct ResolverEnvironment {
	// we need it to be always accessible in resolver
	nest_levels: Rc<HashMap<Expr, u32>>,

	// true if variable, false if const
	env: Rc<RefCell<EnvironmentBase<ResolverEnvironment, bool>>>,

	// current nest level
	level: i32,
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
			nest_levels: Rc::new(HashMap::new()),
			env: Rc::new(RefCell::new(EnvironmentBase::new(None))),
			level: 0,
		}
	}

	fn fork(&self) -> Self {
		ResolverEnvironment {
			nest_levels: Rc::clone(&self.nest_levels),
			env: Rc::new(RefCell::new(EnvironmentBase::new(Some(
				self.clone(),
			)))),
			level: self.level + 1,
		}
	}

	fn read(
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
			enclosing.read(identifier)
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
		let entry = self.read(identifier)?;

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
		expr: &Expr,
		identifier: &Token,
	) -> Result<(), RuntimeError> {
		let name = assume_identifier(&identifier);

		if let Some(_dv) = resolver_unwrap_scope!(self).get(name) {
			unimplemented!("resolve nest level worker at level: {}", self.level)
		} else if let Some(enclosing) = resolver_unwrap_enclosing!(self) {
			enclosing.resolve_nest_level(expr, identifier)?;

			Ok(())
		} else {
			Err(no_identifier(identifier, name))
		}
	}
}
