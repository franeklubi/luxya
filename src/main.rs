use jlox;

use std::{env, process};
use exitcode;

fn main() {
	let args: Vec<String> = env::args().collect();

	let args_len = args.len();

	if args_len > 2 {
		println!("Usage: jlox [script] <-- notice, only one script dummy");
		process::exit(exitcode::USAGE);
	} else if args_len == 2 {
		match jlox::run_file(&args[1]) {
			Err(err) => {
				println!("{}", err);
				process::exit(exitcode::IOERR);
			},
			_ => (),
		};
	} else {
		match jlox::run_prompt() {
			Err(err) => {
				println!("{}", err);
				process::exit(exitcode::OSERR);
			},
			_ => (),
		};
	}
}
