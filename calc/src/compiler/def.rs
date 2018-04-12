
use compiler::error::ParseError;


pub type ParseResult<'a, T> = Result<T, ParseError<'a>>;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loc<'a> {
    pub filename: &'a str,
    pub line: i32,
}


#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Symbol(Loc<'a>, char),
    Int(Loc<'a>, i32),
    Ident(Loc<'a>, &'a str),
    Eof,
}
