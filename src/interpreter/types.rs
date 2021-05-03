use super::interpreter_env::*;
use crate::{ast::expr::*, token::*};

use std::{cell::RefCell, collections::HashMap, fmt, rc::Rc};


const MAX_LIST_VALUES_PRINT: usize = 100;

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
		class: Option<Rc<InterpreterValue>>,
		properties: Rc<RefCell<HashMap<String, InterpreterValue>>>,
	},
	Class {
		superclass: Option<Rc<InterpreterValue>>,
		constructor: Option<Rc<InterpreterValue>>,
		name: Rc<str>,
		methods: Rc<HashMap<String, InterpreterValue>>,
	},
	List(Rc<RefCell<Vec<InterpreterValue>>>),
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

	pub fn gen_repr(&self, nested: bool) -> String {
		match self {
			InterpreterValue::List(l) => {
				let take_amount =
					if nested { 0 } else { MAX_LIST_VALUES_PRINT };

				let l_borrow = l.borrow();

				let mut list_repr = String::from("[ ");

				list_repr += &l_borrow
					.iter()
					.take(take_amount)
					.map(|v| v.gen_repr(true))
					.collect::<Vec<String>>()
					.join(", ");

				let list_len = l_borrow.len();

				if list_len > take_amount {
					list_repr += &format!(
						"{}...{} hidden ]",
						if nested { "" } else { ", " },
						list_len - take_amount
					);
				} else {
					list_repr += " ]";
				}

				list_repr
			}
			InterpreterValue::Instance { class, .. } => {
				if let Some(class) = class {
					format!("instance of {}", class)
				} else {
					String::from("object")
				}
			}
			InterpreterValue::Class { name, .. } => format!("class {}", name),
			InterpreterValue::Function { .. } => String::from("function"),
			InterpreterValue::String(s) => format!("{}", s),
			InterpreterValue::Number(n) => format!("{}", n),
			InterpreterValue::Char(c) => format!("{}", c),
			InterpreterValue::False => String::from("false"),
			InterpreterValue::True => String::from("true"),
			InterpreterValue::Nil => String::from("nil"),
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
		write!(f, "{}", self.gen_repr(false))
	}
}
