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
