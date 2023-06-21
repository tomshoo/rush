use std::fmt::Display;

/// Provides a tracker object to better point where an error has occured,
/// Cannot be changed outside of the `lexer` crate and is for read only
/// purposes outside.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Tracker {
    row: usize,
    col: usize,
}

impl Tracker {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn add_row(&mut self) {
        self.row += 1;
    }

    pub fn add_col(&mut self) {
        self.col += 1;
    }

    pub fn set_col(&mut self, col: usize) {
        self.col = col;
    }

    pub fn set_row(&mut self, row: usize) {
        self.row = row;
    }

    pub fn get_row(&self) -> usize {
        self.row
    }

    pub fn get_col(&self) -> usize {
        self.col
    }
}

impl Default for Tracker {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Tracker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Line: {}, Column: {}", self.row, self.col)
    }
}
