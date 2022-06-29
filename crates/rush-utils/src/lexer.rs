// Token properties
use super::token::DataType as Type;
use super::token::Token as TokenItem;
use super::token::TokenType::*;
use super::TOKEN_MAP;

// Imports
use std::collections::HashMap;

fn string_type(string: &str) -> TokenItem {
    TokenItem {
        value: string.to_string(),
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
        value: string.to_string(),
        type_: if let Some(property) = TOKEN_MAP.get(string) {
            *property
        } else {
            Operator("SPECIAL")
        },
        follow,
    }
}

#[allow(unused_variables, unused_mut)]
pub fn lexer_charwise<'a>(
    syntax_tree: &rush_parser::analyzer::SyntaxValidationTree,
    stream: &'a str,
) -> Result<Vec<TokenItem>, String> {
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
    for (index, token) in stream.chars().enumerate() {
        // Ignore all comments,
        // comments start with a '##'
        if token == '#' {
            if let Some(ch) = stream.as_bytes().get(index + 1) {
                if ch == &('#' as u8) {
                    break;
                }
            }
        }
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
                token_container.push(TokenItem {
                    value: evaluated_sub_stream.clone(),
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
                braced = false;
            } else {
                token_container.push(TokenItem {
                    value: evaluated_sub_stream.clone(),
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
        return Err(format!(
            "Err: Invalid token_stream at {evaluated_sub_stream}"
        ));
    }
    return Ok(token_container);
}
