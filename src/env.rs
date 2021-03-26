use std::collections::HashMap;


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
