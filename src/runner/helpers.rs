use super::types::Line;


pub fn get_line(source: &str, byte_offset: usize) -> Line {
	let mut line_start_offset = 0;
	let mut line_end_offset = source.len();
	let mut lines = 1;

	// getting the start and end of the line
	for (i, c) in source.as_bytes().iter().enumerate() {
		if *c == b'\n' {
			if i < byte_offset {
				line_start_offset = i + 1;
			} else {
				line_end_offset = i;
				break;
			}

			lines += 1;
		}
	}

	Line {
		content: source[line_start_offset..line_end_offset].to_string(),
		number: lines,
		offset: byte_offset - line_start_offset + 1,
	}
}
