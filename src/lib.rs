use std::{io::{self, Read, Write}, fs, fmt};

mod scanner;


pub fn run_file(path: &String) -> Result<(), io::Error> {
	let mut f = fs::File::open(path)?;

	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;

	run(buffer);

	Ok(())
}

pub fn run_prompt() -> Result<(), io::Error> {
	loop {
		print!(">>> ");
		io::stdout().flush()?;

		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer)?;

		if buffer.len() == 0 {
			break;
		}

		run(buffer);
	}

	Ok(())
}

fn run(source: String) {
	print!("{}", source);

	let scanner = scanner::Scanner::from(source);

	let tokens = scanner.scan_tokens();

	tokens.iter().enumerate().for_each(|(index, token)| {
		println!("{}: {}", index, token)
	});
}

fn error<T: fmt::Display>(line: u32, message: T) {
	report(line, None::<&str>, message);
}

fn report<T1, T2>(line: u32, location: Option<T1>, message: T2) where
	T1: fmt::Display,
	T2: fmt::Display,
{
	match location {
		Some(l) => {
			println!("[{}, {}] Error: {}", line, l, message)
		},
		None => {
			println!("[{}] Error: {}", line, message)
		},
	}
}
