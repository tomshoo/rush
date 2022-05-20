use std::collections::HashMap;

use return_structure::{
    ReturnStructure,
    Output
};

#[derive(Debug, Clone, Copy)]
pub struct ClearScreen;

impl ClearScreen {
    pub fn new() -> Self { Self{ } }
    pub fn run<'a>(
        &self, _arguments: &Vec<String>,
        return_structure: &'a mut ReturnStructure
    ) -> ReturnStructure<'a> {
        *return_structure = ReturnStructure::from (
            0,
            HashMap::new(),
            Output::StandardOutput(format!(
                "{} {} {} {}",
                27 as char,
                "2[j",
                "\033c",
                "\x1bc"
            ))
        );
        return_structure.exit_code = 0;
        return_structure.clone()
    }
}