#![feature(option_result_unwrap_unchecked)]
#![warn(
	clippy::all,
	clippy::pedantic,
	// clippy::nursery,
	// clippy::cargo
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
pub use runner::{run_repl, run_file};
