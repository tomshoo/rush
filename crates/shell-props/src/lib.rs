use std::collections::HashMap;
use std::cell::RefCell;
use std::fmt::Debug;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Output {
    StandardOutput(String),
    TabularOutput(HashMap<String, &'static str>)
}

#[derive(Debug)]
pub struct OptionProperties {
    pub editable: bool,
    pub mutable: bool,
    pub value: String
}

impl OptionProperties {
    pub fn from (
        is_editable: bool,
        option_value: String
    ) -> Self { Self {
        editable: is_editable,
        mutable: false,
        value: option_value }
    }

    pub fn set (&mut self, option_value: &str) -> Result<(), &str>{
        if self.mutable {
            Ok(self.value = String::from(option_value))
        }
        else {
            Err("variable is not mutable, please declare it as mutable")
        }
    }
}

#[derive(Clone, Debug)]
pub struct ReturnStructure {
    pub exit_code: i32,
    pub vars: Rc<RefCell<HashMap<String, OptionProperties>>>,
    pub output: Output
}

impl ReturnStructure{
    pub fn from(
        code: i32,
        variable_map: Rc<RefCell<HashMap<String, OptionProperties>>>,
        cmd_output: Output
    ) -> Self { Self {
        exit_code: code,
        vars: variable_map,
        output: cmd_output
    }}
}

pub trait GetRunnable {
    fn create() -> Box<dyn Runnable>;
}

pub trait Runnable: Debug {
    fn run (&self, arguments: &Vec<String>, return_stuct: &mut ReturnStructure) -> ReturnStructure;
}