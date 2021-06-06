use super::{errors, types::RunError};
use crate::{interpreter, parser, resolver, scanner};

use std::{
	fs,
	io::{self, Read, Write},
};


/// # Errors
///
/// Will return `RunError::Io` if `path`
/// does not exist or the user does not have permission to read it.
//
/// Will return `RunError::Exec` if any execution errors occur.
pub fn file(path: &str) -> Result<(), RunError> {
	let mut f = fs::File::open(path)?;

	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;

	if let true = run(&buffer) {
		return Err(RunError::Exec);
	};

	Ok(())
}

/// # Errors
///
/// Will return `Err` if there are any errors during reading from command line.
pub fn repl() -> Result<(), io::Error> {
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
		buffer += ";";

		// TODO: merge envs when doing REPL
		if run(&buffer) {
			eprintln!("Errors occurred")
		}
	}

	Ok(())
}

// TODO: maybe change the signature to return parsed tree of vars and functions
// so that we can merge that with the last tree in the REPL mode - we want
// things to be persistent dont we? (COMBAK: when implementing a better repl)
//
// bool indicates if any error(s) occurred
fn run(source: &str) -> bool {
	// Scanning
	let (tokens, errors) = scanner::scan(source);

	if !errors.is_empty() {
		errors::report(source, "Scan", &errors);

		return true;
	}

	// Parsing
	let (statements, errors) = parser::parse(tokens);

	if !errors.is_empty() {
		errors::report(source, "Parse", &errors);

		return true;
	}

	// Resolving
	if let Err(error) = resolver::resolve(&statements) {
		errors::report(source, "Resolve", &[error]);

		true

	// Interpreting 😇
	} else if let Err(error) = interpreter::interpret(&statements) {
		errors::report(source, "Runtime", &[error]);

		true
	} else {
		false
	}
}
