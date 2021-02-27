use crate::ast::expr::Expr;
use crate::token::Token;

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
	Expression(ExpressionValue),
	Print(PrintValue),
	Declaration(DeclarationValue),
}
