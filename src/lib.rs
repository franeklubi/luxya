use std::{
	convert,
	fmt,
	fs,
	io::{self, Read, Write},
};

mod ast;
mod scanner;
mod token;

pub enum RunError {
	IO(io::Error),
	EXEC,
}

impl convert::From<io::Error> for RunError {
	fn from(e: io::Error) -> Self {
		RunError::IO(e)
	}
}

pub fn run_file(path: &String) -> Result<(), RunError> {
	let mut f = fs::File::open(path)?;

	let mut buffer = String::new();
	f.read_to_string(&mut buffer)?;

	if let true = run(buffer) {
		return Err(RunError::EXEC);
	};

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

		if let true = run(buffer) {
			eprintln!("Errors occurred, expression not merged.")
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
	// println!("==== SOURCE ====");
	// println!("{}", source);
	// println!("==== END ====");

	// just for debug purposes
	let expression = ast::Expr::Binary(ast::BinaryValue {
		left: Box::new(ast::Expr::Unary(ast::UnaryValue {
			operator: token::Token {
				byte_length: 1,
				byte_offset: 1,
				token_type: token::TokenType::Minus,
			},
			right: Box::new(ast::Expr::Literal(ast::LiteralValue::Number(
				1234.0,
			))),
		})),
		operator: token::Token {
			byte_length: 1,
			byte_offset: 1,
			token_type: token::TokenType::Star,
		},
		right: Box::new(ast::Expr::Grouping(ast::GroupingValue {
			expression: Box::new(ast::Expr::Literal(
				ast::LiteralValue::Number(4567.0),
			)),
		})),
	});

	println!("{}", ast::pn_stringify_tree(&expression));

	let (tokens, errors) = scanner::scan_tokens(&source);

	// println!("TOKENS:");
	// tokens.iter().enumerate().for_each(|(index, token)| {
	// 	println!("{}: {}", index, token);
	// });
	println!("{} TOKENS", tokens.len());

	println!("ERRORS:");
	errors.iter().enumerate().for_each(|(index, error)| {
		println!("{}: {}", index, error.message);
	});
	// println!("{} ERRORS", errors.len());

	errors.len() > 0
}

// TODO: delete that allow
#[allow(dead_code)]
fn error<T: fmt::Display>(line: u32, message: T) {
	report(line, None::<&str>, message)
}

// TODO: delete that allow
#[allow(dead_code)]
fn report<T1, T2>(line: u32, location: Option<T1>, message: T2)
where
	T1: fmt::Display,
	T2: fmt::Display,
{
	match location {
		Some(l) => {
			eprintln!("[{}, {}] Error: {}", line, l, message)
		}
		None => {
			eprintln!("[{}] Error: {}", line, message)
		}
	}
}
