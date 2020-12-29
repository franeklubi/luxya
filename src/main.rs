use std::{env, process};

use jlox;

use exitcode;


fn main() {
	let args: Vec<String> = env::args().collect();

	let args_len = args.len();

	if args_len > 2 {
		// TODO: exec all scripts in the order they're passed maybe???
		println!("Usage: jlox [script] <-- notice, only one script dummy");
		process::exit(exitcode::USAGE);
	} else if args_len == 2 {
		if let Err(err) = jlox::run_file(&args[1]) {
			match err {
				jlox::RunError::IO(err) => {
					println!("{}", err);
					process::exit(exitcode::IOERR);
				}
				jlox::RunError::EXEC => {
					println!("Errors while executing the file");
					process::exit(exitcode::DATAERR);
				}
			}
		};
	} else {
		match jlox::run_prompt() {
			Err(err) => {
				println!("{}", err);
				process::exit(exitcode::OSERR);
			}
			_ => (),
		};
	}
}
