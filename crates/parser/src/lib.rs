#![allow(unused)]

use lexer::Lexer;

pub struct ParseTree;

pub struct Parser<'p> {
    token_generator: Lexer<'p>,
}

impl<'p> Parser<'p> {
    pub fn new(token_generator: Lexer<'p>) -> Self {
        Self { token_generator }
    }
}
