use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Output {
    StandardOutput(String),
    TabularOutput(HashMap<String, &'static str>)
}

#[derive(Clone, Debug)]
pub struct ReturnStructure {
    pub exit_code: i32,
    pub vars: Rc<RefCell<HashMap<String, String>>>,
    pub output: Output
}

impl ReturnStructure{
    pub fn from(
        code: i32,
        variable_map: Rc<RefCell<HashMap<String, String>>>,
        cmd_output: Output
    ) -> Self { Self {
        exit_code: code,
        vars: variable_map,
        output: cmd_output
    }}
}