use std::env;
use std::process;

fn main() {
	let args: Vec<String> = env::args().collect();
	println!("{:?}", args);
	let args_len = args.len();
	if args_len > 2 {
		println!("Usage: jlox [script]");
		process::exit(64);
	} else if args_len == 2 {
		run_file(&args[1]);
	} else {
		run_prompt();
	}
}

fn run_file(filename: &String) {
	println!("running file: {}", filename);
}

fn run_prompt() {
	println!("heja prompt");
}
