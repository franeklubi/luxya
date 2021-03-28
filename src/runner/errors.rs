use std::fmt;


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
