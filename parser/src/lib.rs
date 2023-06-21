#![allow(dead_code)]

pub mod error;
pub mod expression;
pub mod statement;

use lexer::{token::Token, Lexer};
use statement::Statement;
use std::iter::Peekable;

type Result<T> = std::result::Result<T, error::Error>;

pub struct Parser<'parser> {
    generator: Peekable<Lexer<'parser>>,
    stack: Vec<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(generator: Lexer<'p>) -> Self {
        let generator = generator.peekable();
        let stack = vec![];

        Self { generator, stack }
    }

    fn parse_let(&mut self) -> Option<<Self as Iterator>::Item> {
        None
    }
}

impl Iterator for Parser<'_> {
    type Item = Result<Statement>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = match self.generator.next()? {
            Ok(token) => token,
            Err(e) => return Some(Err(e.into())),
        };

        match token {
            Token::Keyword(lexer::token::Keyword::Let) => self.parse_let(),
            _ => None,
        }
    }
}
