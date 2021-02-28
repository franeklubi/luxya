use crate::ast::expr::*;

#[allow(dead_code)]
pub fn pn_stringify_tree(expr: &Expr) -> String {
	match expr {
		Expr::Binary(v) => pn_gen(
			format!("{}", v.operator.token_type),
			vec![&v.left, &v.right],
		),
		Expr::Unary(v) => {
			pn_gen(format!("{}", v.operator.token_type), vec![&v.right])
		}
		Expr::Grouping(v) => pn_gen("group".into(), vec![&v.expression]),
		Expr::Literal(v) => match v {
			LiteralValue::String(s) => format!("{:?}", s),
			LiteralValue::Number(n) => format!("{}", n),
			LiteralValue::True => "true".into(),
			LiteralValue::False => "false".into(),
			LiteralValue::Nil => "nil".into(),
		},
		Expr::Identifier(v) => v.token.token_type.to_string(),
	}
}

#[allow(dead_code)]
fn pn_gen(name: String, exprs: Vec<&Expr>) -> String {
	let mut res = format!("({}", name);

	exprs.iter().for_each(|expr| {
		res += " ";
		res += &pn_stringify_tree(expr);
	});

	res + ")"
}
