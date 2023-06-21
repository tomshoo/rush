use lexer::token::{Identifier, Literal, Operator};
use std::{fmt::Display, write};

/// expression -> expression
///             | end
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
/// literal    -> [`Literal`]
#[must_use = "An expression tree must be used"]
pub enum Expression {
    BinaryExp {
        left: Box<Expression>,
        op: Operator,
        right: Box<Expression>,
    },

    UnaryOp {
        operand: Box<Expression>,
        operator: Operator,
    },

    Expr(Box<Expression>),
    Literal(Literal),
    Identifier(Identifier),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::BinaryExp { left, right, op } => {
                write!(f, "(binary {} {:?} {})", left, op, right)
            },

            Expression::UnaryOp { operand, operator } => {
                write!(f, "(unary {} {:?})", operand, operator)
            },

            Expression::Expr(expr) => write!(f, "(expr {})", expr),
            Expression::Literal(literal) => write!(f, "(literal {:?})", literal),
            Expression::Identifier(ident) => write!(f, "(identifier {})", ident),
        }
    }
}

impl Expression {
    pub fn yield_expr(&self) -> String {
        if let Self::Identifier(ident) = self {
            ident.name().to_owned()
        } else {
            format!("{}", self)
        }
    }
}

#[test]
fn display_test() {
    use lexer::token;
    use Expression::*;

    let expr = Expr(Box::new(BinaryExp {
        left: Box::new(Literal(token::Literal::Number(1))),
        op: Operator::Arithmetic(lexer::token::ArithmeticOperator::Plus),
        right: Box::new(BinaryExp {
            left: Box::new(Literal(token::Literal::Number(2))),
            op: Operator::Arithmetic(lexer::token::ArithmeticOperator::Minus),
            right: Box::new(UnaryOp {
                operand: Box::new(Literal(token::Literal::Number(3))),
                operator: Operator::Conditional(lexer::token::ConditionalOperator::Not),
            }),
        }),
    }));

    assert_eq!(
        expr.to_string(),
        "(expr (binary (literal Number(1)) Arithmetic(Plus) (binary (literal Number(2)) Arithmetic(Minus) (unary (literal Number(3)) Conditional(Not)))))".to_string()
    );
}
