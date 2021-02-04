use crate::ast::expr::*;
use crate::token::*;

use std::fmt;


type InterpreterValue = LiteralValue;

impl From<bool> for InterpreterValue {
	fn from(v: bool) -> Self {
		if v {
			LiteralValue::True
		} else {
			LiteralValue::False
		}
	}
}

impl fmt::Display for InterpreterValue {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			LiteralValue::String(s) => write!(f, "{:?}", s),
			LiteralValue::Number(n) => write!(f, "{}", n),
			LiteralValue::Nil => write!(f, "nil"),
			LiteralValue::True => write!(f, "true"),
			LiteralValue::False => write!(f, "false"),
		}
	}
}

// TODO: later, use other enum than LiteralValue
// but it'll do for now
pub fn evaluate(expr: &Expr) -> Result<InterpreterValue, ()> {
	match expr {
		Expr::Literal(v) => Ok(v.clone()),
		Expr::Grouping(v) => evaluate(&v.expression),
		Expr::Unary(v) => eval_unary(v),
		Expr::Binary(v) => eval_binary(v),
	}
}

fn eval_unary(v: &UnaryValue) -> Result<InterpreterValue, ()> {
	let right_value = evaluate(&v.right)?;

	match (&v.operator.token_type, &right_value) {
		(TokenType::Minus, LiteralValue::Number(n)) => {
			Ok(InterpreterValue::Number(-n))
		}
		(TokenType::Bang, LiteralValue::True) => Ok(InterpreterValue::False),
		(TokenType::Bang, LiteralValue::False) => Ok(InterpreterValue::True),

		// TODO: error on how we cant do this to this and etc
		_ => Err(()),
	}
}

fn eval_binary(v: &BinaryValue) -> Result<InterpreterValue, ()> {
	let left_value = evaluate(&v.left)?;
	let right_value = evaluate(&v.right)?;

	// im sorry for this, but i found that the nested matches require
	// much simpler patterns,
	// and with this, i can achieve less comparisons overall
	match v.operator.token_type {
		TokenType::BangEqual => Ok((left_value != right_value).into()),
		TokenType::EqualEqual => Ok((left_value == right_value).into()),

		_ => match (&left_value, &right_value) {
			(LiteralValue::Number(n1), LiteralValue::Number(n2)) => {
				match v.operator.token_type {
					TokenType::Minus => Ok(InterpreterValue::Number(n1 - n2)),
					TokenType::Slash => Ok(InterpreterValue::Number(n1 / n2)),
					TokenType::Star => Ok(InterpreterValue::Number(n1 * n2)),
					TokenType::Plus => Ok(InterpreterValue::Number(n1 + n2)),
					TokenType::Greater => Ok((n1 > n2).into()),
					TokenType::GreaterEqual => Ok((n1 >= n2).into()),
					TokenType::Less => Ok((n1 < n2).into()),
					TokenType::LessEqual => Ok((n1 <= n2).into()),

					_ => Err(()),
				}
			}
			(LiteralValue::String(s1), LiteralValue::String(s2)) => {
				if v.operator.token_type == TokenType::Plus {
					Ok(InterpreterValue::String(s1.to_owned() + s2))
				} else {
					Err(())
				}
			}

			// error bby
			_ => Err(()),
		},
	}
}
