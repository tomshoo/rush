use std::iter::Peekable;

use error::IdError;
use error::LexerError;
use token::{Kind, LiteralKind, Token, TOKENS};

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
/// use lexer::token::Kind;
/// use lexer::token::Keyword;
/// use lexer::token::Token;
/// let mut lxr = Lexer::new("let ident");
///
/// let kw = Token::new(String::from("let"), Kind::Keyword(Keyword::Let));
/// let id = Token::new(String::from("ident"), Kind::Identifier);
///
/// assert_eq!(lxr.next(), Some(Ok(kw)));
/// assert_eq!(lxr.next(), Some(Ok(id)));
/// assert_eq!(lxr.next(), None);
/// ```
pub struct Lexer<'c> {
    state: Tracker,
    source: Peekable<Box<dyn Iterator<Item = char> + 'c>>,
}

impl<'c> Lexer<'c> {
    /// Generates a "source" as `Peekable<Chars>` from the given input string slice,
    /// and holds it with the lifetime of the string slice.
    ///
    /// The struct will consume the source to generate the token objects.
    pub fn new(stream: impl Iterator<Item = char> + 'c) -> Self {
        Self {
            state: Tracker::new(),
            source: (Box::new(stream) as Box<dyn Iterator<Item = char>>).peekable(),
        }
    }

    pub fn next_lexeme(&mut self) -> Option<(String, u8)> {
        let mut value = String::new();
        let mut quotations = 0_u8;

        while let Some(ch) = self.source.next() {
            // Handle CRLF
            #[cfg(windows)]
            if ch == '\r' && matches!(self.source.peek(), Some(n) if n == '\n') {
                self.source.next();

                self.state.add_row();
                self.state.col = 0;

                if !value.is_empty() {
                    break;
                }

                continue;
            }

            // Handle LF
            #[cfg(not(windows))]
            if ch == '\n' {
                self.state.add_row();
                self.state.col = 0;

                if !value.is_empty() {
                    break;
                }

                continue;
            }

            self.state.add_col();

            if let Some(bit) = match ch {
                '\'' => (quotations & 0b10 != 0b10).then_some(0b01),
                '\"' => (quotations & 0b01 != 0b01).then_some(0b10),
                _chr => None,
            } {
                quotations ^= bit;
                if quotations & bit == bit {
                    continue;
                }

                quotations ^= bit;
                break;
            } else if ch.is_whitespace() && quotations == 0b00 {
                match self.source.peek().map(|c| c.is_whitespace()) {
                    Some(true) | Some(false) if value.is_empty() => continue,
                    _default => break,
                }
            } else {
                value.push(ch);
                if let Some(true) = self.source.peek().map(|ch| {
                    !(ch.is_alphanumeric() || ch == &'_')
                        && (token::is_valid_identifier(&value) || value.parse::<usize>().is_ok())
                        && quotations == 0b00
                }) {
                    break;
                }
            }

            if let Some(true) = TOKENS
                .get(&value)
                .map(|k| (k.is_delimitter() || k.is_operator()) && quotations == 0b00)
            {
                let Some(c) = self.source.peek() else { break; };
                let mut new_lexeme = String::with_capacity(value.len() + 1);
                new_lexeme.clone_from(&value);
                new_lexeme.push(*c);

                if TOKENS.get(&new_lexeme).is_some() {
                    continue;
                }

                break;
            }
        }

        (!value.is_empty()).then_some((value, quotations))
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, error::LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        let (lexeme, quotations) = self.next_lexeme()?;

        if quotations & 0b01 == 0b01 {
            match get_char_type(&lexeme) {
                Some(kind) => Some(Ok(Token::new(lexeme, Kind::Literal(kind)))),
                None => Some(Err(LexerError::new(
                    IdError::InvalidLiteral(lexeme),
                    self.state,
                ))),
            }
        } else if quotations & 0b10 == 0b10 {
            Some(Ok(Token::new(lexeme, Kind::Literal(LiteralKind::String))))
        } else {
            match Token::try_from(lexeme) {
                Err(err) => return Some(Err(LexerError::new(err, self.state))),
                Ok(tokn) => Some(Ok(tokn)),
            }
        }
    }
}

fn get_char_type(string: &str) -> Option<LiteralKind> {
    if let Some(string) = string.strip_prefix("\\b") {
        string
            .parse::<u8>()
            .is_ok()
            .then_some(LiteralKind::ByteChar)
    } else if let Some(string) = string.strip_prefix("\\x") {
        u8::from_str_radix(string, 16)
            .is_ok()
            .then_some(LiteralKind::HexChar)
    } else {
        (string.len() == 1).then_some(LiteralKind::Char)
    }
}
