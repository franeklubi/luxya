use std::{iter, str};

// match_to_peek returns true (and consumes next char)
// only if it maches the expected char
pub fn match_to_peek(
	chars: &mut iter::Peekable<str::CharIndices>,
	expected: char,
) -> bool {
	match chars.peek() {
		Some((_, c)) => *c == expected,
		None => false,
	}
}

// will consume chars while peek matches the predicate
//
// returns a result with the index (in bytes) of where the next char would be
// (regardless of it being there or the stream ending)
//
// returns an error with last_offset when the scanning has reached the eof
pub fn consume_while_peek(
	chars: &mut iter::Peekable<str::CharIndices>,
	predicate: impl Fn(&char) -> bool,
) -> Result<usize, usize> {
	let mut last_offset = 0;

	loop {
		break match chars.peek() {
			Some((i, c)) if predicate(c) => {
				last_offset = i + c.len_utf8();
				chars.next();

				continue;
			}
			Some((i, c)) => Ok(i + c.len_utf8()),
			None => Err(last_offset),
		};
	}
}
