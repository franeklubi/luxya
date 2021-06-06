#![feature(option_result_unwrap_unchecked)]
#![warn(
	clippy::all,
	clippy::pedantic,
	clippy::nursery,
	clippy::unnecessary_wraps,
	clippy::semicolon_if_nothing_returned
)]

mod ast;
mod env;
mod interpreter;
mod parser;
mod resolver;
mod runner;
mod scanner;
mod token;

pub use runner::*;
pub use runner::{run_file, run_repl};
