use crate::{runner::DescribableError, token::Location};

use std::{iter, str};


pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

impl DescribableError for ScanError {
	fn location(&self) -> Location {
		Location {
			byte_offset: self.offset,
			byte_length: 1,
		}
	}

	fn description(&self) -> &str {
		&self.message
	}
}

pub type ScannerIter<'a, 'b> = &'a mut iter::Peekable<str::CharIndices<'b>>;

pub struct ConsumptionResult {
	pub last_offset: usize,
	pub hit_eof: bool,
}
