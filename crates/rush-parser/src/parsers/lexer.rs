// Token properties
use crate::token::DataType as Type;
use crate::token::Token as TokenItem;
use crate::token::TokenType::*;
use crate::TOKEN_MAP;
use crate::token::TokenItemType::*;
use super::syntree::analyzer::SyntaxValidationTree;

// Imports
use std::collections::HashMap;

fn string_type(string: &str) -> TokenItem {
    TokenItem {
        value: Single(string.to_string()),
        follow: false,
        type_: if let Ok(_) = string.parse::<isize>() {
            DataType(Type::Number)
        } else if let Ok(_) = string.parse::<f64>() {
            DataType(Type::Float)
        } else if let Some(property) = TOKEN_MAP.get(string) {
            *property
        } else {
            Token
        },
    }
}

fn token_type(string: &str, follow: bool) -> TokenItem {
    TokenItem {
        value: Single(string.to_string()),
        type_: if let Some(property) = TOKEN_MAP.get(string) {
            *property
        } else {
            Operator("SPECIAL")
        },
        follow,
    }
}

#[derive(Debug, Clone, Copy)]
struct Tracker {
    row: u16,
    col: u16
}

impl Tracker {
    pub(crate) fn advance_col(&mut self) {
        self.col+=1;
    }

    pub(crate) fn advance_row(&mut self) {
        self.row+=1;
    }
}

pub fn lexer_charwise<'a>(
    syntax_tree: &SyntaxValidationTree,
    stream: &'a str
) -> Result<Vec<TokenItem>, String> {
    let mut tracker = Tracker {row: 1, col: 0};
    let mut stack_counter = 0;
    let result = lexer(syntax_tree, stream, &mut tracker, &mut stack_counter);
    match result {
        Ok(_) => Ok(result.unwrap()),
        Err(e) => {
            Err(format!("Expected {} at {:?}", &e.0, &e.1))
        }
    }
}

fn lexer<'a>(
    syntax_tree: &SyntaxValidationTree,
    stream: &'a str,
    tracker: &mut Tracker,
    stack_counter: &mut u8
) -> Result<Vec<TokenItem>, (char,Tracker)> {
    *stack_counter+=1;
    let brace_types = HashMap::<char, char>::from([('(', ')'), ('{', '}'), ('[', ']')]);
    let mut evaluated_stream = String::new();
    let mut evaluated_sub_stream = String::new();
    let mut evaluated_subtoken_stream = String::new();
    let mut token_container = Vec::new();
    let mut brace = false;
    let mut brace_close: char = '\0';
    let mut brace_nest = 0;
    let mut single_quote = false;
    let mut double_quote = false;
    let mut braced = false;
    let mut comment = false;
    for (index, token) in stream.chars().enumerate() {
        if token == '\n' {
            if comment {comment = false;}
            tracker.advance_row();
            tracker.col = 0;
            continue;
        }
        else {
            tracker.advance_col();
        }
        // Ignore all comments,
        // comments start with a '##'
        if token == '#' {
            if let Some(ch) = stream.as_bytes().get(index + 1) {
                if ch == &('#' as u8) {
                    comment = true;
                    continue;
                }
            }
        }
        if comment {continue;}
        let previous_state = single_quote || double_quote || brace;

        if token == '\'' && !(double_quote || brace) {
            single_quote = !single_quote;
        } else if token == '"' && !(single_quote || brace) {
            double_quote = !double_quote;
        }

        if !(single_quote || double_quote) {
            if let Some(close) = brace_types.get(&token) {
                if brace {
                    if close == &brace_close {
                        brace_nest += 1;
                    }
                } else {
                    brace = true;
                    brace_close = *close;
                    continue;
                }
            } else if token == brace_close {
                if brace_nest > 0 {
                    brace_nest -= 1;
                } else if previous_state == (brace || single_quote || double_quote) {
                    brace = false;
                    braced = true;
                }
            }
        }

        if brace || single_quote || double_quote {
            if !((token == '\'' && single_quote) || (token == '"' && double_quote)) {
                evaluated_sub_stream.push(token);
            }
        } else {
            if token.is_ascii_alphanumeric() || token == '_' {
                if !evaluated_subtoken_stream.is_empty() {
                    token_container.push(token_type(&evaluated_subtoken_stream, true));
                    evaluated_subtoken_stream.clear();
                }
                evaluated_stream.push(token);
            } else if token != ' '
                && token != '\t'
                && token != '\''
                && token != '"'
                && token != ')'
                && token != '}'
                && token != ']'
                && token != '['
                && token != '{'
                && token != '('
            {
                if !evaluated_stream.is_empty() {
                    token_container.push(string_type(&evaluated_stream));
                    evaluated_stream.clear();
                }
                evaluated_subtoken_stream.push(token);
            } else {
                if !evaluated_subtoken_stream.is_empty() {
                    token_container.push(token_type(&evaluated_subtoken_stream, false));
                    evaluated_subtoken_stream.clear();
                }
                if !evaluated_stream.is_empty() {
                    token_container.push(string_type(&evaluated_stream));
                }
                evaluated_stream.clear();
            }
        }

        if !(evaluated_sub_stream.is_empty() || (brace || single_quote || double_quote)) {
            if braced {
                let mut recursion_tracker = Tracker{row: tracker.row, col: 0};
                match lexer(syntax_tree, &evaluated_sub_stream, &mut recursion_tracker, stack_counter) {
                    Ok(result) => {
                        token_container.push(TokenItem {
                            value: Multiple(result),
                            type_: if brace_close == ']' {
                                DataType(Type::Collection)
                            } else {
                                Evaluatable(if brace_close == ')' {
                                    "EXPRESSION"
                                } else {
                                    "CODE_BLOCK"
                                })
                            },
                            follow: false,
                        });
                    },
                    Err(e) => {
                        return Err((e.0, *tracker));
                    }
                };
                tracker.col-=if tracker.col > recursion_tracker.col {recursion_tracker.col} else {0};
                braced = false;
            } else {
                token_container.push(TokenItem {
                    value: Single(evaluated_sub_stream.clone()),
                    type_: DataType(Type::String),
                    follow: false,
                });
            }
            evaluated_sub_stream.clear();
        }
        if brace || single_quote || double_quote {
            if !evaluated_subtoken_stream.is_empty() {
                token_container.push(token_type(&evaluated_subtoken_stream, false));
                evaluated_subtoken_stream.clear();
            }
            if !evaluated_stream.is_empty() {
                token_container.push(string_type(&evaluated_stream));
                evaluated_stream.clear();
            }
        }
    }

    if !evaluated_stream.is_empty() {
        token_container.push(string_type(&evaluated_stream));
        evaluated_stream.clear();
    }
    if !evaluated_subtoken_stream.is_empty() {
        token_container.push(token_type(&evaluated_subtoken_stream, false));
        evaluated_subtoken_stream.clear();
    }

    if !evaluated_sub_stream.is_empty() || brace || single_quote || double_quote {
        return Err((if brace {
                brace_close
            } else if single_quote {
                '\''
            } else {
                '\"'
            },
        *tracker));
    }
    return Ok(token_container);
}
