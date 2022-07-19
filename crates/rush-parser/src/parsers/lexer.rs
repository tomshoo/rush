// Syntax tree node
use super::syntree::TreeNode;

// Token properties
use super::syntree::syntax_tree::SyntaxValidationTree;
use crate::token::DataType as Type;
use crate::token::Token as TokenItem;
use crate::token::TokenItemType::*;
use crate::token::TokenType::*;
use crate::TOKEN_MAP;

type TokenStream = Vec<TokenItem>;

fn string_type(string: &str, follow: bool) -> TokenItem {
    TokenItem {
        value: Single(string.to_string()),
        follow,
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
    row: usize,
    col: usize,
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
) -> Result<TokenStream, String> {
    let mut tracker = Tracker { row: 1, col: 0 };
    let result = lexer(syntax_tree, stream, &mut tracker, 0, '\0');
    match result {
        Ok(o) => {
            println!("{:?}", tracker);
            Ok(o.0)
        }
        Err(e) => Err(format!("Expected {} at {:?}", &e.0, &e.1)),
    }
}

fn flush(
    stream: &mut String,
    container: &mut TokenStream,
    follow: bool,
    token_creator: impl Fn(&str, bool) -> TokenItem,
) -> bool {
    if stream.is_empty() {
        return false;
    } else {
        container.push(token_creator(stream, follow));
        stream.clear();
        return true;
    }
}

#[allow(unused_mut, unused_variables)]
fn lexer<'a>(
    syntax_tree: &SyntaxValidationTree,
    stream: &'a str,
    tracker: &mut Tracker,
    stack_counter: u8,
    brace: char,
) -> Result<(TokenStream, usize), (char, Tracker)> {
    let stack_counter = stack_counter + 1;
    let brace_map = |ch| match ch {
        '(' => Some(Evaluatable("EXPRESSION")),
        '{' => Some(Evaluatable("CODEBLOCK")),
        '[' => Some(DataType(Type::Collection)),
        _ => None,
    };
    let mut node_stack: Vec<TreeNode> = Vec::new();
    let mut token_container = Vec::new();
    let mut block_stream = String::new();
    let mut main_stream = String::new();
    let mut operator_stream = String::new();
    let mut comment = false;
    let mut single_quote = false;
    let mut double_quote = false;
    let mut idx = 0;
    while let Some(ch) = stream.chars().nth(idx) {
        let follow = stream
            .chars()
            .nth(idx + 1)
            .map_or(false, |ch: char| ch == ' ' || ch == '\t');
        if ch == '\n' {
            comment = false;
            tracker.advance_row();
            tracker.col = 0;
            idx += 1;
            continue;
        } else {
            tracker.advance_col();
        }

        if ch == '#'
            && stream
                .chars()
                .nth(idx + 1)
                .map_or(false, |ch: char| ch == '#')
        {
            comment = true;
        }

        if comment {
            idx += 1;
            continue;
        }

        if ch == '\'' && !double_quote {
            single_quote = !single_quote;
        } else if ch == '"' && !single_quote {
            double_quote = !double_quote;
        }

        if single_quote || double_quote {
            flush(&mut main_stream, &mut token_container, follow, string_type);
            if (single_quote && ch == '\'') || (double_quote && ch == '"') {
                idx += 1;
                continue;
            }
            block_stream.push(ch);
        } else {
            if !block_stream.is_empty() {
                token_container.push(TokenItem {
                    value: Single(block_stream.clone()),
                    type_: DataType(Type::String),
                    follow: false,
                });
                block_stream.clear();
            }
            if let Some(token_type) = brace_map(ch) {
                flush(&mut main_stream, &mut token_container, follow, string_type);
                let result = lexer(syntax_tree, &stream[idx + 1..], tracker, stack_counter, ch)?;
                idx += result.1 + 1;
                token_container.push(TokenItem {
                    value: Multiple(result.0),
                    type_: token_type,
                    follow: false,
                });
            } else {
                if ch.is_ascii_alphanumeric() {
                    main_stream.push(ch);
                } else {
                    if ch == ' ' || ch == '\t' {
                        flush(&mut main_stream, &mut token_container, follow, string_type);
                        if !operator_stream.is_empty() {
                            flush(
                                &mut operator_stream,
                                &mut token_container,
                                follow,
                                token_type,
                            );
                        }
                    } else if ch == ')' || ch == '}' || ch == ']' {
                        flush(&mut main_stream, &mut token_container, follow, string_type);
                        return Ok((token_container, idx));
                    } else if ch == '_' {
                        main_stream.push(ch);
                    } else {
                        flush(&mut main_stream, &mut token_container, follow, string_type);
                        operator_stream.push(ch);
                    }
                }
            }
        }
        idx += 1;
    }
    if stack_counter > 1 {
        Err((
            match brace {
                '(' => ')',
                '[' => ']',
                '{' => '}',
                _ => {
                    println!("{}", stream);
                    brace
                }
            },
            *tracker,
        ))
    } else {
        flush(&mut main_stream, &mut token_container, false, string_type);
        Ok((token_container, idx - 1))
    }
}
