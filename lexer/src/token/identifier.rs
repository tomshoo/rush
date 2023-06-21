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

fn is_valid_identifier(string: &str) -> bool {
    let mut chars = string.chars();

    if chars.next().is_some_and(|s| !s.is_alphabetic()) {
        return false;
    };

    for ch in chars {
        if !ch.is_alphanumeric() {
            return false;
        }
    }

    true
}
