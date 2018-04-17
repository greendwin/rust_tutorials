use std::fmt::{self, Display, Formatter};
use super::location::*;


#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Symbol(Loc, char),
    Int(Loc, i32),
    Str(Loc, &'a str),
    Ident(Loc, &'a str),
    Eof(Loc),
}


impl<'a> Token<'a> {
    pub fn is_symbol(&self, expected: char) -> bool {
        match *self {
            Token::Symbol(_, ch) => (ch == expected),
            _ => false,
        }
    }
}


impl<'a> Location for Token<'a> {
	fn loc(&self) -> &Loc {
		use self::Token::*;

		match *self {
			Symbol(ref loc, _) => loc,
			Int(ref loc, _) => loc,
			Str(ref loc, _) => loc,
			Ident(ref loc, _) => loc,
			Eof(ref loc) => loc,
		}
	}
}


impl<'a> Display for Token<'a> {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
		use self::Token::*;

		match *self {
			Symbol(_, ch) => write!(f, "{}", ch),
			Int(_, val) => write!(f, "{}", val),
			Str(_, val) => write!(f, "{}", val),
			Ident(_, name) => write!(f, "{}", name),
			Eof(_) => write!(f, "<EOF>"),
		}
	}
}

