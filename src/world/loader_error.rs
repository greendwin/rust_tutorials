use std::error::Error;
use std::fmt;
use std::num::{ParseFloatError, ParseIntError};

pub type LoaderResult<T> = Result<T, LoaderError>;

#[derive(Debug, Eq, PartialEq)]
pub enum LoaderError {
    SyntaxError { line: usize, msg: String },
}

use LoaderError::*;

impl LoaderError {
    pub fn new_syntax(msg: impl Into<String>, line: usize) -> Self {
        Self::SyntaxError {
            msg: msg.into(),
            line,
        }
    }
}

impl fmt::Display for LoaderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for LoaderError {}

pub trait SyntaxContext<T> {
    fn with_context(self, line: usize) -> LoaderResult<T>;
}

impl<T> SyntaxContext<T> for Result<T, ParseIntError> {
    fn with_context(self, line: usize) -> LoaderResult<T> {
        match self {
            Err(err) => Err(SyntaxError {
                line,
                msg: format!("{:?}", err),
            }),
            Ok(r) => Ok(r),
        }
    }
}

impl<T> SyntaxContext<T> for Result<T, ParseFloatError> {
    fn with_context(self, line: usize) -> LoaderResult<T> {
        match self {
            Err(err) => Err(SyntaxError {
                line,
                msg: format!("{:?}", err),
            }),
            Ok(r) => Ok(r),
        }
    }
}
