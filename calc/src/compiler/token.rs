use std::fmt::{self, Display, Formatter};
use super::location::*;


#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Symbol(Loc<'a>, char),
    Int(Loc<'a>, i32),
    Ident(Loc<'a>, &'a str),
    Eof(Loc<'a>),
}


impl<'a> Token<'a> {
    pub fn is_symbol(&self, expected: char) -> bool {
        match *self {
            Token::Symbol(_loc, ch) => (ch == expected),
            _ => false,
        }
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

