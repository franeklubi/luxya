use crate::{
	ast::expr::Expr,
	runner::DescribableError,
	token::{Location, Token},
};

use std::{iter, rc::Rc, vec};

pub type ParserIter<'a> = &'a mut iter::Peekable<vec::IntoIter<Token>>;

pub struct ParseError {
	pub token: Option<Token>,
	pub message: String,
}

impl DescribableError for ParseError {
	fn location(&self) -> Location {
		self.token.as_ref().map_or(
			Location {
				byte_offset: usize::MAX,
				byte_length: 1,
			},
			|token| token.location,
		)
	}

	fn description(&self) -> &str {
		&self.message
	}
}

impl Expr {
	pub const fn human_type(&self) -> &str {
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
