
use std::error::Error;
use std::fmt::{self, Display, Formatter};


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Loc<'a> {
    pub filename: &'a str,
    pub line: i32,
}


#[derive(Debug)]
pub struct ParseError<'a> {
    pub description: String,
    pub loc: Loc<'a>,
}


pub type TokenizeResult<'a> = Result<Vec<Token<'a>>, ParseError<'a>>;
pub type ParseResult<'a> = Result<AST<'a>, ParseError<'a>>;


#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Symbol(Loc<'a>, char),
    Int(Loc<'a>, i32),
    Ident(Loc<'a>, &'a str),
    Eof(Loc<'a>),
}


#[derive(Debug, PartialEq)]
pub enum AST<'a> {
	Num(Loc<'a>, i32),
	Var(Loc<'a>, &'a str),
	
	BinOp {
		loc: Loc<'a>,
		op: char,
		left: Box<AST<'a>>,
		right: Box<AST<'a>>,
	},
}


pub trait Location<'a> {
	fn loc(&self) -> Loc<'a>;

    fn error<T>(&self, description: String) -> Result<T, ParseError<'a>> {
        Err(ParseError {
            description: description,
            loc: self.loc(),
        })
    }

    fn error_str<T>(&self, description: &str) -> Result<T, ParseError<'a>> {
        Err(ParseError {
            description: String::from(description),
            loc: self.loc(),
        })
    }
}


impl<'a> Location<'a> for Token<'a> {
	fn loc(&self) -> Loc<'a> {
		use self::Token::*;

		match *self {
			Symbol(loc, _) => loc,
			Int(loc, _) => loc,
			Ident(loc, _) => loc,
			Eof(loc) => loc,
		}
	}
}


impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use self::Token::*;

		match *self {
			Symbol(_loc, ch) => write!(f, "{}", ch),
			Int(_loc, val) => write!(f, "{}", val),
			Ident(_loc, name) => write!(f, "{}", name),
			Eof(_loc) => write!(f, "<EOF>"),
		}
	}
}


impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}({}): {}", self.loc.filename, self.loc.line, self.description)
    }
}


impl<'a> Error for ParseError<'a> {
    fn description(&self) -> &str {
        &self.description
    }
}
