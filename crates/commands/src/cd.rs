use std::ops::Deref;

use return_structure::{ReturnStructure, Output};

#[derive(Debug, Clone, Copy)]
pub struct ChangeDirectory;

impl ChangeDirectory {
    pub fn new() -> Self {Self{}}
    pub fn run<'a>(
        &self,
        arguments: &Vec<String>,
        return_struct: &'a mut ReturnStructure
    ) -> ReturnStructure<'a> {
        if arguments.len() > 1{
            println!("Expected 1 argument found {}", arguments.len());
            return_struct.exit_code = 1;
            return return_struct.clone();
        }
        let _ = match arguments.clone().pop() {
            Some(path) => {
                match path.find('~') {
                    Some(_) => {
                        let mut home = String::new();
                        if let Ok(p) = if cfg!(windows) {
                            std::env::var("USERPROFILE")
                        } else {
                            std::env::var("HOME")
                        } {
                            home = p;
                        }
                        match std::env::set_current_dir(
                            path
                                .replace(
                                    '~',
                                    home.deref()
                                )
                            )
                        {
                            Ok(_) => {
                                return_struct.exit_code = 0
                            },
                            Err(e) => {
                                eprintln!("{}", e);
                                return_struct.exit_code = 1
                            }
                        };
                    }
                    None => {
                        match std::env::set_current_dir(path) {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("{}", e);
                                return_struct.exit_code = 1;
                            }
                        };
                    }
                }
            }
            None => {
                return_struct.exit_code = 0;
                if cfg!(windows) {
                    let mut path = String::new();
                    if let Ok(c) = std::env::var("USERPROFILE") {
                        path = c;
                    }
                    std::env::set_current_dir(path).unwrap();
                }
                else if cfg!(unix) {
                    let mut path = String::new();
                    if let Ok(c) = std::env::var("HOME") {
                        path = c;
                    }
                    std::env::set_current_dir(path).unwrap();
                }
                else {
                    return_struct.exit_code = 1
                }
            }
        };
        return_struct.output = Output::StandardOutput(String::new());
        return return_struct.clone();
    }
}