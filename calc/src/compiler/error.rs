
use std::error::Error;
use std::fmt::{self, Display, Formatter};


#[derive(Debug)]
pub struct ParseError<'a> {
	description: String,
	filename: &'a str,
	line: i32,
}


impl<'a> ParseError<'a> {
	pub fn new(filename: &'a str, line: i32, description: String) -> ParseError<'a> {
		ParseError {
			description: description,
			filename: filename,
			line: line,
		}
	}
}


impl<'a> Display for ParseError<'a> {
	fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		write!(f, "{}({}): {}", self.filename, self.line, self.description)
	}
}


impl<'a> Error for ParseError<'a> {
	fn description(&self) -> &str {
		&self.description
	}

	fn cause(&self) -> Option<&Error> {
		None
	}
}
