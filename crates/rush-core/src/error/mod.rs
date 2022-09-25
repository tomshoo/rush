pub mod lexer;

use std::fmt::{self, Debug};

pub struct RushError {
    kind: String,
    msg: String,
}

impl Debug for RushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}(message: {})", self.kind, self.msg)
    }
}

impl<T: Into<String>> From<T> for RushError {
    fn from(msg: T) -> Self {
        Self {
            kind: "Generic".into(),
            msg: msg.into(),
        }
    }
}
