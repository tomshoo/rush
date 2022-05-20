use std::{ops::Deref, collections::HashMap};
use arg_parser::{argparser, StoreAction, Type};
use std::cell::RefCell;
use std::rc::Rc;

use return_structure::{ReturnStructure, Output};

#[derive(Debug, Clone, Copy)]
pub struct GetChildren;

impl GetChildren {
    pub fn new() -> Self {Self{}}
    pub fn run<'a>(
        &self,
        arguments: &Vec<String>,
        return_struct: &'a mut ReturnStructure
    ) -> ReturnStructure<'a> {
        let show_all = Rc::from(RefCell::from(false));
        {
            let mut parser = argparser::ArgumentParser::new();
            parser.add_argument(
                ["-a", "--all"].to_vec(),
                "show all elements in the directory",
                StoreAction::StoreBool
            ).borrow_mut()
             .refer(Type::Boolean(Rc::clone(&show_all)));

            match parser.parse_args(arguments) {
                Err(e) => {
                    *return_struct = ReturnStructure::from (
                        1,
                        HashMap::new(),
                        Output::StandardOutput(format!("{}\n", e))
                    );
                    return return_struct.clone();
                }
                _ => {}
            };
        }
        let mut current_path = "";
        let mut out_string = String::new();
        match std::env::current_dir() {
            Ok(p) => {
                if let Some(c) = p.to_str() {
                    current_path = c;
                }
                match std::fs::read_dir(current_path) {
                    Ok(rd) => {
                        for property in rd {
                            if let Some(c) = property
                                .unwrap()
                                .file_name()
                                .to_str()
                            {
                                if let Some(0) = &c.find(".") {
                                    if *show_all.borrow() {
                                        out_string+=format!("{}\n", c).deref();
                                    }
                                } else {
                                    out_string+=format!("{}\n", c).deref();
                                }
                            }
                        }
                        return_struct.output = Output::StandardOutput(out_string);
                        return_struct.exit_code = 0;
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        return_struct.exit_code = 1;
                    }
                };
            }
            Err(e) => {
                eprintln!("{}", e);
                return_struct.exit_code = 1;
            }
        }
        return_struct.clone()
    }
}