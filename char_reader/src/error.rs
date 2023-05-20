use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    IoError(#[from] std::io::Error),

    #[error("invalid utf-8 sequence")]
    InvalidUtf8,
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::IoError(e) => e,
            Error::InvalidUtf8 => Self::new(std::io::ErrorKind::InvalidData, value),
        }
    }
}
