use crate::error::IdError;
use phf::{phf_map, Map};
use std::{fmt::Display, str::FromStr, write};

pub use delimitter::Delimitter;
pub use identifier::Identifier;
pub use keyword::Keyword;
pub use literal::Literal;
pub use operator::*;

pub mod delimitter;
pub mod identifier;
pub mod keyword;
pub mod literal;
pub mod operator;

#[derive(Debug, Clone)]
pub enum Token {
    Operator(Operator),
    Delimitter(Delimitter),
    Literal(Literal),
    Keyword(Keyword),
    Identifier(Identifier),
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

    "nil" => Token::Literal(Literal::Nil),

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

    "true"  => Token::Literal(Literal::Boolean(true)),
    "false" => Token::Literal(Literal::Boolean(false)),
};

impl FromStr for Token {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(kind) = TOKENS.get(s) {
            Ok(kind.clone())
        } else if let Ok(ident) = s.parse() {
            Ok(Self::Identifier(ident))
        } else if let Ok(num) = s.parse::<isize>() {
            Ok(Self::Literal(Literal::Number(num)))
        } else if let Ok(f) = s.parse::<f64>() {
            Ok(Self::Literal(Literal::Float(f)))
        } else {
            Err(IdError::UnidentifiedToken(s.into()))
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::Literal(lit) => write!(f, "{:?}", lit),
            Self::Operator(op) => write!(f, "{:?}", op),
            Self::Keyword(kw) => write!(f, "{:?}", kw),
            Self::Delimitter(delm) => write!(f, "{:?}", delm),
        }
    }
}
