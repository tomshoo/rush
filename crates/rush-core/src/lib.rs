pub mod errors;

#[derive(Debug, Clone, Copy)]
pub struct Tracker {
    row: usize,
    col: usize,
}

impl Tracker {
    pub fn new() -> Self {
        Self { row: 0, col: 0 }
    }

    pub fn update_row(&mut self) {
        self.row += 1;
    }
    pub fn update_col(&mut self) {
        self.col += 1;
    }

    pub fn reset_row(&mut self) {
        self.row = 0;
    }
    pub fn reset_col(&mut self) {
        self.col = 0;
    }
}
