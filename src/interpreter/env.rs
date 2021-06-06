use super::{
	helpers::assume_identifier,
	types::{InterpreterValue, RuntimeError},
};
use crate::{
	env::{DeclaredValue, EnvironmentBase, EnvironmentWrapper},
	token::Token,
	unwrap_scope_mut,
};

use std::{cell::RefCell, rc::Rc};


#[derive(Clone)]
pub struct InterpreterEnvironment(
	Rc<
		RefCell<
			EnvironmentBase<
				InterpreterEnvironment,
				DeclaredValue<InterpreterValue>,
			>,
		>,
	>,
);

impl PartialEq for InterpreterEnvironment {
	fn eq(&self, other: &Self) -> bool {
		Rc::ptr_eq(&self.0, &other.0)
	}
}

impl EnvironmentWrapper<InterpreterValue> for InterpreterEnvironment {
	fn new() -> Self {
		Self(Rc::new(RefCell::new(EnvironmentBase::new(None))))
	}

	fn fork(&self) -> Self {
		Self(Rc::new(RefCell::new(EnvironmentBase::new(Some(
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
				let enclosing =
					borrowed.enclosing.as_ref().expect("Enclosing environment");

				enclosing.0.clone()
			};

			scope = new_scope;
		}

		let name = assume_identifier(identifier);

		let borrowed = scope.borrow();

		Ok(borrowed.scope.get(name).expect("Identifier").clone())
	}

	fn declare(
		&self,
		name: String,
		value: DeclaredValue<InterpreterValue>,
	) -> Option<DeclaredValue<InterpreterValue>> {
		unwrap_scope_mut!(self).insert(name, value)
	}

	fn assign(
		&self,
		steps: u32,
		identifier: &Token,
		value: InterpreterValue,
	) -> Result<InterpreterValue, RuntimeError> {
		let mut scope: Rc<RefCell<EnvironmentBase<_, _>>> = self.0.clone();

		for _ in 0..steps {
			let new_scope = {
				let borrowed = scope.borrow_mut();
				let enclosing = borrowed
					.enclosing
					.as_ref()
					.expect("The enclosing environment to exist");

				enclosing.0.clone()
			};

			scope = new_scope;
		}

		let name = assume_identifier(identifier);

		let mut borrowed = scope.borrow_mut();

		let entry = borrowed
			.scope
			.get_mut(name)
			.expect("The identifier to be there");

		if entry.mutable {
			*entry = DeclaredValue {
				mutable: entry.mutable,
				value: value.clone(),
			};

			Ok(value)
		} else {
			Err(RuntimeError {
				message: format!(
					"Cannot reassign a const {} `{}`",
					entry.value.human_type(),
					name
				),
				token: identifier.clone(),
			})
		}
	}
}
