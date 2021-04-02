use crate::ast::expr::Expr;
use crate::token::Token;

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

pub struct DeclarationValue {
	pub name: Token,
	pub initializer: Option<Box<Expr>>,
	pub mutable: bool,
}

pub struct ReturnValue {
	pub keyword: Token,
	pub expression: Option<Expr>,
}

pub struct ClassValue {
	pub name: Token,
	pub methods: Vec<Expr>,
}

pub struct ExpressionValue {
	pub expression: Box<Expr>,
}

pub struct BlockValue {
	pub statements: Vec<Stmt>,
}

pub struct PrintValue {
	pub expression: Box<Expr>,
}

pub struct ContinueValue {
	pub keyword: Token,
}

pub struct BreakValue {
	pub keyword: Token,
}

pub enum Stmt {
	For(ForValue),
	If(IfValue),
	Declaration(DeclarationValue),
	Return(ReturnValue),
	Class(ClassValue),
	Expression(ExpressionValue),
	Block(BlockValue),
	Print(PrintValue),
	Continue(ContinueValue),
	Break(BreakValue),
}
