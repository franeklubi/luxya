use super::{helpers::*, types::*};
use crate::{interpreter, parser, resolver, scanner};

use std::{
	fs,
	io::{self, Read, Write},
};


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
	let (tokens, scan_errors) = scanner::scan(&source);

	if !scan_errors.is_empty() {
		println!("\nSCAN ERRORS:");

		scan_errors.iter().enumerate().for_each(|(index, error)| {
			println!("{}: {}", index, error.message);
		});

		return true;
	}

	// parsing
	let (statements, parse_errors) = parser::parse(tokens);

	if !parse_errors.is_empty() {
		println!("\nPARSE ERRORS:");

		parse_errors.iter().enumerate().for_each(|(index, error)| {
			println!(
				"{}: {} at {}",
				index,
				error.message,
				get_line(
					&source,
					error.token.clone().map_or(0, |t| t.byte_offset)
				)
			);
		});

		println!();

		return true;
	}

	// interpreting 😇
	if let Err(e) = resolver::resolve(&statements) {
		println!(
			"\nResolve error {}\n\t{}\n",
			get_line(&source, e.token.byte_offset),
			e.message
		);

		true
	} else if let Err(e) = interpreter::interpret(&statements) {
		println!(
			"\nRuntime error {}\n\t{}\n",
			get_line(&source, e.token.byte_offset),
			e.message
		);

		true
	} else {
		false
	}
}
