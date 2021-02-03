use crate::ast::expr::*;
use crate::token::*;

impl From<bool> for InterpreterValue {
	fn from(v: bool) -> Self {
		if v {
			LiteralValue::True
		} else {
			LiteralValue::False
		}
	}
}

type InterpreterValue = LiteralValue;

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

	match (&v.operator.token_type, &left_value, &right_value) {
		(
			TokenType::Minus,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok(InterpreterValue::Number(n1 - n2)),
		(
			TokenType::Slash,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok(InterpreterValue::Number(n1 / n2)),
		(
			TokenType::Star,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok(InterpreterValue::Number(n1 * n2)),
		(
			TokenType::Plus,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok(InterpreterValue::Number(n1 + n2)),
		(
			TokenType::Plus,
			LiteralValue::String(s1),
			LiteralValue::String(s2),
		) => Ok(InterpreterValue::String(s1.to_owned() + s2)),
		(
			TokenType::Greater,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok((n1 > n2).into()),
		(
			TokenType::GreaterEqual,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok((n1 >= n2).into()),
		(
			TokenType::Less,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok((n1 < n2).into()),
		(
			TokenType::LessEqual,
			LiteralValue::Number(n1),
			LiteralValue::Number(n2),
		) => Ok((n1 <= n2).into()),
		(TokenType::BangEqual, _, _) => Ok((left_value != right_value).into()),
		(TokenType::EqualEqual, _, _) => Ok((left_value == right_value).into()),

		// same kind of error as up above, but with binary explanation
		_ => Err(()),
	}
}
