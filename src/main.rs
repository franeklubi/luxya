use std::{env, process};


fn main() {
	let args: Vec<String> = env::args().collect();

	if args.len() > 1 {
		for arg in args.iter().skip(1) {
			match luxya::run_file(&arg) {
				Err(luxya::RunError::Io(err)) => {
					println!("{}", err);
					process::exit(exitcode::IOERR);
				}
				Err(luxya::RunError::Exec) => {
					println!("Errors while executing {}", arg);
					process::exit(exitcode::DATAERR);
				}
				_ => (),
			}
		}
	} else if let Err(err) = luxya::run_prompt() {
		println!("{}", err);
		process::exit(exitcode::OSERR);
	}
}
