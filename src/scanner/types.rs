use std::{iter, str};


pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

pub type ScannerIter<'a, 'b> = &'a mut iter::Peekable<str::CharIndices<'b>>;

pub struct ConsumptionResult {
	pub last_offset: usize,
	pub hit_eof: bool,
}
