use std::fmt::Display;
use lexer::token::{Operator, LiteralKind};

/// expression -> expression
///             | end
///
/// end        -> $
///
/// expression -> binary
///             | unary
///             | expression
///             | literal
///
/// binary     -> left operator right
///
/// unary      -> (operator operand)
///             | (operand operator)
///
/// left       -> operand
///
/// right      -> operand
///
/// operand    -> literal
///             | expression
///
/// operator   -> [`Operator`]
/// literal    -> [`LiteralKind`]
#[must_use = "An expression tree must be used"]
#[derive(Default)]
pub enum Expression {
    BinaryExp {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>
    },

    UnaryOp {
        operand: Box<Expression>,
        operator: Operator
    },

    Expr(Box<Expression>),
    Literal(LiteralKind),

    #[default]
    End,
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Expr(expr)                    => write!(f, "(expr {})", expr),
            Expression::BinaryExp { left, right, op } => write!(f, "(binary {} {:?} {})", left, op, right),
            Expression::UnaryOp { operand, operator } => write!(f, "(unary {} {:?})", operand, operator),
            Expression::Literal(literal)              => write!(f, "(literal {:?})", literal),

            Expression::End => write!(f, "(end)"),
        }
    }
}

#[test]
fn display_test() {
    use Expression::*;
    let expr = Expr(Box::new(BinaryExp {
        left: Box::new(Literal(LiteralKind::Number(1))),
        op: Operator::Arithmetic(lexer::token::ArithmeticOperator::Plus),
        right: Box::new(BinaryExp {
            left: Box::new(Literal(LiteralKind::Number(2))),
            op: Operator::Arithmetic(lexer::token::ArithmeticOperator::Minus),
            right: Box::new(UnaryOp {
                operand:Box::new(Literal(LiteralKind::Number(3))),
                operator: Operator::Conditional(lexer::token::ConditionalOperator::Not),
            })
        })
    }));

    assert_eq!(
        expr.to_string(),
        "(expr (binary (literal Number(1)) Arithmetic(Plus) (binary (literal Number(2)) Arithmetic(Minus) (unary (literal Number(3)) Conditional(Not)))))".to_string()
    );
}
