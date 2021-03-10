use crate::ast::interpreter::*;
use crate::token::*;

use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct InterpreterEnvironment {
	enclosing: Option<WrappedInterpreterEnvironment>,
	scope: HashMap<String, DeclaredValue>,
}

impl InterpreterEnvironment {
	pub fn new(enclosing: Option<WrappedInterpreterEnvironment>) -> Self {
		Self {
			scope: HashMap::new(),
			enclosing,
		}
	}

	pub fn wrap(self) -> WrappedInterpreterEnvironment {
		WrappedInterpreterEnvironment(Rc::new(RefCell::new(self)))
	}
}

#[derive(Clone)]
pub struct WrappedInterpreterEnvironment(Rc<RefCell<InterpreterEnvironment>>);

macro_rules! unwrap_scope {
	($wie:expr) => {{
		&$wie.0.borrow().scope
	}};
}

macro_rules! unwrap_scope_mut {
	($wie:expr) => {{
		&mut $wie.0.borrow_mut().scope
	}};
}

macro_rules! unwrap_enclosing {
	($wie:expr) => {{
		&$wie.0.borrow().enclosing
	}};
}

impl WrappedInterpreterEnvironment {
	pub fn fork(&self) -> Self {
		InterpreterEnvironment::new(Some(self.clone())).wrap()
	}

	pub fn read(
		&self,
		identifier: &Token,
	) -> Result<DeclaredValue, RuntimeError> {
		let name = assume_identifier(&identifier);

		if let Some(dv) = unwrap_scope!(self).get(name) {
			Ok(dv.clone())
		} else if let Some(enclosing) = unwrap_enclosing!(self) {
			enclosing.read(identifier)
		} else {
			Err(no_identifier(identifier, name))
		}
	}

	pub fn declare(
		&self,
		name: String,
		value: DeclaredValue,
	) -> Option<DeclaredValue> {
		unwrap_scope_mut!(self).insert(name, value)
	}

	pub fn assign(
		&self,
		identifier: &Token,
		value: InterpreterValue,
	) -> Result<InterpreterValue, RuntimeError> {
		let name = assume_identifier(&identifier);

		if let Some(entry) = unwrap_scope_mut!(self).get_mut(name) {
			return if entry.mutable {
				*entry = DeclaredValue {
					mutable: entry.mutable,
					value: value.clone(),
				};

				Ok(value)
			} else {
				Err(RuntimeError {
					message: format!("Cannot assign to a const `{}`", name),
					token: identifier.clone(),
				})
			};
		}

		// not doing an `else if` on purpose, because we want the borrow in the
		// upper `if` statement to be dropped
		if let Some(enclosing) = unwrap_enclosing!(self) {
			enclosing.assign(identifier, value)
		} else {
			Err(no_identifier(identifier, name))
		}
	}
}

fn no_identifier(token: &Token, name: &str) -> RuntimeError {
	RuntimeError {
		token: token.clone(),
		message: format!("Identifier {} not defined", name),
	}
}
