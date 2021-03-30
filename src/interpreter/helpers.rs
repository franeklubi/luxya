use super::types::*;
use crate::token::*;


// A shorthand way to extract identifier's name
pub fn assume_identifier(t: &Token) -> &str {
	if let TokenType::Identifier(i) = &t.token_type {
		i
	} else {
		unreachable!("Couldn't extract identifier. This shouldn't happen")
	}
}

pub fn guard_function(
	ibv: InterpreterStmtValue<InterpreterValue>,
) -> Result<InterpreterValue, RuntimeError> {
	match ibv {
		InterpreterStmtValue::Break(token) => Err(RuntimeError {
			message: "Cannot use `break` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Continue(token) => Err(RuntimeError {
			message: "Cannot use `continue` outside of a loop".into(),
			token,
		}),
		InterpreterStmtValue::Return { value, .. } => Ok(value),
		InterpreterStmtValue::Noop => Ok(InterpreterValue::Nil),
	}
}

pub fn no_identifier(token: &Token, name: &str) -> RuntimeError {
	RuntimeError {
		token: token.clone(),
		message: format!("Identifier {} not defined", name),
	}
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
