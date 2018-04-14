use std::error;
use std::fmt::{self, Display, Formatter};
use super::location::*;


#[derive(Debug)]
pub struct Error<'a> {
    pub description: String,
    pub loc: Loc<'a>,
}


impl<'a> error::Error for Error<'a> {
    fn description(&self) -> &str {
        &self.description
    }
}


impl<'a> Display for Error<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}({}): {}", self.loc.filename, self.loc.line, self.description)
    }
}

