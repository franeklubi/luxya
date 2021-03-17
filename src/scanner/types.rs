use std::{iter, str};


pub struct ScanError {
	pub offset: usize,
	pub message: String,
}

pub type ScannerIter<'a, 'b> = &'a mut iter::Peekable<str::CharIndices<'b>>;
