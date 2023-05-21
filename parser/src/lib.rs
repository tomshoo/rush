pub mod expression;

use expression::Expression;
use lexer::token::*;

pub struct Parser {
    stack: Vec<Token>,
}

impl Parser {
    pub fn generate_parser_tree(token_generator: impl Iterator<Item = Token>) -> Expression {
        let this = &mut Self { stack: vec![] };
        this.generate_prefix_stack(token_generator);

        while let Some(token) = this.stack.pop() {
            println!("{}", token);
        }

        Default::default()
    }

    fn generate_prefix_stack(&mut self, token_generator: impl Iterator<Item = Token>) {
        let mut lstack = vec![];

        for token in token_generator {
            match token {
                Token::Literal(..)    => self.stack.push(token.clone()),
                Token::Identifier(..) => self.stack.push(token.clone()),

                Token::Operator(Operator::Arithmetic(ref op)) => {
                    while 
                        matches!(lstack.last(), Some(Token::Operator(Operator::Arithmetic(s_op))) if s_op < op)
                    {
                        self.stack.push(lstack.pop().unwrap());
                    }
                    lstack.push(token.clone())
                },

                Token::Delimitter(Delimitter::LParen) => {
                    lstack.push(token.clone())
                },

                Token::Delimitter(Delimitter::RParen) => {
                    while let Some(s_token) = lstack.pop() {
                        if let Token::Delimitter(Delimitter::LParen) = s_token { break; }
                        self.stack.push(s_token);
                    }
                },

                _ => {
                    unimplemented!()
                },
            }
        }

        if ! lstack.is_empty() {
            while let Some(tok) = lstack.pop() { self.stack.push(tok) }
        } else {
            eprintln!("invalid operations");
        }
    }
}
