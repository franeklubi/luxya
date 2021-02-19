use std::{env, process};


fn main() {
	let args: Vec<String> = env::args().collect();

	let args_len = args.len();

	// TODO: rebuild this whole section

	match args_len {
		len if len > 2 => {
			println!("Usage: jlox [script] <-- notice, only one script dummy");
			process::exit(exitcode::USAGE);
		}
		len if len == 2 => {
			if let Err(err) = jlox::run_file(&args[1]) {
				match err {
					jlox::RunError::Io(err) => {
						println!("{}", err);
						process::exit(exitcode::IOERR);
					}
					jlox::RunError::Exec => {
						println!("Errors while executing {}", &args[1]);
						process::exit(exitcode::DATAERR);
					}
				}
			};
		}
		_ => {
			if let Err(err) = jlox::run_prompt() {
				println!("{}", err);
				process::exit(exitcode::OSERR);
			}
		}
	}
}
