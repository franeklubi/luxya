use crate::{ast::expr::*, token::*};

use std::{iter, vec};

pub type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub struct ParseError {
	pub token: Option<Token>,
	pub message: String,
}

impl Expr {
	pub fn to_human_readable(&self) -> &str {
		match self {
			Expr::Assignment(_) => "an assignment",
			Expr::Binary(_) => "a binary expression",
			Expr::Grouping(_) => "a grouping",
			Expr::Literal(_) => "a literal",
			Expr::Unary(_) => "a unary expression",
			Expr::Identifier(_) => "an identifier",
			Expr::Call(_) => "a function/method call",
			Expr::Function(_) => "a function/method declaration",
			Expr::Get(_) => "property getter",
			Expr::Set(_) => "property setter",
			Expr::This(_) => "a this expression",
			Expr::Super(_) => "a super expression",
		}
	}
}
