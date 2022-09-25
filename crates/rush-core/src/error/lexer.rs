use super::RushError;
use crate::interfaces::tracker::Tracker;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(Tracker, char),
    UnescapableCharacter(Tracker, String),
    UndeterminedType(char),
    ExpectedCharacter(Tracker, char),
    Internal(RushError),
}

impl From<LexerError> for RushError {
    fn from(err: LexerError) -> Self {
        match err {
            LexerError::UnexpectedCharacter(tracker, ch) => Self {
                kind: "LexerError".into(),
                msg: format!("Unexpected character \'{}\' found at {:?}", ch, tracker),
            },
            LexerError::UndeterminedType(ch) => Self {
                kind: "LexerError".into(),
                msg: format!("Cannot determine type for \'{}\'", ch),
            },
            LexerError::UnescapableCharacter(tracker, ctype) => Self {
                kind: "LexerError".into(),
                msg: format!("Cannot escape {} at {:?}", ctype, tracker),
            },
            LexerError::ExpectedCharacter(tracker, ch) => Self {
                kind: "LexerError".into(),
                msg: format!("Expected {} at {:?}", ch, tracker),
            },
            LexerError::Internal(err) => err,
        }
    }
}

impl From<RushError> for LexerError {
    fn from(err: RushError) -> Self {
        LexerError::Internal(err)
    }
}
