#[derive(Clone, Copy)]
#[derive(Debug)]
pub enum StringState {
    SQuote,
    DQuote,
    Comment,
    Normal,
}

#[allow(dead_code)]
impl StringState {
    /// Returns `true` if the string state is [`SQuote`].
    ///
    /// [`SQuote`]: StringState::SQuote
    #[must_use]
    pub fn is_squote(&self) -> bool {
        matches!(self, Self::SQuote)
    }

    /// Returns `true` if the string state is [`DQuote`].
    ///
    /// [`DQuote`]: StringState::DQuote
    #[must_use]
    pub fn is_dquote(&self) -> bool {
        matches!(self, Self::DQuote)
    }

    /// Returns `true` if the string state is [`Comment`].
    ///
    /// [`Comment`]: StringState::Comment
    #[must_use]
    pub fn is_comment(&self) -> bool {
        matches!(self, Self::Comment)
    }

    /// Returns `true` if the string state is [`Normal`].
    ///
    /// [`Normal`]: StringState::Normal
    #[must_use]
    pub fn is_normal(&self) -> bool {
        matches!(self, Self::Normal)
    }
}

impl Default for StringState {
    #[inline]
    fn default() -> Self {
        Self::Normal
    }
}
