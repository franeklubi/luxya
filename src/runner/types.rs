use std::{
	fmt,
	io::{self},
};

pub enum RunError {
	Io(io::Error),
	Exec,
}

impl From<io::Error> for RunError {
	fn from(e: io::Error) -> Self {
		RunError::Io(e)
	}
}

pub struct Line {
	pub number: u32,
	pub offset: usize,
	pub content: String,
}

impl fmt::Display for Line {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}:{}]: {}", self.number, self.offset, self.content)
	}
}
