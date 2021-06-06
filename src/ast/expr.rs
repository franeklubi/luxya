use crate::{ast::stmt::Stmt, parser::types::Property, token::Token};
use std::{cell::Cell, rc::Rc};

#[derive(Clone)]
pub enum LiteralValue {
	List(Rc<Vec<Expr>>),
	String(Rc<str>),
	Number(f64),
	Char(char),
	True,
	False,
	Nil,
}

pub struct FunctionValue {
	pub keyword: Token,
	pub name: Option<Token>,
	pub params: Option<Rc<Vec<Token>>>,
	pub body: Option<Rc<Vec<Stmt>>>,
}

pub struct SetValue {
	pub setee: Box<Expr>,
	pub key: GetAccessor,
	pub value: Box<Expr>,
	pub blame: Token,
}

pub struct SuperValue {
	pub blame: Token,
	pub accessor: SuperAccessor,
	pub env_distance: Cell<u32>,
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

pub struct ObjectValue {
	pub blame: Token,
	pub properties: Vec<Property>,
}

pub struct ThisValue {
	pub blame: Token,
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
	Set(SetValue),
	Super(SuperValue),
	Call(CallValue),
	Assignment(AssignmentValue),
	Binary(BinaryValue),
	Get(GetValue),
	Identifier(IdentifierValue),
	Object(ObjectValue),
	This(ThisValue),
	Unary(UnaryValue),
	Grouping(GroupingValue),
	Literal(LiteralValue),
}

pub enum GetAccessor {
	DotName(Rc<str>),
	DotEval(Box<Expr>),
	SubscriptionNumber(f64),
	SubscriptionEval(Box<Expr>),
}

pub enum SuperAccessor {
	Method(Token),
	Call(Vec<Expr>),
}
