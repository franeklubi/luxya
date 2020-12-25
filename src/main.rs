use std::io::{self, Read, Write};
use std::env;
use std::process;
use std::fs::File;
use exitcode;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args_len = args.len();

	if args_len > 2 {
		println!("Usage: jlox [script] <-- notice, only one script dummy");
		process::exit(exitcode::USAGE);
	} else if args_len == 2 {
		match run_file(&args[1]) {
			Err(err) => {
				println!("{}", err);
				process::exit(exitcode::IOERR);
			},
			_ => (),
		};
	} else {
		match run_prompt() {
			Err(err) => {
				println!("{}", err);
				process::exit(exitcode::OSERR);
			},
			_ => (),
		};
	}
}

fn run_file(path: &String) -> Result<(), io::Error> {
	let mut f = File::open(path)?;

	let mut buffer = String::new();

	f.read_to_string(&mut buffer)?;

	run(&buffer);

	Ok(())
}

fn run_prompt() -> Result<(), io::Error> {
	println!("========\nPROMPT:");

	loop {
		print!(">>> ");
		io::stdout().flush()?;

		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer)?;

		run(&buffer);
	}
}

fn run(source: &String) {
	print!("{}", source);
}
