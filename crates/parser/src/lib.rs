#![allow(unused)]

use lexer::Lexer;

pub struct ParseTree;

pub struct Parser {
    token_generator: Lexer,
}

impl Parser {
    pub fn new(token_generator: Lexer) -> Self {
        Self { token_generator }
    }
}
