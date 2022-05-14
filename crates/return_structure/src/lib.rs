use std::collections::HashMap;

#[derive(Clone)]
pub enum Output {
    StandardOutput(String),
    TabularOutput(HashMap<&'static str, &'static str>)
}

#[derive(Clone)]
pub struct ReturnStructure {
    pub exit_code: i32,
    pub output: Output
}
