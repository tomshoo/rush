#[derive(Debug)]
pub enum IRError {
    UnexpectedOperator(String),
}

#[derive(Debug)]
pub enum SVTError {
    IRError(IRError),
}

impl From<IRError> for SVTError {
    fn from(err: IRError) -> Self {
        SVTError::IRError(err)
    }
}
