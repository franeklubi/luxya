use crate::{ast::stmt::*, token::Token};
use std::{cell::Cell, rc::Rc};

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
	pub body: Option<Rc<Vec<Stmt>>>,
}

pub struct CallValue {
	pub calee: Box<Expr>,
	pub closing_paren: Token,
	pub arguments: Vec<Expr>,
}

pub struct AssignmentValue {
	pub name: Token,
	pub value: Box<Expr>,
	pub env_distance: Cell<u32>,
}

pub struct BinaryValue {
	pub left: Box<Expr>,
	pub operator: Token,
	pub right: Box<Expr>,
}

pub struct GetValue {
	pub getee: Box<Expr>,
	pub key: GetAccessor,
	pub blame: Token,
}

pub struct IdentifierValue {
	pub name: Token,
	pub env_distance: Cell<u32>,
}

pub struct UnaryValue {
	pub operator: Token,
	pub right: Box<Expr>,
}

pub struct GroupingValue {
	pub expression: Box<Expr>,
}

pub enum Expr {
	Function(FunctionValue),
	Call(CallValue),
	Assignment(AssignmentValue),
	Binary(BinaryValue),
	Get(GetValue),
	Identifier(IdentifierValue),
	Unary(UnaryValue),
	Grouping(GroupingValue),
	Literal(LiteralValue),
}

pub enum GetAccessor {
	Name(Token),
	Eval(Box<Expr>),
}
