use super::interpreter_env::*;
use crate::{ast::expr::*, token::*};

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};


const MAX_LIST_VALUES_PRINT: usize = 10;

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
		properties: Rc<RefCell<HashMap<String, InterpreterValue>>>,
	},
	Class {
		superclass: Option<Rc<InterpreterValue>>,
		constructor: Option<Rc<InterpreterValue>>,
		name: Rc<str>,
		methods: Rc<HashMap<String, InterpreterValue>>,
	},
	List(Vec<InterpreterValue>),
	String(Rc<str>),
	Number(f64),
	Char(char),
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
			InterpreterValue::List(_) => "list",
			InterpreterValue::Char(_) => "char",
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

// TODO: convert to a method
// (like to_human_readable we already have or something)
impl fmt::Display for InterpreterValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InterpreterValue::List(l) => {
				let mut list_repr = String::from("[ ");

				list_repr += &l
					.iter()
					.take(MAX_LIST_VALUES_PRINT)
					.map(|v| v.to_string())
					.collect::<Vec<String>>()
					.join(", ");

				let list_len = l.len();

				if list_len > MAX_LIST_VALUES_PRINT {
					list_repr += &format!(
						", ...{} hidden ]",
						list_len - MAX_LIST_VALUES_PRINT
					);
				} else {
					list_repr += " ]";
				}

				write!(f, "{}", list_repr)
			}
			InterpreterValue::Instance { class, .. } => {
				write!(f, "instance of {}", class)
			}
			InterpreterValue::Class { name, .. } => write!(f, "class {}", name),
			InterpreterValue::Function { .. } => write!(f, "function"),
			InterpreterValue::String(s) => write!(f, "{}", s),
			InterpreterValue::Number(n) => write!(f, "{}", n),
			InterpreterValue::Char(c) => write!(f, "{}", c),
			InterpreterValue::False => write!(f, "false"),
			InterpreterValue::True => write!(f, "true"),
			InterpreterValue::Nil => write!(f, "nil"),
		}
	}
}
