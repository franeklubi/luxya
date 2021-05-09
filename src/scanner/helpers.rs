use super::types::{ScanError, ScannerIter};


// match_to_peek returns true (and consumes next char)
// only if it maches the expected char
pub fn match_to_peek(chars: ScannerIter, expected: char) -> bool {
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
// returns an error with last_offset when the scanning has reached eof
pub fn consume_while_peek(
	chars: ScannerIter,
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
			Some((i, _)) => Ok(*i),
			None => Err(last_offset),
		};
	}
}

pub fn expect_char(
	chars: ScannerIter,
	after: char,
	offset: usize,
	override_message: Option<&str>,
) -> Result<char, ScanError> {
	if let Some(c) = chars.next() {
		Ok(c.1)
	} else if let Some(msg) = override_message {
		Err(ScanError {
			message: msg.to_owned(),
			offset,
		})
	} else {
		Err(ScanError {
			message: format!("Expected char after `{}`", after),
			offset,
		})
	}
}
