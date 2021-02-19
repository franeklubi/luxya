use crate::ast::expr::Expr;

pub struct ExpressionValue {
	pub expression: Box<Expr>,
}

pub struct PrintValue {
	pub expression: Box<Expr>,
}

pub enum Stmt {
	Expression(ExpressionValue),
	Print(PrintValue),
}
