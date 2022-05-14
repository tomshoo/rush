use std::ops::Deref;

use return_structure::{ReturnStructure, Output};

#[derive(Debug, Clone, Copy)]
pub struct GetChildren;

impl GetChildren {
    pub fn new() -> Self {Self{}}
    pub fn run(&self, _: &Vec<String>, return_struct: &mut ReturnStructure) -> ReturnStructure {
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
                            if let Some(c) = property.unwrap().file_name().to_str() {
                                out_string+=format!("{}\n", c).deref();
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