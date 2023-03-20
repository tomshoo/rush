use crate::error::IdError;
use phf::{phf_map, Map};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Operator(Operator),
    Delimitter(Delimitter),
    Literal(LiteralKind),
    Keyword(Keyword),
    Identifier,
}

impl Kind {
    /// Returns `true` if the kind is [`Operator`].
    ///
    /// [`Operator`]: Kind::Operator
    #[must_use]
    pub fn is_operator(&self) -> bool {
        matches!(self, Self::Operator(..))
    }

    /// Returns `true` if the kind is [`Delimitter`].
    ///
    /// [`Delimitter`]: Kind::Delimitter
    #[must_use]
    pub fn is_delimitter(&self) -> bool {
        matches!(self, Self::Delimitter(..))
    }

    /// Returns `true` if the kind is [`Keyword`].
    ///
    /// [`Keyword`]: Kind::Keyword
    #[must_use]
    pub fn is_keyword(&self) -> bool {
        matches!(self, Self::Keyword(..))
    }

    /// Returns `true` if the kind is [`Literal`].
    ///
    /// [`Literal`]: Kind::Literal
    #[must_use]
    pub fn is_literal(&self) -> bool {
        matches!(self, Self::Literal(..))
    }

    /// Returns `true` if the kind is [`Identifier`].
    ///
    /// [`Identifier`]: Kind::Identifier
    #[must_use]
    pub fn is_identifier(&self) -> bool {
        matches!(self, Self::Identifier)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    // Arithmetic operators
    Plus,
    Minus,
    IncrAssign,
    DecrAssign,
    Multiply,
    Divide,

    // Bitwise operators
    Xor,
    XorAssign,
    BitWiseAnd,
    BitWiseAndAssign,
    BitWiseOr,
    BitWiseOrAssign,
    LeftShift,
    RightShift,

    // Conditional operators
    And,
    Or,
    Not,

    // Relational operators
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,

    // Misc operators
    Assign,
    FatArrow,
    ThinArrow,
    ScopeResolution,

    // Range operators
    InclusiveRange,
    ExclusiveRange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Delimitter {
    Tilde,
    BackTick,
    At,
    Pound,
    Dollar,
    LParen,
    RParen,
    LCurly,
    RCurly,
    LSquare,
    RSquare,
    BackSlash,
    Colon,
    SemiColon,
    Comma,
    Dot,
    Question,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Keyword {
    For,
    While,
    If,
    Else,
    Const,
    Let,
    Break,
    Return,
    Struct,
    Enum,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LiteralKind {
    HexChar,
    ByteChar,
    Char,
    String,
    Number,
    Float,
    BooleanTrue,
    BooleanFalse,
}

impl LiteralKind {
    /// Returns `true` if the literal kind is [`HexChar`].
    ///
    /// [`HexChar`]: LiteralKind::HexChar
    #[must_use]
    pub fn is_hex_char(&self) -> bool {
        matches!(self, Self::HexChar)
    }

    /// Returns `true` if the literal kind is [`ByteChar`].
    ///
    /// [`ByteChar`]: LiteralKind::ByteChar
    #[must_use]
    pub fn is_byte_char(&self) -> bool {
        matches!(self, Self::ByteChar)
    }

    /// Returns `true` if the literal kind is [`Char`].
    ///
    /// [`Char`]: LiteralKind::Char
    #[must_use]
    pub fn is_char(&self) -> bool {
        matches!(self, Self::Char)
    }

    /// Returns `true` if the literal kind is [`String`].
    ///
    /// [`String`]: LiteralKind::String
    #[must_use]
    pub fn is_string(&self) -> bool {
        matches!(self, Self::String)
    }

    /// Returns `true` if the literal kind is [`Number`].
    ///
    /// [`Number`]: LiteralKind::Number
    #[must_use]
    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number)
    }

    /// Returns `true` if the literal kind is [`Float`].
    ///
    /// [`Float`]: LiteralKind::Float
    #[must_use]
    pub fn is_float(&self) -> bool {
        matches!(self, Self::Float)
    }

    /// Returns `true` if the literal kind is [`BooleanTrue`].
    ///
    /// [`BooleanTrue`]: LiteralKind::BooleanTrue
    #[must_use]
    pub fn is_boolean_true(&self) -> bool {
        matches!(self, Self::BooleanTrue)
    }

    /// Returns `true` if the literal kind is [`BooleanFalse`].
    ///
    /// [`BooleanFalse`]: LiteralKind::BooleanFalse
    #[must_use]
    pub fn is_boolean_false(&self) -> bool {
        matches!(self, Self::BooleanFalse)
    }
}

pub(crate) const TOKENS: Map<&'static str, Kind> = phf_map! {
    "~"  => Kind::Delimitter(Delimitter::Tilde),
    "`"  => Kind::Delimitter(Delimitter::BackTick),
    "@"  => Kind::Delimitter(Delimitter::At),
    "#"  => Kind::Delimitter(Delimitter::Pound),
    "$"  => Kind::Delimitter(Delimitter::Dollar),
    "("  => Kind::Delimitter(Delimitter::LParen),
    ")"  => Kind::Delimitter(Delimitter::RParen),
    "["  => Kind::Delimitter(Delimitter::LSquare),
    "]"  => Kind::Delimitter(Delimitter::RSquare),
    "{"  => Kind::Delimitter(Delimitter::LCurly),
    "}"  => Kind::Delimitter(Delimitter::RCurly),
    "\\" => Kind::Delimitter(Delimitter::BackSlash),
    ";"  => Kind::Delimitter(Delimitter::SemiColon),
    ":"  => Kind::Delimitter(Delimitter::Colon),
    ","  => Kind::Delimitter(Delimitter::Comma),
    "."  => Kind::Delimitter(Delimitter::Dot),
    "?"  => Kind::Delimitter(Delimitter::Question),

    "for"    => Kind::Keyword(Keyword::For),
    "while"  => Kind::Keyword(Keyword::While),
    "if"     => Kind::Keyword(Keyword::If),
    "else"   => Kind::Keyword(Keyword::Else),
    "const"  => Kind::Keyword(Keyword::Const),
    "let"    => Kind::Keyword(Keyword::Let),
    "break"  => Kind::Keyword(Keyword::Break),
    "return" => Kind::Keyword(Keyword::Return),
    "struct" => Kind::Keyword(Keyword::Struct),
    "enum"   => Kind::Keyword(Keyword::Enum),

    "+"  => Kind::Operator(Operator::Plus),
    "-"  => Kind::Operator(Operator::Minus),
    "+=" => Kind::Operator(Operator::IncrAssign),
    "-=" => Kind::Operator(Operator::DecrAssign),
    "*"  => Kind::Operator(Operator::Multiply),
    "/"  => Kind::Operator(Operator::Divide),

    "^"  => Kind::Operator(Operator::Xor),
    "^=" => Kind::Operator(Operator::XorAssign),
    "&"  => Kind::Operator(Operator::BitWiseAnd),
    "&=" => Kind::Operator(Operator::BitWiseAndAssign),
    "|"  => Kind::Operator(Operator::BitWiseOr),
    "|=" => Kind::Operator(Operator::BitWiseOrAssign),
    "<<" => Kind::Operator(Operator::LeftShift),
    ">>" => Kind::Operator(Operator::RightShift),

    "&&" => Kind::Operator(Operator::And),
    "||" => Kind::Operator(Operator::Or),
    "!"  => Kind::Operator(Operator::Not),

    ">"  => Kind::Operator(Operator::GreaterThan),
    ">=" => Kind::Operator(Operator::GreaterThanOrEqual),
    "<"  => Kind::Operator(Operator::LessThan),
    "<=" => Kind::Operator(Operator::LessThanOrEqual),
    "==" => Kind::Operator(Operator::Equal),
    "!=" => Kind::Operator(Operator::NotEqual),

    "="  => Kind::Operator(Operator::Assign),
    "=>" => Kind::Operator(Operator::FatArrow),
    "->" => Kind::Operator(Operator::ThinArrow),
    "::" => Kind::Operator(Operator::ScopeResolution),

    "..=" => Kind::Operator(Operator::InclusiveRange),
    ".."  => Kind::Operator(Operator::ExclusiveRange),

    "true"  => Kind::Literal(LiteralKind::BooleanTrue),
    "false" => Kind::Literal(LiteralKind::BooleanFalse),
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token {
    value: String,
    kind: Kind,
}

pub(crate) fn is_valid_identifier(string: &str) -> bool {
    string
        .char_indices()
        .filter(|(i, c)| {
            (if *i == 0 {
                c.is_alphabetic()
            } else {
                c.is_alphanumeric()
            }) || *c == '_'
        })
        .count()
        == string.len()
}

impl TryFrom<String> for Token {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if let Some(kind) = TOKENS.get(&value) {
            Ok(Self { value, kind: *kind })
        } else if is_valid_identifier(&value) {
            Ok(Self {
                value,
                kind: Kind::Identifier,
            })
        } else if value.parse::<usize>().is_ok() {
            Ok(Self {
                value,
                kind: Kind::Literal(LiteralKind::Number),
            })
        } else {
            Err(IdError::UnidentifiedToken(value))
        }
    }
}

impl TryFrom<&str> for Token {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let value = value.to_string();
        Self::try_from(value)
    }
}

impl Token {
    pub fn new(value: String, kind: Kind) -> Self {
        Self { value, kind }
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
