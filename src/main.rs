use std::{env, process};


fn main() {
	let args: Vec<String> = env::args().collect();

	// TODO: rebuild this whole section

	if args.len() > 1 {
		for arg in args.iter().skip(1) {
			match jlox::run_file(&arg) {
				Err(jlox::RunError::Io(err)) => {
					println!("{}", err);
					process::exit(exitcode::IOERR);
				}
				Err(jlox::RunError::Exec) => {
					println!("Errors while executing {}", arg);
					process::exit(exitcode::DATAERR);
				}
				_ => (),
			}
		}
	} else if let Err(err) = jlox::run_prompt() {
		println!("{}", err);
		process::exit(exitcode::OSERR);
	}
}
