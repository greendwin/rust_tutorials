use std::error;
use std::fmt::{self, Display, Formatter};
use super::location::*;


#[derive(Debug)]
pub struct Error {
    pub description: String,
    pub loc: Loc,
}


impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}


impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}({}): {}", self.loc.filename, self.loc.line, self.description)
    }
}

