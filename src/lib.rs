#![feature(option_result_unwrap_unchecked)]
// #![warn(
// 	clippy::all,
// 	clippy::restriction,
// 	clippy::pedantic,
// 	clippy::nursery,
// 	clippy::cargo,
// )]

mod ast;
mod env;
mod interpreter;
mod parser;
mod resolver;
mod runner;
mod scanner;
mod token;

pub use runner::*;
