use std::io::{self, Read, Write};
use std::fs::File;

pub fn run_file(path: &String) -> Result<(), io::Error> {
	let mut f = File::open(path)?;

	let mut buffer = String::new();

	f.read_to_string(&mut buffer)?;

	run(&buffer);

	Ok(())
}

pub fn run_prompt() -> Result<(), io::Error> {
	println!("========\nPROMPT:");

	loop {
		print!(">>> ");
		io::stdout().flush()?;

		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer)?;

		if buffer.len() == 0 {
			break;
		}

		run(&buffer);
	}

	Ok(())
}

fn run(source: &String) {
	print!("{}", source);
}
