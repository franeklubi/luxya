use crate::ast::{expr::*, stmt::*};
use crate::token::*;

use std::fmt;

pub struct RuntimeError {
	pub message: String,
	pub token: Token,
}

// TODO: later, use other enum than LiteralValue
// but it'll do for now
//
// IDEAS TODO:
// - LiteralValue should know if it's a child of some identifier or smth
// - If it is /\, then we can print the identifier, rather than it's value
// - Should mimic LiteralValue's fields
type InterpreterValue = LiteralValue;

impl From<bool> for InterpreterValue {
	fn from(v: bool) -> Self {
		if v {
			InterpreterValue::True
		} else {
			InterpreterValue::False
		}
	}
}

impl fmt::Display for InterpreterValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			InterpreterValue::String(s) => write!(f, "{:?}", s),
			InterpreterValue::Number(n) => write!(f, "{}", n),
			InterpreterValue::Nil => write!(f, "nil"),
			InterpreterValue::True => write!(f, "true"),
			InterpreterValue::False => write!(f, "false"),
		}
	}
}

pub fn interpret(statements: &[Stmt]) {
	statements.iter().enumerate().for_each(|(index, stmt)| {
		if let Err(e) = evaluate(&stmt) {
			println!("Error [{}]:\n\t{}", index, e.message)
		}
	});
}

fn evaluate(stmt: &Stmt) -> Result<InterpreterValue, RuntimeError> {
	match stmt {
		Stmt::Expression(v) => eval_expression(&v.expression),
		Stmt::Print(v) => {
			let evaluated = eval_expression(&v.expression);

			if let Ok(value) = &evaluated {
				println!("{}", value);
			}

			evaluated
		}
		Stmt::Declaration(_v) => unimplemented!("VARIABLE DECLARATION"),
	}
}

fn eval_expression(expr: &Expr) -> Result<InterpreterValue, RuntimeError> {
	match expr {
		Expr::Literal(v) => Ok(v.clone()),
		Expr::Grouping(v) => eval_expression(&v.expression),
		Expr::Unary(v) => eval_unary(v),
		Expr::Binary(v) => eval_binary(v),
		Expr::Identifier(_v) => unimplemented!("IDENTIFIER DEREFERENCE"),
	}
}

fn eval_unary(v: &UnaryValue) -> Result<InterpreterValue, RuntimeError> {
	let right_value = eval_expression(&v.right)?;

	match (&v.operator.token_type, &right_value) {
		(TokenType::Minus, InterpreterValue::Number(n)) => {
			Ok(InterpreterValue::Number(-n))
		}
		(TokenType::Bang, InterpreterValue::True) => {
			Ok(InterpreterValue::False)
		}
		(TokenType::Bang, InterpreterValue::False) => {
			Ok(InterpreterValue::True)
		}

		// TODO: error on how we cant do this to this and etc
		_ => Err(RuntimeError {
			message: format!(
				"Cannot use `{}` on `{}`",
				v.operator.token_type, right_value
			),
			token: v.operator.clone(),
		}),
	}
}

fn eval_binary(v: &BinaryValue) -> Result<InterpreterValue, RuntimeError> {
	let left_value = eval_expression(&v.left)?;
	let right_value = eval_expression(&v.right)?;

	// im sorry for this, but i found that the nested matches require
	// much simpler patterns,
	// and with this, i can achieve less comparisons overall
	match v.operator.token_type {
		TokenType::BangEqual => Ok((left_value != right_value).into()),
		TokenType::EqualEqual => Ok((left_value == right_value).into()),

		_ => match (&left_value, &right_value) {
			(InterpreterValue::Number(n1), InterpreterValue::Number(n2)) => {
				match v.operator.token_type {
					TokenType::Minus => Ok(InterpreterValue::Number(n1 - n2)),
					TokenType::Slash => Ok(InterpreterValue::Number(n1 / n2)),
					TokenType::Star => Ok(InterpreterValue::Number(n1 * n2)),
					TokenType::Plus => Ok(InterpreterValue::Number(n1 + n2)),
					TokenType::Greater => Ok((n1 > n2).into()),
					TokenType::GreaterEqual => Ok((n1 >= n2).into()),
					TokenType::Less => Ok((n1 < n2).into()),
					TokenType::LessEqual => Ok((n1 <= n2).into()),

					_ => unreachable!("Scanner did a bad job 😎."),
				}
			}
			(InterpreterValue::String(s1), InterpreterValue::String(s2)) => {
				if v.operator.token_type == TokenType::Plus {
					Ok(InterpreterValue::String(s1.to_owned() + s2))
				} else {
					Err(RuntimeError {
						message: format!(
							"You cannot use `{}` on two strings. Did you mean \
							 `+`?",
							v.operator.token_type
						),
						token: v.operator.clone(),
					})
				}
			}

			// error bby
			_ => Err(RuntimeError {
				message: format!(
					"Cannot use `{}` on `{}` and `{}`",
					v.operator.token_type, left_value, right_value
				),
				token: v.operator.clone(),
			}),
		},
	}
}
