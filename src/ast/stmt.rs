use crate::ast::expr::Expr;
use crate::token::Token;

pub struct ForValue {
	pub condition: Option<Expr>,
	pub body: Box<Stmt>,
	pub closer: Option<Box<Stmt>>,
}

pub struct IfValue {
	pub condition: Expr,
	pub then: Option<Box<Stmt>>,
	pub otherwise: Option<Box<Stmt>>,
}

pub struct DeclarationValue {
	pub name: Token,
	pub initializer: Option<Expr>,
	pub mutable: bool,
}

pub struct ClassValue {
	pub name: Token,
	pub methods: Vec<Expr>,
	pub superclass: Option<Expr>,
}

pub struct ReturnValue {
	pub keyword: Token,
	pub expression: Option<Expr>,
}

pub struct ExpressionValue {
	pub expression: Expr,
}

pub struct BlockValue {
	pub statements: Vec<Stmt>,
}

pub struct ContinueValue {
	pub keyword: Token,
}

pub struct PrintValue {
	pub expression: Expr,
}

pub struct BreakValue {
	pub keyword: Token,
}

pub enum Stmt {
	For(ForValue),
	If(IfValue),
	Declaration(DeclarationValue),
	Class(ClassValue),
	Return(ReturnValue),
	Expression(ExpressionValue),
	Block(BlockValue),
	Continue(ContinueValue),
	Print(PrintValue),
	Break(BreakValue),
}
