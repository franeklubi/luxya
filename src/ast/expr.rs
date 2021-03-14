use crate::{ast::stmt::*, token::Token};
use std::rc::Rc;

#[derive(Clone, PartialEq)]
pub enum LiteralValue {
	String(Rc<str>),
	Number(f64),
	True,
	False,
	Nil,
}

pub struct FunctionValue {
	pub keyword: Token,
	pub name: Option<Token>,
	pub params: Option<Vec<Token>>,
	pub body: Option<Box<Stmt>>,
}

pub struct CallValue {
	pub calee: Box<Expr>,
	pub closing_paren: Token,
	pub arguments: Vec<Expr>,
}

pub struct AssignmentValue {
	pub name: Token,
	pub value: Box<Expr>,
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
	pub name: Token,
}

pub enum Expr {
	Function(FunctionValue),
	Call(CallValue),
	Assignment(AssignmentValue),
	Binary(BinaryValue),
	Grouping(GroupingValue),
	Literal(LiteralValue),
	Unary(UnaryValue),
	Identifier(IdentifierValue),
}
