use crate::ast::expr::*;
use crate::token::Token;

pub fn parse(tokens: &Vec<Token>) {
	println!("parser yo:");

	println!("{} TOKENS", tokens.len());
	// tokens.iter().enumerate().for_each(|(index, token)| {
	// 	println!("{}: {}", index, token);
	// });

	println!("parser done");
}
