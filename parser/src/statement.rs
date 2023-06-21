use lexer::token::Identifier;

use crate::expression::Expression;

#[derive(Default)]
pub enum Statement {
    Let {
        ident: Identifier,
        expr: Expression,
    },

    #[default]
    End,
}
