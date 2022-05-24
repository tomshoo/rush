use std::rc::Rc;
use trait_macros::GetRunnable;

use shell_props::{
    ReturnStructure,
    Output, Runnable
};

#[derive(Debug, Clone, Copy, GetRunnable)]
pub struct ClearScreen;

impl ClearScreen {
    pub fn new() -> Self { Self{ } }
}

impl Runnable for ClearScreen {
    fn run<'a>(
        &self, _arguments: &Vec<String>,
        return_structure: &'a mut ReturnStructure
    ) -> ReturnStructure {
        *return_structure = ReturnStructure::from (
            0,
            Rc::clone(&return_structure.vars),
            Output::StandardOutput(format!(
                "{} {} {} {}",
                27 as char,
                "2[j",
                "\033c",
                "\x1bc"
            ))
        );
        return_structure.exit_code = 0;
        return_structure.to_owned()
    }
}