use crate::{interpreter::types::RuntimeError, token::Token};

use std::collections::HashMap;


#[derive(Clone)]
pub struct DeclaredValue<V> {
	pub mutable: bool,
	pub value: V,
}

pub struct EnvironmentBase<W, V> {
	pub enclosing: Option<W>,
	pub scope: HashMap<String, V>,
}

impl<W, V> EnvironmentBase<W, V> {
	pub fn new(enclosing: Option<W>) -> Self {
		Self {
			enclosing,
			scope: HashMap::new(),
		}
	}
}

pub trait EnvironmentWrapper<V> {
	fn new() -> Self;

	fn fork(&self) -> Self;

	fn read(
		&self,
		steps: u32,
		identifier: &Token,
	) -> Result<DeclaredValue<V>, RuntimeError>;

	fn declare(
		&self,
		name: String,
		value: DeclaredValue<V>,
	) -> Option<DeclaredValue<V>>;

	fn assign(
		&self,
		steps: u32,
		identifier: &Token,
		value: V,
	) -> Result<V, RuntimeError>;
}

#[macro_export]
macro_rules! unwrap_scope {
	($wie:expr) => {{
		&$wie.0.borrow().scope
	}};
}

#[macro_export]
macro_rules! unwrap_scope_mut {
	($wie:expr) => {{
		&mut $wie.0.borrow_mut().scope
	}};
}

#[macro_export]
macro_rules! unwrap_enclosing {
	($wie:expr) => {{
		&$wie.0.borrow().enclosing
	}};
}
