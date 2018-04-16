use std::rc::Rc;
use super::error::*;


#[derive(Debug, Clone, PartialEq)]
pub struct Loc {
    pub filename: Rc<String>,
    pub line: i32,
}


impl Loc {
    pub fn new(filename: &str, line: i32) -> Self {
        Loc {
            filename: Rc::new(String::from(filename)),
            line,
        }
    }
}


pub trait Location {
	fn loc(&self) -> &Loc;

    fn error(&self, description: String) -> Error {
        Error {
            description: description,
            loc: self.loc().clone(),
        }
    }

    fn error_str(&self, description: &str) -> Error {
        Error {
            description: String::from(description),
            loc: self.loc().clone(),
        }
    }
}


impl Location for Loc {
    fn loc(&self) -> &Loc {
        self
    }
}


impl<T> Into<Result<T, Error>> for Error {
    fn into(self) -> Result<T, Error> {
        Err(self)
    }
}
