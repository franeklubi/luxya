use crate::ast::interpreter::*;
use crate::token::*;

use std::collections::HashMap;

pub type InterpreterEnvironment<'a> = &'a mut EnvironmentHolder;

pub struct EnvironmentHolder {
	parent: Option<Box<EnvironmentHolder>>,
	current: HashMap<String, DeclaredValue>,
}

impl EnvironmentHolder {
	pub fn new() -> Self {
		Self {
			parent: None,
			current: HashMap::new(),
		}
	}

	/// Returns a reference to the value if exists in any node
	pub fn get(
		&self,
		identifier: &Token,
	) -> Result<&DeclaredValue, RuntimeError> {
		let name = assume_identifier(&identifier);

		if let Some(v) = self.current.get(name) {
			Ok(v)
		} else if let Some(p) = &self.parent {
			p.get(identifier)
		} else {
			Err(no_identifier(identifier, name))
		}
	}

	/// Returns a mutable reference to the value if exists in any node
	pub fn get_mut(
		&mut self,
		identifier: &Token,
	) -> Result<&mut DeclaredValue, RuntimeError> {
		let name = assume_identifier(&identifier);

		if let Some(v) = self.current.get_mut(name) {
			Ok(v)
		} else if let Some(p) = &mut self.parent {
			p.get_mut(identifier)
		} else {
			Err(no_identifier(identifier, name))
		}
	}

	/// Declares value in the current node
	pub fn declare(
		&mut self,
		name: String,
		value: DeclaredValue,
	) -> Option<DeclaredValue> {
		self.current.insert(name, value)
	}
}

fn no_identifier(token: &Token, name: &str) -> RuntimeError {
	RuntimeError {
		token: token.clone(),
		message: format!("Identifier {} not defined", name),
	}
}
