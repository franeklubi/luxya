use std::{
	fmt,
	fs,
	io::{self, Read, Write},
};

mod ast;
mod scanner;
mod token;

pub enum RunError {
	Io(io::Error),
	Exec,
}

impl From<io::Error> for RunError {
	fn from(e: io::Error) -> Self {
		RunError::Io(e)
	}
}

pub fn run_file(path: &str) -> Result<(), RunError> {
	let mut f = fs::File::open(path)?;

	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;

	if let true = run(buffer) {
		return Err(RunError::Exec);
	};

	Ok(())
}

pub fn run_prompt() -> Result<(), io::Error> {
	loop {
		print!(">>> ");
		io::stdout().flush()?;

		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer)?;

		if buffer.is_empty() {
			break;
		}

		// In REPL mode, we always add `;` at the end so that
		// the user doesn't have to 😇
		//
		// That also saves us the check for EOF, every time I expect
		// a semicolon
		buffer += ";";

		if run(buffer) {
			eprintln!("Errors occurred, statement not merged.")
		}
	}

	Ok(())
}

// TODO: maybe change the signature to return parsed tree of vars and functions
// so that we can merge that with the last tree in the REPL mode - we want
// things to be persistent dont we?
//
// bool indicates if any error(s) occurred, but maybe it should return errors?
// errors would have to be handled outside and not printed outright
fn run(source: String) -> bool {
	// scanning
	let (tokens, scan_errors) = scanner::scan_tokens(&source);

	// parsing
	let (statements, parse_errors) = ast::parse(tokens);

	// interpreting 😇
	if scan_errors.is_empty() && parse_errors.is_empty() {
		if let Err(e) = ast::interpret(&statements) {
			println!(
				"Runtime error {}\n\t{}",
				get_line(&source, e.token.byte_offset),
				e.message
			);
		}
	}

	if !scan_errors.is_empty() {
		println!("\nSCAN ERRORS:");
	}
	scan_errors.iter().enumerate().for_each(|(index, error)| {
		println!("{}: {}", index, error.message);
	});

	if !parse_errors.is_empty() {
		println!("\nPARSE ERRORS:");
	}
	parse_errors.iter().enumerate().for_each(|(index, error)| {
		println!(
			"{}: {} at {}",
			index,
			error.message,
			get_line(&source, error.token.clone().map_or(0, |t| t.byte_offset))
		);
	});

	println!();

	!scan_errors.is_empty() || !parse_errors.is_empty()
}

// TODO: delete that allow
#[allow(dead_code)]
fn error<T: fmt::Display>(line: u32, message: T) {
	report(line, None::<&str>, message)
}

// TODO: delete that allow
#[allow(dead_code)]
fn report<T1, T2>(line: u32, location: Option<T1>, message: T2)
where
	T1: fmt::Display,
	T2: fmt::Display,
{
	match location {
		Some(l) => {
			eprintln!("[{}, {}] Error: {}", line, l, message)
		}
		None => {
			eprintln!("[{}] Error: {}", line, message)
		}
	}
}

fn get_line(source: &str, byte_offset: usize) -> Line {
	let mut line_start_offset = 0;
	let mut line_end_offset = source.len();
	let mut lines = 1;

	// getting the start and end of the line
	for (i, c) in source.as_bytes().iter().enumerate() {
		if *c == b'\n' {
			if i < byte_offset {
				line_start_offset = i + 1;
			} else {
				line_end_offset = i;
				break;
			}

			lines += 1;
		}
	}

	Line {
		content: source[line_start_offset..line_end_offset].to_string(),
		number: lines,
		offset: byte_offset - line_start_offset + 1,
	}
}

struct Line {
	number: u32,
	offset: usize,
	content: String,
}

impl fmt::Display for Line {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "[{}:{}]: {}", self.number, self.offset, self.content)
	}
}
