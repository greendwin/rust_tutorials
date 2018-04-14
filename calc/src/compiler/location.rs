use super::error::*;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loc<'a> {
    pub filename: &'a str,
    pub line: i32,
}


pub trait Location<'a> {
	fn loc(&self) -> Loc<'a>;

    fn error<T>(&self, description: String) -> Result<T, Error<'a>> {
        Err(Error {
            description: description,
            loc: self.loc(),
        })
    }

    fn error_str<T>(&self, description: &str) -> Result<T, Error<'a>> {
        Err(Error {
            description: String::from(description),
            loc: self.loc(),
        })
    }
}

