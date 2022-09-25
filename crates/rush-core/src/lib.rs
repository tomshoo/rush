pub mod error;
pub mod interfaces;

use crate::error::RushError;
use error::lexer::LexerError;

pub type Result<T> = std::result::Result<T, RushError>;
pub type LexerResult<T> = std::result::Result<T, LexerError>;
