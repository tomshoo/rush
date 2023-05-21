use std::{fmt::Display, str::FromStr};

use crate::error::IdError;
use phf::{phf_map, Map};

#[derive(Debug, Clone)]
pub enum Token {
    Operator(Operator),
    Delimitter(Delimitter),
    Literal(LiteralKind),
    Keyword(Keyword),
    Identifier(String),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operator {
    Bitwise(BitwiseOperator),
    Arithmetic(ArithmeticOperator),

    Relational(RelationalOperator),
    Conditional(ConditionalOperator),

    Range(RangeOperator),
    Misc(MiscOperator),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArithmeticOperator {
    Divide,
    Multiply,
    Plus,
    Minus,
    IncrAssign,
    DecrAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BitwiseOperator {
    BitWiseNot,
    LeftShift,
    RightShift,
    BitWiseAnd,
    BitWiseOr,
    Xor,
    BitWiseAndAssign,
    BitWiseOrAssign,
    XorAssign,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConditionalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RelationalOperator {
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Equal,
    NotEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum RangeOperator {
    InclusiveRange,
    ExclusiveRange,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MiscOperator {
    Assign,
    FatArrow,
    ThinArrow,
    ScopeResolution,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Delimitter {
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Char(char),
    String(String),
    Number(isize),
    Float(f64),
    Boolean(bool),
    Nil,
}

pub(crate) const TOKENS: Map<&'static str, Token> = phf_map! {
    "`"  => Token::Delimitter(Delimitter::BackTick),
    "@"  => Token::Delimitter(Delimitter::At),
    "#"  => Token::Delimitter(Delimitter::Pound),
    "$"  => Token::Delimitter(Delimitter::Dollar),
    "("  => Token::Delimitter(Delimitter::LParen),
    ")"  => Token::Delimitter(Delimitter::RParen),
    "["  => Token::Delimitter(Delimitter::LSquare),
    "]"  => Token::Delimitter(Delimitter::RSquare),
    "{"  => Token::Delimitter(Delimitter::LCurly),
    "}"  => Token::Delimitter(Delimitter::RCurly),
    "\\" => Token::Delimitter(Delimitter::BackSlash),
    ";"  => Token::Delimitter(Delimitter::SemiColon),
    ":"  => Token::Delimitter(Delimitter::Colon),
    ","  => Token::Delimitter(Delimitter::Comma),
    "."  => Token::Delimitter(Delimitter::Dot),
    "?"  => Token::Delimitter(Delimitter::Question),

    "for"    => Token::Keyword(Keyword::For),
    "while"  => Token::Keyword(Keyword::While),
    "if"     => Token::Keyword(Keyword::If),
    "else"   => Token::Keyword(Keyword::Else),
    "const"  => Token::Keyword(Keyword::Const),
    "let"    => Token::Keyword(Keyword::Let),
    "break"  => Token::Keyword(Keyword::Break),
    "return" => Token::Keyword(Keyword::Return),
    "struct" => Token::Keyword(Keyword::Struct),
    "enum"   => Token::Keyword(Keyword::Enum),

    "nil" => Token::Literal(LiteralKind::Nil),

    "+"  => Token::Operator(Operator::Arithmetic(ArithmeticOperator::Plus)),
    "-"  => Token::Operator(Operator::Arithmetic(ArithmeticOperator::Minus)),
    "+=" => Token::Operator(Operator::Arithmetic(ArithmeticOperator::IncrAssign)),
    "-=" => Token::Operator(Operator::Arithmetic(ArithmeticOperator::DecrAssign)),
    "*"  => Token::Operator(Operator::Arithmetic(ArithmeticOperator::Multiply)),
    "/"  => Token::Operator(Operator::Arithmetic(ArithmeticOperator::Divide)),

    "~"  => Token::Operator(Operator::Bitwise(BitwiseOperator::BitWiseNot)),
    "^"  => Token::Operator(Operator::Bitwise(BitwiseOperator::Xor)),
    "^=" => Token::Operator(Operator::Bitwise(BitwiseOperator::XorAssign)),
    "&"  => Token::Operator(Operator::Bitwise(BitwiseOperator::BitWiseAnd)),
    "&=" => Token::Operator(Operator::Bitwise(BitwiseOperator::BitWiseAndAssign)),
    "|"  => Token::Operator(Operator::Bitwise(BitwiseOperator::BitWiseOr)),
    "|=" => Token::Operator(Operator::Bitwise(BitwiseOperator::BitWiseOrAssign)),
    "<<" => Token::Operator(Operator::Bitwise(BitwiseOperator::LeftShift)),
    ">>" => Token::Operator(Operator::Bitwise(BitwiseOperator::RightShift)),

    "&&" => Token::Operator(Operator::Conditional(ConditionalOperator::And)),
    "||" => Token::Operator(Operator::Conditional(ConditionalOperator::Or)),
    "!"  => Token::Operator(Operator::Conditional(ConditionalOperator::Not)),

    ">"  => Token::Operator(Operator::Relational(RelationalOperator::GreaterThan)),
    ">=" => Token::Operator(Operator::Relational(RelationalOperator::GreaterThanOrEqual)),
    "<"  => Token::Operator(Operator::Relational(RelationalOperator::LessThan)),
    "<=" => Token::Operator(Operator::Relational(RelationalOperator::LessThanOrEqual)),
    "==" => Token::Operator(Operator::Relational(RelationalOperator::Equal)),
    "!=" => Token::Operator(Operator::Relational(RelationalOperator::NotEqual)),

    "="  => Token::Operator(Operator::Misc(MiscOperator::Assign)),
    "=>" => Token::Operator(Operator::Misc(MiscOperator::FatArrow)),
    "->" => Token::Operator(Operator::Misc(MiscOperator::ThinArrow)),
    "::" => Token::Operator(Operator::Misc(MiscOperator::ScopeResolution)),

    "..=" => Token::Operator(Operator::Range(RangeOperator::InclusiveRange)),
    ".."  => Token::Operator(Operator::Range(RangeOperator::ExclusiveRange)),

    "true"  => Token::Literal(LiteralKind::Boolean(true)),
    "false" => Token::Literal(LiteralKind::Boolean(false)),
};

pub(crate) fn is_valid_identifier(string: &str) -> bool {
    string
        .char_indices()
        .filter(|(i, c)| (if *i == 0 { c.is_alphabetic() } else { c.is_alphanumeric() }) || *c == '_')
        .count()
        == string.len()
}

impl FromStr for Token {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(kind) = TOKENS.get(s) {
            Ok(kind.clone())
        } else if is_valid_identifier(s) {
            Ok(Self::Identifier(s.to_owned()))
        } else if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Literal(LiteralKind::Number(num)))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Self::Literal(LiteralKind::Float(f)))
        } else {
            Err(IdError::UnidentifiedToken(s.to_owned()))
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
