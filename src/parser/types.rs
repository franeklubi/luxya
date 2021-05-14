use crate::{ast::expr::*, runner::DescribableError, token::*};

use std::{iter, rc::Rc, vec};

pub type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub struct ParseError {
	pub token: Option<Token>,
	pub message: String,
}

impl DescribableError for ParseError {
	fn location(&self) -> Location {
		if let Some(token) = &self.token {
			token.location
		} else {
			Location {
				byte_offset: usize::MAX,
				byte_length: 1,
			}
		}
	}

	fn description(&self) -> &str {
		&self.message
	}
}

impl Expr {
	pub fn human_type(&self) -> &str {
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
			Expr::Object(_) => "an object definition",
		}
	}
}

pub struct Property {
	pub key: Rc<str>,
	pub value: Expr,
}
