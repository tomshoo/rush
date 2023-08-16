use crate::error::IdError;
use std::{fmt::Display, rc::Rc, str::FromStr};

#[derive(Debug, Clone)]
pub struct Identifier {
    name: Rc<str>,
}

impl FromStr for Identifier {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        is_valid_identifier(s)
            .then_some(Self { name: s.into() })
            .ok_or_else(|| IdError::InvalidLiteral(s.into()))
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Identifier({})", self.name)
    }
}

impl Identifier {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[inline(always)]
fn is_valid_identifier(string: &str) -> bool {
    string.chars().enumerate().all(|(i, ch)| {
        ch == '_' || ((i == 0 && ch.is_ascii_alphabetic()) || ch.is_ascii_alphanumeric())
    })
}
