use crate::ast::expr::{Expr, IdentifierValue};


// A shorthand way to extract
pub fn assume_identifier_expr(expr: &Expr) -> &IdentifierValue {
	if let Expr::Identifier(i) = expr {
		i
	} else {
		unreachable!("Couldn't extract identifier expr. This shouldn't happen")
	}
}


// COMBAK: These macros are just temporary (I hope)

#[macro_export]
macro_rules! resolver_unwrap_scope {
	($wie:expr) => {{
		&$wie.env.borrow().scope
	}};
}

#[macro_export]
macro_rules! resolver_unwrap_scope_mut {
	($wie:expr) => {{
		&mut $wie.env.borrow_mut().scope
	}};
}

#[macro_export]
macro_rules! resolver_unwrap_enclosing {
	($wie:expr) => {{
		&$wie.env.borrow().enclosing
	}};
}
