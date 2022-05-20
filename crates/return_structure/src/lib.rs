use std::collections::HashMap;

#[derive(Clone)]
pub enum Output<'out> {
    StandardOutput(String),
    TabularOutput(HashMap<&'out str, &'out str>)
}

#[derive(Clone)]
pub struct ReturnStructure<'ret> {
    pub exit_code: i32,
    pub vars: HashMap<&'ret str, &'ret str>,
    pub output: Output<'ret>
}

impl <'ret> ReturnStructure<'ret> {
    pub fn from(
        code: i32,
        variable_map: HashMap<&'ret str, &'ret str>,
        cmd_output: Output<'ret>
    ) -> Self { Self {
        exit_code: code,
        vars: variable_map,
        output: cmd_output
    }}
}