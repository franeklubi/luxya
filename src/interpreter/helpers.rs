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
