use std::fmt::{self, Display};

// All available datatypes excluding string and collection
#[derive(Debug)]
pub enum DataType {
    Float,
    Number,
    Boolean,
    String,
}

// Enum to evaluate token type flexibly using match case
#[derive(Debug)]
pub enum TokenType {
    Evaluatable(String),
    Keyword(String),
    Operator(String),
    DataType(DataType),
    Token,
}

// Token Type to hold each token signature
#[derive(Debug)]
#[allow(dead_code)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub follow: bool,
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token(value={}, type={:?})",
            &self.value, &self.token_type
        )
    }
}
