use crate::Tracker;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum IdError {
    #[error("Invalid character '{0}'")]
    InvalidCharacter(char),

    #[error("Could not identify token '{0}'")]
    UnidentifiedToken(String),

    #[error("Given literal is invalid: {0}")]
    InvalidLiteral(String),
}

#[derive(Error, Debug, PartialEq, Eq)]
pub struct LexerError(IdError, Tracker);

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LexerError: {} at {:?}", self.0, self.1)
    }
}

impl LexerError {
    pub(crate) fn new(err: IdError, at: Tracker) -> Self {
        Self(err, at)
    }
}
