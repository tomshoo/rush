use crate::Tracker;

#[derive(Debug)]
pub enum IRError {
    UnexpectedOperator(String),
}

pub enum SVTError {
    RequiredOperand(String),
}

pub enum LexerError {
    UnexpectedCharacter(Tracker, char),
    UnescapableCharacter(Tracker, String),
    UndeterminedType(char),
}

pub struct RushError {
    kind: String,
    msg: String,
}

impl From<IRError> for RushError {
    fn from(err: IRError) -> Self {
        match err {
            IRError::UnexpectedOperator(msg) => Self {
                kind: "IRError::UnexpectedOperator".into(),
                msg,
            },
        }
    }
}

impl From<SVTError> for RushError {
    fn from(err: SVTError) -> Self {
        match err {
            SVTError::RequiredOperand(msg) => Self {
                kind: "IRError::UnexpectedOperator".into(),
                msg,
            },
        }
    }
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
        }
    }
}

impl std::fmt::Debug for RushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(message: {})", self.kind, self.msg)
    }
}

impl<T: Into<String>> From<T> for RushError {
    fn from(msg: T) -> Self {
        Self {
            kind: "General".into(),
            msg: msg.into(),
        }
    }
}
