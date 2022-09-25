pub mod parsers;

use phf::phf_map;
use token::TokenType;

pub trait IntoString: Into<String> + std::fmt::Debug + std::fmt::Display + Clone {}
impl<T: Into<String> + std::fmt::Debug + std::fmt::Display + Clone> IntoString for T {}

const TOKEN_MAP: phf::Map<&'static str, TokenType> = phf_map!(
    "."       => TokenType::Operator("SCOPE_LOCATION"),
    "$"       => TokenType::Operator("EXPAND"),
    "="       => TokenType::Operator("ASSIGN"),
    "#!"      => TokenType::Operator("SHEBANG"),
    "::"      => TokenType::Operator("DECLARE_TYPE"),
    "-="      => TokenType::Operator("DECREMENT_ASSIGNMENT"),
    "+="      => TokenType::Operator("INCREMENT_ASSIGNMENT"),
    "=="      => TokenType::Operator("CHECK_EQUALS"),
    "||"      => TokenType::Operator("DISJUNCTION"),
    "&&"      => TokenType::Operator("CONJUNCTION"),
    "if"      => TokenType::Keyword("CHECK_CONDITION"),
    "else"    => TokenType::Keyword("CHECK_CONDITION_INVERSE"),
    "for"     => TokenType::Keyword("ITERATE_RANGED"),
    "in"      => TokenType::Keyword("IN_RANGE"),
    "let"     => TokenType::Keyword("VARIABLE_ASSIGNMENT"),
    "dyn"     => TokenType::Keyword("DYNAMIC_ASSIGNMENT"),
    "mut"     => TokenType::Keyword("MUTABLE_ASSIGNMENT"),
    "exit"    => TokenType::Keyword("TERMINATE_PROCESS"),
    "true"    => TokenType::Keyword("BOOLEAN"),
    "false"   => TokenType::Keyword("BOOLEAN"),
    "while"   => TokenType::Keyword("ITERATE_CONDITIONAL"),
    "switch"  => TokenType::Keyword("CONDITIONAL_PATTERN"),
    "case"    => TokenType::Keyword("PATTERN_MATCH"),
    "return"  => TokenType::Keyword("TERMINATE_FUNCTION"),
    "Number"  => TokenType::Keyword("DATA_TYPE_NUM"),
    "String"  => TokenType::Keyword("DATA_TYPE_STRING"),
    "Float"   => TokenType::Keyword("DATA_TYPE_FLOAT"),
    "Boolean" => TokenType::Keyword("DATA_TYPE_BOOLEAN"),
);

pub mod token {
    use rush_core::error::lexer::LexerError;
    use rush_core::interfaces::smtype::SMulType;
    use rush_core::Result;
    use std::fmt::{self, Display};
    use std::hash::{Hash, Hasher};

    use crate::TOKEN_MAP;

    type TokenItemType = SMulType<String, Token>;

    // All available datatypes excluding string and collection
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
    pub enum DataType {
        Float,
        Number,
        Boolean,
        String,
        Collection,
    }

    // Enum to evaluate token type flexibly using match case
    #[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
    pub enum TokenType {
        Evaluatable(&'static str),
        Keyword(&'static str),
        Operator(&'static str),
        DataType(DataType),
        Token,
    }

    // Token Type to hold each token signature
    #[derive(Debug, Clone, Eq)]
    pub struct Token {
        pub value: TokenItemType,
        pub type_: TokenType,
        pub follow: bool,
    }

    impl Token {
        pub fn from_char(ch: char) -> Result<Self> {
            if ch.is_alphanumeric() {
                Ok(Self {
                    value: SMulType::Single(ch.to_string()),
                    type_: TokenType::Token,
                    follow: false,
                })
            } else if ch.is_whitespace() {
                Err(LexerError::UndeterminedType(ch).into())
            } else {
                Ok(Self {
                    value: SMulType::Single(ch.to_string()),
                    type_: TOKEN_MAP
                        .get(ch.to_string().as_str())
                        .map_or(TokenType::Operator("SPECIAL"), |type_| *type_),
                    follow: false,
                })
            }
        }
    }

    impl Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.value {
                SMulType::Single(string) => {
                    write!(f, "Token(value={}, type={:?})", string, &self.type_)
                }
                SMulType::Multiple(vec) => {
                    writeln!(f, "Token(values=[")?;
                    for token in vec {
                        writeln!(f, "\t[{}]", token)?;
                    }
                    write!(f, ", type={:?})", &self.type_)
                }
            }
        }
    }

    impl PartialEq for Token {
        fn eq(&self, other: &Token) -> bool {
            if let TokenType::Token = other.type_ {
                return true;
            }
            self.value == other.value && self.type_ == other.type_
        }
    }

    impl Hash for Token {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.type_.hash(state);
        }

        fn hash_slice<H: Hasher>(data: &[Self], state: &mut H)
        where
            Self: Sized,
        {
            for piece in data {
                piece.hash(state);
            }
        }
    }
}
