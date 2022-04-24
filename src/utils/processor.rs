#[path = "string_utils/splitters.rs"] mod splitters;
use command_utils;
use std::ops::Deref;
use rustyline::{Editor, error::ReadlineError};
use splitters::{Splitters, Split};

pub struct Process;

impl Process {

pub fn interactive () -> i32 {
    let map = command_utils::init();
    let mut reader = Editor::<()>::new();
    loop {
        let in_string = match reader.readline("> ") {
            Ok(s) => {
                reader.add_history_entry(&s);
                println!("{}", reader.history().len());
                s
            },
            Err(ReadlineError::Eof) => {return 0;},
            Err(ReadlineError::Interrupted) => {return 130;},
            Err(e) => {
                println!("Error: {}", e);
                String::new()
            }
        };
        match Splitters::bracket(in_string.deref(), '(') {
            Split::Split(vector) => {
                println!("{:?}", vector);
            },
            Split::Failed(e) => {
                println!("{}", e);
            }
            _ => {
                println!("{}", in_string);
            }
        };
        match Splitters::dbreaker(in_string.deref(), ' ') {
            Split::Split(vector) => {
                println!("{:?}", vector);
            },
            Split::Failed(e) => {
                println!("{}", e);
            }
            _ => {
                println!("1, {}", in_string);
            }
        };
        match Splitters::quote(in_string.deref(), ' ') {
            Split::Split(vector) => {
                println!("{:?}", vector);
            },
            Split::Failed(e) => {
                println!("{}", e);
            }
            _ => {
                println!("1, {}", in_string);
            }
        };
        // if in_string == "exit" {
        //     break;
        // } else {
            command_utils::run(in_string.deref(), &map);
    }
}
}