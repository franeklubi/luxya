use crate::ast::expr::Expr;

use std::cell::Cell;


// A shorthand way to extract identifier expr
pub fn assume_resolvable_expr(expr: &Expr) -> &Cell<u32> {
	match expr {
		Expr::Identifier(i) => &i.env_distance,
		Expr::Assignment(a) => &a.env_distance,
		Expr::This(t) => &t.env_distance,
		Expr::Super(s) => &s.env_distance,
		_ => unreachable!(
			"Couldn't extract resolvable expr. This shouldn't happen"
		),
	}
}
