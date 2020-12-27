use crate::token;
use std::str;


fn scan_token<'a>(
	chars: &mut str::CharIndices,
	source: &'a String,
) -> Option<token::Token<'a>> {
	while let Some((i, c)) = chars.next() {
		// We should be at the beginning of the next lexeme
		// println!("{}: {:?} ", i, c);
		let char_len = c.len_utf8();

		return Some(token::Token {
			token: token::TokenType::CharSlice(&source[i..i+char_len]),
			byte_offset: i,
			byte_length: char_len,
		})
	}

	None
}

pub fn scan_tokens(source: &String) -> Vec<token::Token> {
	let mut tokens = vec![];

	let mut chars = source.char_indices();

	// let mut current_index = 0;
	// let mut current_char = ' ';

	while let Some(token) = scan_token(&mut chars, source) {
		// We should be at the beginning of the next lexeme
		// println!("{}", token);

		tokens.push(token);
	}

	let last_offset = match tokens.last() {
		Some(token) => token.byte_offset + token.byte_length,
		None => 0,
	};

	tokens.push(
		token::Token {
			token: token::TokenType::Eof,
			byte_offset: last_offset,
			byte_length: 1,
		},
	);

	tokens
}
