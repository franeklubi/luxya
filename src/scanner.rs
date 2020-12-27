use crate::token;


pub fn scan_tokens(source: &String) -> Vec<token::Token> {
	source.split(' ').map(|lexeme| {
		token::Token {
			token: token::TokenType::CharSlice(lexeme.trim()),
			offset: 0,
			length: 0,
		}
	}).collect()
}
