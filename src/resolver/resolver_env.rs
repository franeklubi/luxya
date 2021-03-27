use crate::{
	env::*,
	interpreter::{
		interpreter_env::InterpreterEnvironment,
		types::RuntimeError,
	},
	token::Token,
};

#[derive(Clone)]
pub struct ResolverEnvironment {
	env: InterpreterEnvironment,
	level: i32,
}

impl EnvironmentWrapper<bool> for ResolverEnvironment {
	fn new() -> Self {
		ResolverEnvironment {
			env: InterpreterEnvironment::new(),
			level: 0,
		}
	}

	fn fork(&self) -> Self {
		ResolverEnvironment {
			env: self.env.fork(),
			level: self.level + 1,
		}
	}

	fn read(
		&self,
		_identifier: &Token,
	) -> Result<DeclaredValue<bool>, RuntimeError> {
		unimplemented!()
	}

	fn declare(
		&self,
		_name: String,
		_value: DeclaredValue<bool>,
	) -> Option<DeclaredValue<bool>> {
		unimplemented!()
	}

	fn assign(
		&self,
		_identifier: &Token,
		_value: bool,
	) -> Result<bool, RuntimeError> {
		unimplemented!()
	}
}
