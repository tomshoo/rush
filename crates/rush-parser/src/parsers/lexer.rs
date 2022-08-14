use std::iter::Peekable;
use std::str::Chars;

use super::syntree::TreeNode;
// Token properties
use super::syntree::analyzer::SyntaxValidationTree;
use crate::token::DataType as Type;
use crate::token::Token as TokenItem;
use crate::token::TokenItemType::*;
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

#[derive(Debug, Clone, Copy)]
struct Tracker {
    row: u16,
    col: u16,
}

impl Tracker {
    pub(crate) fn advance_col(&mut self) {
        self.col += 1;
    }

    pub(crate) fn advance_row(&mut self) {
        self.row += 1;
    }
}

pub fn lexer_charwise<'a>(
    syntax_tree: &SyntaxValidationTree,
    stream: &'a str,
) -> Result<Vec<TokenItem>, String> {
    let mut tracker = Tracker { row: 1, col: 0 };
    let result = lexer(
        create_validator(syntax_tree),
        &mut stream.chars().peekable(),
        &mut tracker,
    );
    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(e) => Err(format!("Expected {} at {:?}", &e.0, &e.1)),
    }
}

#[allow(dead_code)]
fn create_validator(_syntax_tree: &SyntaxValidationTree) -> impl Fn(TokenItem) -> bool {
    use std::cell::RefCell;
    use std::rc::Rc;
    let validator_stack: Rc<RefCell<Vec<Rc<TreeNode>>>> = Rc::new(RefCell::new(vec![]));
    move |_token: TokenItem| -> bool {
        let _ = Rc::clone(&validator_stack);
        return false;
    }
}

#[inline(always)]
fn muxed_ref<'m>(
    brace_stack: &'m mut Vec<(Vec<TokenItem>, (crate::token::TokenType, char))>,
    token_stack: &'m mut Vec<TokenItem>,
) -> &'m mut Vec<TokenItem> {
    let mux = !brace_stack.is_empty();
    if mux {
        &mut brace_stack.last_mut().unwrap().0
    } else {
        token_stack
    }
}

fn flush(
    brace_stack: &mut Vec<(Vec<TokenItem>, (crate::token::TokenType, char))>,
    token_stack: &mut Vec<TokenItem>,
    ch: char,
) {
    let new_token = || TokenItem {
        value: Single(ch.to_string()),
        type_: Token,
        follow: false,
    };
    let stack = muxed_ref(brace_stack, token_stack);
    if let Some(item) = stack.last_mut() {
        match &mut item.value {
            Single(content) => {
                if item.type_ == Token && !item.follow {
                    content.push(ch);
                } else {
                    stack.push(new_token())
                }
            }
            Multiple(_) => stack.push(TokenItem {
                value: Single(ch.to_string()),
                type_: Token,
                follow: false,
            }),
        }
    } else {
        stack.push(new_token())
    }
}

fn lexer<'a>(
    _validator: impl Fn(TokenItem) -> bool,
    stream: &mut Peekable<Chars>,
    tracker: &mut Tracker,
) -> Result<Vec<TokenItem>, (char, Tracker)> {
    let block_type = |ch: char| match ch {
        '(' => Some((Evaluatable("Expr"), ')')),
        '[' => Some((DataType(Type::Collection), ']')),
        '{' => Some((Evaluatable("Block"), '}')),
        _ => None,
    };
    let type_assign = |item: &mut TokenItem| {
        match item.type_ {
            Token => item.type_ = string_type(&item.value.get_string().unwrap()),
            Operator(_) => item.type_ = token_type(&item.value.get_string().unwrap()),
            _ => {}
        };
        item.follow = true
    };
    let mut brace_stack: Vec<(Vec<TokenItem>, (crate::token::TokenType, char))> = vec![];
    let mut token_stack = vec![];
    let mut comment = false;
    let (mut squote, mut dquote) = (false, false);
    while let Some(ch) = stream.next() {
        if ch == '\n' {
            if comment {
                comment = false;
            };
            tracker.advance_row();
            tracker.col = 0;
            continue;
        } else {
            tracker.advance_col();
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
                    value: Single(String::new()),
                    type_: DataType(Type::String),
                    follow: false,
                })
            } else {
                stack.last_mut().map_or((), |entry| {
                    let mut orig = entry.value.get_string().unwrap();
                    orig.push(ch);
                    entry.value = Single(orig);
                })
            }
        } else if let Some(entry) = block_type(ch) {
            brace_stack.push((vec![], entry));
        } else if ch.is_alphanumeric() || ch == '_' {
            flush(&mut brace_stack, &mut token_stack, ch);
        } else if ch == ')' || ch == '}' || ch == ']' {
            let verify = brace_stack.last().map_or(false, |entry| ch == entry.1 .1);
            if verify {
                let last = brace_stack.pop().unwrap();
                if let Some(entry) = brace_stack.last_mut() {
                    entry.0.push(TokenItem {
                        value: Multiple(last.0),
                        type_: last.1 .0,
                        follow: false,
                    })
                } else {
                    token_stack.push(TokenItem {
                        value: Multiple(last.0),
                        type_: last.1 .0,
                        follow: false,
                    })
                }
            } else {
                return Err((ch, *tracker));
            }
        } else if ch.is_whitespace() {
            muxed_ref(&mut brace_stack, &mut token_stack)
                .last_mut()
                .map_or((), type_assign);
        } else {
            let stack = muxed_ref(&mut brace_stack, &mut token_stack);
            let new_operator = || TokenItem {
                value: Single(ch.to_string()),
                type_: token_type(&ch.to_string()),
                follow: false,
            };
            if let Some(item) = stack.last_mut() {
                match &mut item.value {
                    Single(content) => {
                        if let Operator(_) = item.type_ {
                            if item.follow {
                                stack.push(new_operator());
                            } else {
                                content.push(ch);
                                item.type_ = token_type(&content);
                            }
                        } else {
                            stack.push(new_operator());
                        }
                    }
                    Multiple(_) => stack.push(new_operator()),
                }
            } else {
                stack.push(new_operator());
            }
        }
    }
    token_stack.last_mut().map_or((), type_assign);
    Ok(token_stack)
}
