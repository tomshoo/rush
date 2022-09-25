use rush_core::error;
use rush_core::interfaces;

pub mod lexer {
    use super::*;
    use error::lexer::LexerError;
    use interfaces::{smtype::SMulType, tracker::Tracker};
    use std::{iter::Peekable, str::Chars};

    // Token properties
    use crate::token::DataType as Type;
    use crate::token::Token as TokenItem;
    use crate::token::TokenType::{self, *};
    use crate::TOKEN_MAP;

    #[inline(always)]
    fn string_type(string: &str) -> TokenType {
        if string.parse::<isize>().is_ok() {
            DataType(Type::Number)
        } else if string.parse::<f64>().is_ok() {
            DataType(Type::Float)
        } else if let Some(property) = TOKEN_MAP.get(string) {
            *property
        } else {
            Token
        }
    }

    #[inline(always)]
    fn token_type(string: &str) -> TokenType {
        match TOKEN_MAP.get(string) {
            Some(property) => *property,
            None => Operator("SPECIAL"),
        }
    }

    #[inline(always)]
    fn muxed_ref<'m>(
        brace_stack: &'m mut [(Vec<TokenItem>, (crate::token::TokenType, char))],
        token_stack: &'m mut Vec<TokenItem>,
    ) -> &'m mut Vec<TokenItem> {
        match brace_stack.last_mut() {
            Some(last) => &mut last.0,
            None => token_stack,
        }
    }

    fn flush(stack: &mut Vec<TokenItem>, ch: char) -> rush_core::LexerResult<()> {
        if let Some((content, Token, false)) = stack.last_mut().and_then(|node| {
            node.value
                .as_single_mut()
                .map(|content| (content, node.type_, node.follow))
        }) {
            content.push(ch);
        } else {
            stack.push(TokenItem::from_char(ch)?);
        };
        Ok(())
    }

    #[inline(always)]
    const fn block_type(ch: char) -> Option<(TokenType, char)> {
        match ch {
            '(' => Some((Evaluatable("PAREN"), ')')),
            '[' => Some((Evaluatable("SQURE"), ']')),
            '{' => Some((Evaluatable("BRACE"), '}')),
            _ => None,
        }
    }

    #[inline(always)]
    fn auto_assign_type(item: &mut TokenItem) {
        match item.type_ {
            Token => item.type_ = string_type(item.value.as_single().unwrap()),
            Operator(_) => item.type_ = token_type(item.value.as_single().unwrap()),
            _ => {}
        };
    }

    #[allow(unused_mut, unused_variables)]
    fn lexer(
        stream: &mut Peekable<Chars>,
        tracker: &mut Tracker,
    ) -> rush_core::LexerResult<Vec<TokenItem>> {
        let mut brace_stack: Vec<(Vec<TokenItem>, (crate::token::TokenType, char))> = vec![];
        let mut token_stack = vec![];
        let mut comment = false;
        let mut squote = false;
        let mut dquote = false;
        while let Some(ch) = stream.next() {
            if ch == '\n' {
                if comment {
                    comment = false;
                };
                tracker.update_row();
                tracker.reset_col();
                continue;
            } else {
                tracker.update_col();
            }

            if ch == '#' && stream.peek().map_or(false, |ch| ch == &'#') {
                comment = true;
            }

            if comment {
                continue;
            };

            if ch == '\'' && !dquote {
                squote = !squote;
            } else if ch == '"' && !squote {
                dquote = !dquote;
            }

            if squote || dquote {
                let stack = muxed_ref(&mut brace_stack, &mut token_stack);
                if (ch == '\'' && squote) || (ch == '"' && dquote) {
                    stack.push(TokenItem {
                        value: SMulType::Single(String::new()),
                        type_: DataType(Type::String),
                        follow: false,
                    })
                } else {
                    stack
                        .last_mut()
                        .map_or((), |entry| entry.value.as_single_mut().unwrap().push(ch))
                }
            } else if let Some(entry) = block_type(ch) {
                brace_stack.push((vec![], entry));
            } else if ch.is_alphanumeric() || ch == '_' {
                flush(muxed_ref(&mut brace_stack, &mut token_stack), ch)?;
            } else if ch == ')' || ch == '}' || ch == ']' {
                let verify = brace_stack.last().map_or(false, |entry| ch == entry.1 .1);
                if verify {
                    let last = brace_stack.pop().unwrap();
                    let push_mul = |stack: &mut Vec<TokenItem>, item, type_, follow| {
                        stack.push(TokenItem {
                            value: SMulType::Multiple(item),
                            type_,
                            follow,
                        })
                    };
                    if let Some(entry) = brace_stack.last_mut() {
                        push_mul(&mut entry.0, last.0, last.1 .0, false);
                    } else {
                        push_mul(&mut token_stack, last.0, last.1 .0, false);
                    }
                } else {
                    Err(LexerError::ExpectedCharacter(*tracker, ch))?;
                }
            } else if ch.is_whitespace() {
                muxed_ref(&mut brace_stack, &mut token_stack)
                    .last_mut()
                    .map_or((), |item| {
                        auto_assign_type(item);
                        item.follow = true
                    });
            } else if ch == '\\' {
                tracker.update_col();
                match stream.next() {
                    Some('\n') => Err(LexerError::UnescapableCharacter(*tracker, "EOL".into()))?,
                    Some(item) => {
                        flush(muxed_ref(&mut brace_stack, &mut token_stack), item)?;
                    }
                    None => Err(LexerError::UnescapableCharacter(*tracker, "EOF".into()))?,
                }
            } else {
                let stack = muxed_ref(&mut brace_stack, &mut token_stack);
                if let Some((content, item_type, follow)) = stack.last_mut().and_then(|node| {
                    node.value
                        .as_single_mut()
                        .map(|content| (content, &mut node.type_, node.follow))
                }) {
                    if let Operator(_) = item_type {
                        if follow {
                            stack.push(TokenItem::from_char(ch)?);
                        } else {
                            content.push(ch);
                            *item_type = token_type(content);
                        }
                    } else {
                        if let Token = item_type {
                            *item_type = string_type(content);
                        };
                        stack.push(TokenItem::from_char(ch)?);
                    }
                } else {
                    stack.push(TokenItem::from_char(ch)?);
                }
            }
        }
        match brace_stack.last() {
            Some(last) => Err(LexerError::ExpectedCharacter(*tracker, last.1 .1))?,
            None => {
                token_stack.last_mut().map_or((), auto_assign_type);
                Ok(token_stack)
            }
        }
    }

    pub fn lexer_charwise(stream: &str) -> rush_core::LexerResult<Vec<TokenItem>> {
        lexer(&mut stream.chars().peekable(), &mut Tracker::new())
    }
}
