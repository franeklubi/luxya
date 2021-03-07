use crate::ast::interpreter::*;
use crate::token::*;

use std::collections::HashMap;

pub struct InterpreterEnvironmentScope<'a> {
	environment: &'a mut InterpreterEnvironment,
}

impl Drop for InterpreterEnvironmentScope<'_> {
	fn drop(&mut self) {
		self.environment.scopes.pop();
	}
}

pub struct InterpreterEnvironment {
	scopes: Vec<HashMap<String, DeclaredValue>>,
}

impl InterpreterEnvironment {
	pub fn new() -> Self {
		Self { scopes: Vec::new() }
	}

	#[allow(dead_code)]
	pub fn acquire_scope(&mut self) -> InterpreterEnvironmentScope {
		self.scopes.push(HashMap::new());

		InterpreterEnvironmentScope { environment: self }
	}
}

impl InterpreterEnvironmentScope<'_> {
	pub fn nest(&mut self) -> InterpreterEnvironmentScope {
		self.environment.acquire_scope()
	}

	/// Returns a reference to the value if exists in any node
	pub fn get(
		&self,
		identifier: &Token,
	) -> Result<&DeclaredValue, RuntimeError> {
		let name = assume_identifier(&identifier);

		for map in self.environment.scopes.iter().rev() {
			if let Some(dv) = map.get(name) {
				return Ok(dv);
			}
		}

		Err(no_identifier(identifier, name))
	}

	/// Returns a mutable reference to the value if exists in any node
	pub fn get_mut(
		&mut self,
		identifier: &Token,
	) -> Result<&mut DeclaredValue, RuntimeError> {
		let name = assume_identifier(&identifier);

		for map in self.environment.scopes.iter_mut().rev() {
			if let Some(dv) = map.get_mut(name) {
				return Ok(dv);
			}
		}

		Err(no_identifier(identifier, name))
	}

	/// Declares value in the current node
	pub fn declare(
		&mut self,
		name: String,
		value: DeclaredValue,
	) -> Option<DeclaredValue> {
		self.environment
			.scopes
			.last_mut()
			.unwrap()
			.insert(name, value)
	}
}

#[allow(dead_code)]
fn no_identifier(token: &Token, name: &str) -> RuntimeError {
	RuntimeError {
		token: token.clone(),
		message: format!("Identifier {} not defined", name),
	}
}
