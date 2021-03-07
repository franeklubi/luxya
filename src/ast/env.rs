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
			Err(RuntimeError {
				token: identifier.clone(),
				message: format!("Identifier {} not defined", name),
			})
		}
	}

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
			Err(RuntimeError {
				token: identifier.clone(),
				message: format!("Identifier {} not defined", name),
			})
		}
	}

	pub fn contains_key(&self, name: &str) -> bool {
		if self.current.contains_key(name) {
			true
		} else if let Some(p) = &self.parent {
			p.contains_key(name)
		} else {
			false
		}
	}

	pub fn insert(
		&mut self,
		name: String,
		value: DeclaredValue,
	) -> Option<DeclaredValue> {
		self.current.insert(name, value)
	}
}
