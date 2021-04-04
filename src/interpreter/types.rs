use super::interpreter_env::*;
use crate::{ast::expr::*, token::*};

use std::{collections::HashMap, fmt, rc::Rc};


pub struct RuntimeError {
	pub message: String,
	pub token: Token,
}

#[derive(Clone, PartialEq)]
pub enum InterpreterValue {
	Function {
		fun: Rc<InterpreterFunction>,
		enclosing_env: InterpreterEnvironment,
	},
	Instance {
		class: Rc<InterpreterValue>,
		properties: HashMap<String, InterpreterValue>,
	},
	Class {
		name: Rc<str>,
	},
	String(Rc<str>),
	Number(f64),
	True,
	False,
	Nil,
}

impl InterpreterValue {
	pub fn to_human_readable(&self) -> &str {
		match self {
			InterpreterValue::Instance { .. } => "class instance",
			InterpreterValue::Function { .. } => "function",
			InterpreterValue::Class { .. } => "class",
			InterpreterValue::String(_) => "string",
			InterpreterValue::Number(_) => "number",
			InterpreterValue::False => "boolean",
			InterpreterValue::True => "boolean",
			InterpreterValue::Nil => "nil",
		}
	}
}

pub enum InterpreterStmtValue<T> {
	Return { keyword: Token, value: T },
	Break(Token),
	Continue(Token),
	Noop,
}

pub type NativeFunctionSignature = fn(
	&Token,
	&InterpreterEnvironment,
	&[InterpreterValue],
)
	-> Result<InterpreterValue, RuntimeError>;

pub enum InterpreterFunction {
	Native {
		arity: usize,
		fun: NativeFunctionSignature,
	},
	LoxDefined(FunctionValue),
}

impl PartialEq for InterpreterFunction {
	fn eq(&self, other: &Self) -> bool {
		match (&self, &other) {
			(
				InterpreterFunction::LoxDefined(FunctionValue {
					body: Some(body1),
					..
				}),
				InterpreterFunction::LoxDefined(FunctionValue {
					body: Some(body2),
					..
				}),
			) => Rc::ptr_eq(body1, body2),
			_ => false,
		}
	}
}

impl From<bool> for InterpreterValue {
	fn from(v: bool) -> Self {
		if v {
			InterpreterValue::True
		} else {
			InterpreterValue::False
		}
	}
}

impl From<LiteralValue> for InterpreterValue {
	fn from(v: LiteralValue) -> Self {
		match v {
			LiteralValue::String(s) => InterpreterValue::String(Rc::clone(&s)),
			LiteralValue::Number(n) => InterpreterValue::Number(n),
			LiteralValue::True => InterpreterValue::True,
			LiteralValue::False => InterpreterValue::False,
			LiteralValue::Nil => InterpreterValue::Nil,
		}
	}
}

impl fmt::Display for InterpreterValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InterpreterValue::Instance { class, .. } => {
				write!(f, "instance of {}", class)
			}
			InterpreterValue::Class { name } => write!(f, "class {}", name),
			InterpreterValue::Function { .. } => write!(f, "function"),
			InterpreterValue::String(s) => write!(f, "{}", s),
			InterpreterValue::Number(n) => write!(f, "{}", n),
			InterpreterValue::False => write!(f, "false"),
			InterpreterValue::True => write!(f, "true"),
			InterpreterValue::Nil => write!(f, "nil"),
		}
	}
}
