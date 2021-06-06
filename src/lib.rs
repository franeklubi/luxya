#![feature(option_result_unwrap_unchecked)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

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
