
use compiler::error::ParseError;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loc<'a> {
    pub filename: &'a str,
    pub line: i32,
}


impl<'a> Loc<'a> {
    pub fn error<T>(&self, description: String) -> Result<T, ParseError<'a>> {
        Err(ParseError::new(
            self.filename, self.line,
            description))
    }
}


#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Int(Loc<'a>, i32),
    LeftBr(Loc<'a>),
    RightBr(Loc<'a>),
    Plus(Loc<'a>),
    Minus(Loc<'a>),
    Mul(Loc<'a>),
    Div(Loc<'a>),
    Mod(Loc<'a>),
}
