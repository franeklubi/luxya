use crate::{
	ast::expr::Expr,
	env::*,
	interpreter::types::{InterpreterValue, RuntimeError},
	token::Token,
};

use std::{cell::RefCell, collections::HashMap, rc::Rc};


// Everything we need to create resolved map will have to be inside this env
#[derive(Clone)]
pub struct ResolverEnvironment {
	// we need it to be always accessible in resolver
	nest_levels: Rc<HashMap<Expr, u32>>,

	// true if variable, false if const
	env: Rc<RefCell<EnvironmentBase<ResolverEnvironment, bool>>>,

	// current nest level
	level: i32,
}

// The InterpreterValue in this implementation tells us basically nothing, as
// we won't be resolving the true values of our nodes.
//
// It's just there to satisfy EnvironmentWrapper and a couple of statement
// functions in interpreter/statements.
//
// I'll always supply Nil here
impl EnvironmentWrapper<InterpreterValue> for ResolverEnvironment {
	fn new() -> Self {
		ResolverEnvironment {
			nest_levels: Rc::new(HashMap::new()),
			env: Rc::new(RefCell::new(EnvironmentBase::new(None))),
			level: 0,
		}
	}

	fn fork(&self) -> Self {
		ResolverEnvironment {
			nest_levels: Rc::clone(&self.nest_levels),
			env: Rc::new(RefCell::new(EnvironmentBase::new(Some(
				self.clone(),
			)))),
			level: self.level + 1,
		}
	}

	fn read(
		&self,
		_identifier: &Token,
	) -> Result<DeclaredValue<InterpreterValue>, RuntimeError> {
		unimplemented!("read")
	}

	fn declare(
		&self,
		name: String,
		value: DeclaredValue<InterpreterValue>,
	) -> Option<DeclaredValue<InterpreterValue>> {
		self.env.borrow_mut().scope.insert(name, value.mutable);

		None
	}

	fn assign(
		&self,
		_identifier: &Token,
		_value: InterpreterValue,
	) -> Result<InterpreterValue, RuntimeError> {
		unimplemented!("assignment")
	}
}

impl ResolverEnvironment {
	pub fn resolve_nest_level(
		&self,
		_expr: &Expr,
		_name: &Token,
	) -> Result<(), RuntimeError> {
		unimplemented!("nest level resolver")
	}
}
