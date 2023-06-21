use core::tracker::Tracker;
use std::rc::Rc;

use lexer::error::LexerError;
use thiserror::Error;

#[derive(Error)]
#[derive(Debug)]
#[error("Error: {message} at: {tracker}")]
pub struct ParseError {
    message: Rc<str>,
    tracker: Tracker,
}

#[derive(Error)]
#[derive(Debug)]
pub enum Error {
    #[error("{0}")]
    Lexer(#[from] LexerError),

    #[error("{0}")]
    Parser(#[from] ParseError),
}
