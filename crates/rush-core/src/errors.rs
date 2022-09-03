pub type Result<T> = std::result::Result<T, RushError>;

#[derive(Debug)]
pub enum IRError {
    UnexpectedOperator(String),
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

impl std::fmt::Debug for RushError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(message: {})", self.kind, self.msg)
    }
}

impl<T: Into<String>> From<T> for RushError {
    fn from(msg: T) -> Self {
        Self { kind: "General".into(), msg: msg.into() }
    }
}