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
	let (tokens, scan_errors) = scanner::scan_tokens(&source);

	if !scan_errors.is_empty() {
		println!("SCAN ERRORS:");
	}
	scan_errors.iter().enumerate().for_each(|(index, error)| {
		println!("{}: {}", index, error.message);
	});

	// tokens.iter().enumerate().for_each(|(index, token)| {
	// 	println!("{}: {}", index, token);
	// });

	let (statements, parse_errors) = ast::parse(tokens);

	if !parse_errors.is_empty() {
		println!("PARSE ERRORS:");
	}
	parse_errors.iter().enumerate().for_each(|(index, error)| {
		println!("{}: {} at {:?}", index, error.message, error.token);
	});

	statements.iter().enumerate().for_each(|(index, stmt)| {
		if let Err(e) = ast::evaluate(&stmt) {
			error(index as u32, e.message)
		}
	});

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
