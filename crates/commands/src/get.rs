use arg_parser::argparser::ArgumentParser;
use arg_parser::Type;
use shell_props::{ReturnStructure, Output};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone, Copy)]
pub struct Get;

impl Get {
    pub fn new () -> Self { Self { } }
    pub fn run(
        &self,
        arguments: &Vec<String>,
        return_struct: &mut ReturnStructure
    ) -> ReturnStructure {
        let variable_name = Rc::from(RefCell::from(String::new()));
        let mut parser = ArgumentParser::new();
        parser.add_argument(
            ["-v", "--variable"].to_vec(),
            "get the variable value from name",
            arg_parser::StoreAction::StoreValue
        ).borrow_mut().refer(Type::String(Rc::clone(&variable_name)));
        parser.parse_args(arguments).unwrap();
        if let Some(value) = return_struct.vars.borrow().get(&(*variable_name.borrow())) {
            return_struct.output = Output::StandardOutput(format!("{}\n", value.value));
            return_struct.exit_code = 0;
        }
        else {
            return_struct.output = Output::StandardOutput(String::new());
            return_struct.exit_code = 1;
        }
        return_struct.to_owned()
    }
}
