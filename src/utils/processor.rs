#[path = "string_utils/splitters.rs"] mod splitters;
use command_utils;
use std::ops::Deref;
use rustyline::{Editor, error::ReadlineError};
use splitters::{Splitters, Split};

pub struct Process;

impl Process {
    pub fn interactive () -> i32 {
        let mut call_command = command_utils::CallCommand::new();
        let map = call_command.init();
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
                    let mut arg_vec = vector.clone();
                    arg_vec.remove(0);
                    let command_done = call_command.run(vector[0].deref(), arg_vec, &map).unwrap();
                    _exit_code = command_done.1.exit_code;
                    if command_done.0 == "exit" {
                        break;
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
