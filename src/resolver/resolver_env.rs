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
		steps: u32,
		identifier: &Token,
	) -> Result<DeclaredValue<InterpreterValue>, RuntimeError> {
		let mut scope: Rc<RefCell<EnvironmentBase<_, _>>> = self.env.clone();

		for _ in 0..steps {
			let new_scope = {
				let borrowed = scope.borrow();
				let enclosing = borrowed
					.enclosing
					.as_ref()
					.expect("The enclosing environment to exist");

				enclosing.env.clone()
			};

			scope = new_scope;
		}

		let name = assume_identifier(identifier);

		let borrowed = scope.borrow();

		Ok(DeclaredValue {
			mutable: *borrowed
				.scope
				.get(name)
				.expect("The identifier to be there"),
			value: InterpreterValue::Nil,
		})
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
		steps: u32,
		identifier: &Token,
		_value: InterpreterValue,
	) -> Result<InterpreterValue, RuntimeError> {
		let entry = self.read(steps, identifier)?;

		if !entry.mutable {
			let name = assume_identifier(identifier);

			Err(RuntimeError {
				message: format!("Cannot reassign a const `{}`", name),
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
		resolvable_token: &Token,
	) -> Result<(), RuntimeError> {
		self.resolve_nest_level_worker(
			self.level,
			resolvable_node,
			resolvable_token,
		)
	}

	fn resolve_nest_level_worker(
		&self,
		initial_level: u32,
		resolvable_node: &Expr,
		resolvable_token: &Token,
	) -> Result<(), RuntimeError> {
		let name = assume_identifier(resolvable_token);

		if let Some(_dv) = resolver_unwrap_scope!(self).get(name) {
			let env_distance = assume_resolvable_expr(resolvable_node);

			env_distance.set(initial_level - self.level);

			Ok(())
		} else if let Some(enclosing) = resolver_unwrap_enclosing!(self) {
			enclosing.resolve_nest_level_worker(
				initial_level,
				resolvable_node,
				resolvable_token,
			)?;

			Ok(())
		} else {
			Err(no_identifier(resolvable_token, name))
		}
	}
}
