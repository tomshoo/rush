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
