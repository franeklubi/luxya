use crate::interpreter::interpreter_env::InterpreterEnvironment;

#[derive(Clone)]
pub struct ResolverEnvironment {
	env: InterpreterEnvironment,
	level: i32,
}

impl ResolverEnvironment {
	pub fn new() -> Self {
		ResolverEnvironment {
			env: InterpreterEnvironment::new(),
			level: 0,
		}
	}

	pub fn fork(&self) -> Self {
		ResolverEnvironment {
			env: self.env.fork(),
			level: self.level + 1,
		}
	}

	// pub fn read_probe(
	// 	&self,
	// 	identifier: &Token,
	// ) -> Result<(i32, DeclaredValue), RuntimeError> {
	// 	let name = assume_identifier(&identifier);
	//
	// if let Some(dv) = unwrap_scope!(self).get(name) {
	// 	Ok(dv.clone())
	// } else if let Some(enclosing) = unwrap_enclosing!(self) {
	// 	enclosing.read(identifier)
	// } else {
	// 	Err(no_identifier(identifier, name))
	// }
	// }
	// pub fn declare(
	// 	&self,
	// 	name: String,
	// 	value: DeclaredValue,
	// ) -> Option<DeclaredValue> {
	// 	unwrap_scope_mut!(self).insert(name, value)
	// }
	//
	// pub fn assign(
	// 	&self,
	// 	identifier: &Token,
	// 	value: InterpreterValue,
	// ) -> Result<InterpreterValue, RuntimeError> {
	// 	let name = assume_identifier(&identifier);
	//
	// 	if let Some(entry) = unwrap_scope_mut!(self).get_mut(name) {
	// 		return if entry.mutable {
	// 			*entry = DeclaredValue {
	// 				mutable: entry.mutable,
	// 				value: value.clone(),
	// 			};
	//
	// 			Ok(value)
	// 		} else {
	// 			Err(RuntimeError {
	// 				message: format!(
	// 					"Cannot assign to a const {} `{}`",
	// 					entry.value.to_human_readable(),
	// 					name
	// 				),
	// 				token: identifier.clone(),
	// 			})
	// 		};
	// 	}
	//
	// 	// not doing an `else if` on purpose, because we want the borrow in the
	// 	// upper `if` statement to be dropped
	// 	if let Some(enclosing) = unwrap_enclosing!(self) {
	// 		enclosing.assign(identifier, value)
	// 	} else {
	// 		Err(no_identifier(identifier, name))
	// 	}
	// }
}
