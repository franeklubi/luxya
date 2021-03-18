use crate::ast::expr::Expr;
use crate::token::Token;

pub struct ContinueValue {
	pub keyword: Token,
}

pub struct BreakValue {
	pub keyword: Token,
}

pub struct ReturnValue {
	pub keyword: Token,
	pub expression: Option<Expr>,
}

pub struct ForValue {
	pub condition: Option<Box<Expr>>,
	pub body: Box<Stmt>,
	pub closer: Option<Box<Stmt>>,
}

pub struct IfValue {
	pub condition: Box<Expr>,
	pub then: Box<Stmt>,
	pub otherwise: Option<Box<Stmt>>,
}

pub struct BlockValue {
	pub statements: Vec<Stmt>,
}

pub struct ExpressionValue {
	pub expression: Box<Expr>,
}

pub struct PrintValue {
	pub expression: Box<Expr>,
}

pub struct DeclarationValue {
	pub name: Token,
	pub initializer: Option<Box<Expr>>,
	pub mutable: bool,
}

pub enum Stmt {
	Continue(ContinueValue),
	Break(BreakValue),
	Return(ReturnValue),
	For(ForValue),
	If(IfValue),
	Block(BlockValue),
	Expression(ExpressionValue),
	Print(PrintValue),
	Declaration(DeclarationValue),
}
