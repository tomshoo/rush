use crate::Tracker;
use std::{fmt, rc::Rc};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
#[derive(Clone)]
pub enum IdError {
    #[error("Could not identify token '{0}'")]
    UnidentifiedToken(Rc<str>),

    #[error("Given literal is invalid: {0}")]
    InvalidLiteral(Rc<str>),
}

#[derive(Error, Debug, PartialEq, Eq)]
#[derive(Clone)]
pub struct LexerError(IdError, Tracker);

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LexerError: {} at {:?}", self.0, self.1)
    }
}

impl LexerError {
    pub fn new(err: IdError, at: Tracker) -> Self {
        Self(err, at)
    }

    pub fn at(&self) -> Tracker {
        self.1
    }

    pub fn error(&self) -> IdError {
        self.0.clone()
    }
}
