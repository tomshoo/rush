use rush_core::{error::LexerError, interfaces::smtype::SMulType, Tracker};
use std::{iter::Peekable, str::Chars};

// Token properties
use crate::token::DataType as Type;
use crate::token::Token as TokenItem;
use crate::token::TokenType::{self, *};
use crate::TOKEN_MAP;

#[inline(always)]
fn string_type(string: &str) -> TokenType {
    if let Ok(_) = string.parse::<isize>() {
        DataType(Type::Number)
    } else if let Ok(_) = string.parse::<f64>() {
        DataType(Type::Float)
    } else if let Some(property) = TOKEN_MAP.get(string) {
        *property
    } else {
        Token
    }
}

#[inline(always)]
fn token_type(string: &str) -> TokenType {
    if let Some(property) = TOKEN_MAP.get(string) {
        *property
    } else {
        Operator("SPECIAL")
    }
}

#[inline(always)]
fn muxed_ref<'m>(
    brace_stack: &'m mut Vec<(Vec<TokenItem>, (crate::token::TokenType, char))>,
    token_stack: &'m mut Vec<TokenItem>,
) -> &'m mut Vec<TokenItem> {
    match brace_stack.last_mut() {
        Some(last) => &mut last.0,
        None => token_stack,
    }
}

fn flush(stack: &mut Vec<TokenItem>, ch: char) -> rush_core::Result<()> {
    Ok(
        if let Some((content, Token, false)) = stack.last_mut().map_or(None, |node| {
            node.value
                .get_single_mut()
                .map_or(None, |content| Some((content, node.type_, node.follow)))
        }) {
            content.push(ch);
        } else {
            stack.push(TokenItem::from_char(ch)?);
        },
    )
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
        Token => item.type_ = string_type(&item.value.get_single().unwrap()),
        Operator(_) => item.type_ = token_type(&item.value.get_single().unwrap()),
        _ => {}
    };
    item.follow = true;
}

#[allow(unused_mut, unused_variables)]
fn lexer<'a>(
    stream: &mut Peekable<Chars>,
    tracker: &mut Tracker,
) -> rush_core::Result<Vec<TokenItem>> {
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
                    .map_or((), |entry| entry.value.get_single_mut().unwrap().push(ch))
            }
        } else if let Some(entry) = block_type(ch) {
            brace_stack.push((vec![], entry));
        } else if ch.is_alphanumeric() || ch == '_' {
            flush(muxed_ref(&mut brace_stack, &mut token_stack), ch)?;
        } else if ch == ')' || ch == '}' || ch == ']' {
            let verify = brace_stack.last().map_or(false, |entry| ch == entry.1 .1);
            if verify {
                let last = brace_stack.pop().unwrap();
                if let Some(entry) = brace_stack.last_mut() {
                    entry.0.push(TokenItem {
                        value: SMulType::Multiple(last.0),
                        type_: last.1 .0,
                        follow: false,
                    })
                } else {
                    token_stack.push(TokenItem {
                        value: SMulType::Multiple(last.0),
                        type_: last.1 .0,
                        follow: false,
                    })
                }
            } else {
                return Err(LexerError::UnexpectedCharacter(*tracker, ch))?;
            }
        } else if ch.is_whitespace() {
            muxed_ref(&mut brace_stack, &mut token_stack)
                .last_mut()
                .map_or((), auto_assign_type);
        } else if ch == '\\' {
            match stream.next() {
                Some('\n') => {
                    return Err(LexerError::UnescapableCharacter(*tracker, "EOL".into()))?;
                }
                Some(item) => {
                    tracker.update_col();
                    flush(muxed_ref(&mut brace_stack, &mut token_stack), item)?;
                }
                None => return Err(LexerError::UnescapableCharacter(*tracker, "EOF".into()))?,
            }
        } else {
            let stack = muxed_ref(&mut brace_stack, &mut token_stack);
            if let Some((content, type_, follow)) = stack.last_mut().map_or(None, |node| {
                node.value.get_single_mut().map_or(None, |content| {
                    Some((content, &mut node.type_, node.follow))
                })
            }) {
                if let Operator(_) = type_ {
                    if follow {
                        stack.push(TokenItem::from_char(ch)?);
                    } else {
                        content.push(ch);
                        *type_ = token_type(content);
                    }
                } else {
                    if let Token = type_ {
                        *type_ = string_type(content);
                    };
                    stack.push(TokenItem::from_char(ch)?);
                }
            } else {
                stack.push(TokenItem::from_char(ch)?);
            }
        }
    }
    match brace_stack.last() {
        Some(last) => Err(LexerError::UnexpectedCharacter(*tracker, last.1 .1))?,
        None => {
            token_stack.last_mut().map_or((), auto_assign_type);
            Ok(token_stack)
        }
    }
}

pub fn lexer_charwise<'a>(stream: &'a str) -> rush_core::Result<Vec<TokenItem>> {
    let mut tracker = Tracker::new();
    lexer(&mut stream.chars().peekable(), &mut tracker)
}
