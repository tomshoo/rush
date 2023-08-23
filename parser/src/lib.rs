#![allow(dead_code)]

pub mod error;
pub mod expression;
pub mod statement;

use lexer::{token::Token, Lexer};
use rush_core::lazybuf::LazyBuf;
use statement::Statement;

type Result<T> = std::result::Result<T, error::Error>;

pub struct Parser<'parser> {
    generator: LazyBuf<'parser, Lexer<'parser>>,
    stack: Vec<Token>,
}

impl<'p> Parser<'p> {
    pub fn new(generator: Lexer<'p>) -> Self {
        let generator = rush_core::BufferExt::lazy_buf(generator);
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
            Err(e) => return Some(Err(e.clone().into())),
        };

        match token {
            Token::Keyword(lexer::token::Keyword::Let) => self.parse_let(),
            _ => None,
        }
    }
}
