use crate::token::{self, TokenType};

pub enum LiteralValue {
	String(String),
	Number(i32),
	True,
	False,
	Nil,
}

pub struct BinaryValue {
	left: Box<Expr>,
	operator: TokenType,
	right: Box<Expr>,
}

pub struct GroupingValue {
	expression: Box<Expr>,
}

pub struct UnaryValue {
	operator: TokenType,
	right: Box<Expr>,
}

pub enum Expr {
	Binary(BinaryValue),
	Grouping(GroupingValue),
	Literal(LiteralValue),
	Unary(UnaryValue),
}
