use crate::{interpreter::types::RuntimeError, token::Token};

use std::collections::HashMap;

#[derive(Clone)]
pub struct DeclaredValue<T> {
	pub mutable: bool,
	pub value: T,
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

pub trait EnvironmentWrapper<T> {
	fn new() -> Self;

	fn fork(&self) -> Self;

	fn read(
		&self,
		identifier: &Token,
	) -> Result<DeclaredValue<T>, RuntimeError>;

	fn declare(
		&self,
		name: String,
		value: DeclaredValue<T>,
	) -> Option<DeclaredValue<T>>;

	fn assign(&self, identifier: &Token, value: T) -> Result<T, RuntimeError>;
}
