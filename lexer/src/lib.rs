use std::iter::Peekable;

use error::IdError;
use error::LexerError;
use state::StringState;
use token::{Token, LiteralKind};

mod state;
pub mod error;
pub mod token;

/// Provides a tracker object to better point where an error has occured,
/// Cannot be changed outside of the `lexer` crate and is for read only
/// purposes outside.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tracker {
    row: usize,
    col: usize,
}

impl Tracker {
    pub(crate) fn add_row(&mut self) {
        self.row += 1;
    }

    pub(crate) fn add_col(&mut self) {
        self.col += 1;
    }

    pub(crate) fn new() -> Self {
        Self { row: 0, col: 0 }
    }
}

/// Lexer class lazily generates token objects to be later used by a parser,
/// It takes a source string and lives as long as the string slice exists
/// in the memory.
///
/// It implements the `Iterator` trait to provide lazy evaluation.
///
/// Example
/// ```
/// use lexer::Lexer;
/// use lexer::token::Keyword;
/// use lexer::token::Token;
///
/// let string  = String::from("let ident");
/// let mut lxr = Lexer::new(Box::new(string.chars()));
///
/// let kw = Token::Keyword(Keyword::Let);
/// let id = Token::Identifier(String::from("ident"));
///
/// assert!(matches!(lxr.next(), Some(Ok(kw))));
/// assert!(matches!(lxr.next(), Some(Ok(id))));
/// assert!(matches!(lxr.next(), None));
/// ```
#[allow(dead_code)]
pub struct Lexer<'c> {
    some_char: Option<char>,
    tracker: Tracker,
    qstate:  StringState,
    source:  Peekable<Box<dyn Iterator<Item = char> + 'c>>,
}

#[allow(dead_code)]
impl<'c> Lexer<'c> {
    /// Generates a "source" as `Peekable<Chars>` from the given input string slice,
    /// and holds it with the lifetime of the string slice.
    ///
    /// The struct will consume the source to generate the token objects.
    pub fn new(stream: Box<dyn Iterator<Item = char> + 'c>) -> Self {
        Self {
            some_char: None,
            tracker: Tracker::new(),
            source: stream.peekable(),
            qstate: StringState::default(),
        }
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, error::LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = String::new();

        if let Some(c) = self.some_char {
            buffer.push(c);
            self.some_char = None;
        }

        while let Some(ch) = self.source.next() {
            if cfg!(windows)
                .then(|| (ch == '\r' && self.source.peek().filter(|c| **c == '\n').is_some()) || ch == '\n')
                .unwrap_or_else(|| ch == '\n')
            {
                #[cfg(windows)]
                if ch == '\r' { self.source.next(); }

                let mut tracker = self.tracker;
                tracker.add_col();

                self.tracker.add_row();
                self.tracker.col = 0;

                if self.qstate.is_squote() {
                    return Some(Err(LexerError::new(IdError::InvalidLiteral(buffer.clone()), tracker)));
                }

                if self.qstate.is_comment() {
                    self.qstate = StringState::default();
                }

                if buffer.is_empty()         { continue; }
                if ! self.qstate.is_dquote() { break; }
            }

            if self.qstate.is_normal() && ch == '#' { self.qstate = StringState::Comment; }

            match self.qstate {
                StringState::Comment => { continue; },
                StringState::Normal  => {
                    self.qstate = if ch == '\''      { StringState::SQuote }
                                  else if ch == '\"' { StringState::DQuote }
                                  else               { self.qstate };

                    if ! self.qstate.is_normal() {
                        if ! buffer.is_empty() { break; }
                        continue;
                    }

                    if ch.is_whitespace() {
                        if buffer.is_empty() {
                            continue;
                        }

                        let result = buffer.as_str().parse().map_err(|e| LexerError::new(e, self.tracker));
                        return Some(result);
                    }

                    buffer.push(ch);
                    let maybe_token = buffer.as_str().parse().map_err(|e| LexerError::new(e, self.tracker));

                    if self.source.peek().is_none() {
                        return Some(maybe_token);
                    }

                    if maybe_token.is_err() {
                        self.some_char = buffer.pop();

                        let result = buffer.as_str().parse().map_err(|e| LexerError::new(e, self.tracker));
                        return Some(result);
                    }
                },
                StringState::SQuote if ch == '\'' => {
                    self.qstate = StringState::default();
                    let result = match get_char_type(&buffer) {
                        Some(kind) => Ok(Token::Literal(kind)),
                        None       => Err(LexerError::new(IdError::InvalidLiteral(buffer.clone()), self.tracker)),
                    };

                    return Some(result);
                }
                StringState::DQuote if ch == '\"' => {
                    self.qstate = StringState::default();
                    return Some(Ok(Token::Literal(LiteralKind::String(buffer.clone()))));
                }
                StringState::DQuote | StringState::SQuote => {
                    buffer.push(ch);
                },
            }

            self.tracker.add_col();
        }

        (! buffer.is_empty()).then(|| {
            let result = buffer.parse();
            result.map_err(|e| LexerError::new(e, self.tracker))
        })
    }
}

fn get_char_type(string: &str) -> Option<LiteralKind> {
    if let Some(string) = string.strip_prefix("\\b") {
        string.parse::<u8>()
            .map(|c| LiteralKind::Char(c as char))
            .ok()
    } else if let Some(string) = string.strip_prefix("\\x") {
        u8::from_str_radix(string, 16)
            .map(|c| LiteralKind::Char(c as char))
            .ok()
    } else {
        (string.len() == 1) .then_some(LiteralKind::Char(string.chars().next().unwrap()))
    }
}
