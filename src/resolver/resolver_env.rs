use super::helpers::assume_resolvable_expr;
use crate::{
	ast::expr::Expr,
	env::*,
	interpreter::{
		helpers::assume_identifier,
		types::{InterpreterValue, RuntimeError},
	},
	token::Token,
	unwrap_enclosing,
	unwrap_scope,
	unwrap_scope_mut,
};

use std::{cell::RefCell, rc::Rc};


// Everything we need to create resolved map will have to be inside this env
#[derive(Clone)]
pub struct ResolverEnvironment(
	// true if variable, false if const
	pub Rc<RefCell<EnvironmentBase<ResolverEnvironment, bool>>>,
);

// The InterpreterValue in this implementation tells us basically nothing, as
// we won't be resolving the true values of our nodes.
//
// It's just there to satisfy EnvironmentWrapper and a couple of statement
// functions in interpreter/statements.
//
// I'll always supply Nil here
impl EnvironmentWrapper<InterpreterValue> for ResolverEnvironment {
	fn new() -> Self {
		ResolverEnvironment(Rc::new(RefCell::new(EnvironmentBase::new(None))))
	}

	fn fork(&self) -> Self {
		ResolverEnvironment(Rc::new(RefCell::new(EnvironmentBase::new(Some(
			self.clone(),
		)))))
	}

	fn read(
		&self,
		steps: u32,
		identifier: &Token,
	) -> Result<DeclaredValue<InterpreterValue>, RuntimeError> {
		let mut scope: Rc<RefCell<EnvironmentBase<_, _>>> = self.0.clone();

		for _ in 0..steps {
			let new_scope = {
				let borrowed = scope.borrow();
				let enclosing = borrowed
					.enclosing
					.as_ref()
					.expect("The enclosing environment to exist");

				enclosing.0.clone()
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
		unwrap_scope_mut!(self).insert(name, value.mutable);

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
		self.resolve_nest_level_worker(0, resolvable_node, resolvable_token)
	}

	fn resolve_nest_level_worker(
		&self,
		curr_distance: u32,
		resolvable_node: &Expr,
		resolvable_token: &Token,
	) -> Result<(), RuntimeError> {
		let name = assume_identifier(resolvable_token);

		if let Some(_dv) = unwrap_scope!(self).get(name) {
			let env_distance = assume_resolvable_expr(resolvable_node);

			env_distance.set(curr_distance);

			Ok(())
		} else if let Some(enclosing) = unwrap_enclosing!(self) {
			enclosing.resolve_nest_level_worker(
				curr_distance + 1,
				resolvable_node,
				resolvable_token,
			)?;

			Ok(())
		} else {
			Err(RuntimeError {
				token: resolvable_token.clone(),
				message: format!("Identifier `{}` not defined", name),
			})
		}
	}
}
