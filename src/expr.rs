use crate::token;

pub enum Expr {
	// left expression, operator, right expression
	Binary(Box<Expr>, token::TokenType, Box<Expr>),

	Grouping(Box<Expr>),
	Literal(LiteralValue),

	// operator, right expression
	Unary(token::TokenType, Box<Expr>),
}

enum LiteralValue {
	String(String),
	Number(i32),
}
