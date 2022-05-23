#[path = "string_utils/splitters.rs"] mod splitters;
use std::collections::HashMap;
use call_command::{CallCommand, ShellStatus};
use shell_props::{ReturnStructure, Output, OptionProperties};
use std::ops::Deref;
use std::rc::Rc;
use std::cell::RefCell;
use rustyline::{Editor, error::ReadlineError};
use splitters::{Splitters, Split};

#[allow(dead_code)]
pub struct Process {
    // variables: Rc<RefCell<HashMap<&'shell str, &'shell str>>>k
    variables: Rc<RefCell<HashMap<String, OptionProperties>>>
}

impl Process {
    pub fn new () -> Self {Self {
        variables: Rc::from(RefCell::from(HashMap::from([
            ("TEST_VAR".to_string(), OptionProperties::from(false, "TEST_VALUE".to_string())),
            ("EXIT_CODE".to_string(), OptionProperties::from(false, "0".to_string()))
        ])))
    }}

    pub fn interactive<'x> (&'x self) -> i32 {
        let mut call_command = CallCommand::new();
        let return_struct: ReturnStructure = ReturnStructure::from(
            0,
            Rc::clone(&self.variables),
            Output::StandardOutput(String::new())
        );
        call_command.init(return_struct);
        let mut reader = Editor::<()>::new();
        let mut _exit_code = 0;
        loop {
            let in_string = match reader.readline("> ") {
                Ok(s) => {
                    reader.add_history_entry(&s);
                    s
                },
                Err(ReadlineError::Eof) => {return 0;},
                Err(ReadlineError::Interrupted) => {return 130;},
                Err(e) => {
                    println!("Error: {}", e);
                    String::new()
                }
            };
            match Splitters::dbreaker(in_string.deref(), ' ') {
                Split::Split(vector) => {
                    if !vector.is_empty() {
                        let mut arg_vec = vector.clone();
                        arg_vec.remove(0);
                        let command_done = call_command.run(
                            vector[0].deref(),
                            arg_vec
                        ).unwrap();
                        match command_done {
                            ShellStatus::Maintain(c) => {
                                _exit_code = c.exit_code;
                                // println!("{:?}", c);
                                print!(
                                    "{}", if let Output::StandardOutput(c) = c.output {c}
                                    else {String::from("tabs")}
                                );
                            }
                            ShellStatus::Terminate(c) => {
                                _exit_code = c;
                                break;
                            }
                        }
                    }
                },
                Split::Failed(e) => {
                    println!("{}", e);
                }
                _ => {
                    println!("1, {}", in_string);
                }
            };
        }
        return _exit_code;
    }
}
