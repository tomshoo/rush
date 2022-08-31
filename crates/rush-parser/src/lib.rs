pub mod dtype;
pub mod parsers;
use dtype::SMType;
use lazy_static::lazy_static;
use std::collections::HashMap;
use token::TokenType;

pub trait IntoString: Into<String> + std::fmt::Debug + std::fmt::Display + Clone {}
impl<T: Into<String> + std::fmt::Debug + std::fmt::Display + Clone> IntoString for T {}

lazy_static! {
    static ref TOKEN_MAP: HashMap<&'static str, TokenType> = HashMap::from([
        (".", TokenType::Operator("SCOPE_LOCATION")),
        ("$", TokenType::Operator("EXPAND")),
        ("=", TokenType::Operator("ASSIGN")),
        ("#!", TokenType::Operator("SHEBANG")),
        ("::", TokenType::Operator("DECLARE_TYPE")),
        ("-=", TokenType::Operator("DECREMENT_ASSIGNMENT")),
        ("+=", TokenType::Operator("INCREMENT_ASSIGNMENT")),
        ("==", TokenType::Operator("CHECK_EQUALS")),
        ("||", TokenType::Operator("DISJUNCTION")),
        ("&&", TokenType::Operator("CONJUNCTION")),
        ("?#", TokenType::Operator("ALTERNATE_SYMBOL_NAME")),
        ("in", TokenType::Keyword("IN_RANGE")),
        ("if", TokenType::Keyword("CHECK_CONDITION")),
        ("for", TokenType::Keyword("ITERATE_RANGED")),
        ("dyn", TokenType::Keyword("DYNAMIC_ASSIGNMENT")),
        ("mut", TokenType::Keyword("MUTABLE_ASSIGNMENT")),
        ("let", TokenType::Keyword("VARIABLE_ASSIGNMENT")),
        ("else", TokenType::Keyword("CHECK_CONDITION_INVERSE")),
        ("case", TokenType::Keyword("PATTERN_MATCH")),
        ("exit", TokenType::Keyword("TERMINATE_PROCESS")),
        ("true", TokenType::Keyword("BOOLEAN")),
        ("false", TokenType::Keyword("BOOLEAN")),
        ("while", TokenType::Keyword("ITERATE_CONDITIONAL")),
        ("switch", TokenType::Keyword("CONDITIONAL_PATTERN")),
        ("return", TokenType::Keyword("TERMINATE_FUNCTION")),
        ("Number", TokenType::Keyword("DATA_TYPE_NUM")),
        ("String", TokenType::Keyword("DATA_TYPE_STRING")),
        ("Float", TokenType::Keyword("DATA_TYPE_FLOAT")),
        ("Boolean", TokenType::Keyword("DATA_TYPE_BOOLEAN")),
    ]);
}

pub mod token {
    type TokenItemType = crate::SMType<String, Token>;
    use std::fmt::{self, Display};
    use std::hash::{Hash, Hasher};

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

    impl Display for Token {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.value {
                TokenItemType::Single(string) => {
                    write!(f, "Token(value={}, type={:?})", string, &self.type_)
                }
                TokenItemType::Multiple(vec) => {
                    let mut string = String::from("\n");
                    for token in vec {
                        string.push_str(&format!("\t[{}]\n", token));
                    }
                    write!(f, "Token(values=[{}], type={:?})", string, &self.type_)
                }
            }
        }
    }

    impl PartialEq for Token {
        fn eq(&self, other: &Token) -> bool {
            if let TokenType::Token = other.type_ {
                return true;
            }
            &self.value == &other.value && &self.type_ == &other.type_
        }
    }

    impl Hash for Token {
        fn hash<H: Hasher>(&self, state: &mut H) {
            self.type_.hash(state);
        }
    }
}
