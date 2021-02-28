use crate::token::Token;

#[derive(Clone, PartialEq)]
pub enum LiteralValue {
	String(String),
	Number(f64),
	True,
	False,
	Nil,
}

pub struct BinaryValue {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>,
}

pub struct GroupingValue {
	pub expression: Box<Expr>,
}

pub struct UnaryValue {
	pub operator: Token,
	pub right: Box<Expr>,
}

pub struct IdentifierValue {
	pub token: Token,
}

pub enum Expr {
	Binary(BinaryValue),
	Grouping(GroupingValue),
	Literal(LiteralValue),
	Unary(UnaryValue),
	Identifier(IdentifierValue),
}
