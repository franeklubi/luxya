use super::helpers::get_line;
use crate::token::Location;

// use std::fmt;


pub trait DescribableError {
	fn location(&self) -> Location;
	fn description(&self) -> &str;
}


pub fn report_errors<T>(source: &str, category: &str, errors: &[T])
where
	T: DescribableError,
{
	print!(
		"\n{} error{}:",
		category,
		if errors.len() > 1 { "s" } else { "" }
	);

	errors.iter().for_each(|error| report_error(source, error));

	println!();
}

fn report_error<T>(source: &str, error: &T)
where
	T: DescribableError,
{
	let location = error.location();

	let line = get_line(source, location.byte_offset);
	let line_prefix = line.prefix();

	let trimmed_content = line.content.trim_start();

	let trimmed_offset = (line.offset + 1)
		- (line.content.as_bytes().len() - trimmed_content.as_bytes().len());

	let trimmed_content = trimmed_content.trim_end();

	// Line output
	println!(
		"\n\t{line_prefix}: {trimmed_content}",
		line_prefix = line_prefix,
		trimmed_content = trimmed_content
	);

	// Error message output TODO: red colour
	println!(
		"\t{offset}{marker} {description}",
		offset = " ".repeat(line_prefix.len() + trimmed_offset),
		marker = "^".repeat(location.byte_length),
		description = error.description()
	);
}
