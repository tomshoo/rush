use std::rc::Rc;

use shell_props::{
    ReturnStructure,
    Output, Runnable, GetRunnable
};

#[derive(Debug, Clone, Copy)]
pub struct ClearScreen;

impl ClearScreen {
    pub fn new() -> Self { Self{ } }
}

impl GetRunnable for ClearScreen {
    fn create(&self) -> Box<dyn Runnable> {
        return Box::new(Self::new());
    }
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