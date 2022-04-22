use std::ops::Deref;

use rustyline::{Editor, error::ReadlineError};
#[path = "utils/string_utils/splitters.rs"] mod splitters;
use splitters::Splitters;

fn main() {
    for _argument in std::env::args().skip(1) {}

    let mut reader = Editor::<()>::new();
    loop {
        let in_string = match reader.readline("> ") {
            Ok(s) => s,
            Err(ReadlineError::Eof) => {break;}
            Err(e) => {
                println!("Error: {}", e);
                String::new()
            }
        };
        match Splitters::bracket(in_string.deref(), '(') {
            Some(vector) => {
                println!("{:?}", vector);
            }
            _ => {
                println!("{}", in_string);
            }
        };
        match Splitters::dbreaker(in_string.deref(), ' ') {
            Some(vector) => {
                println!("{:?}", vector);
            }
            _ => {
                println!("1, {}", in_string);
            }
        };
        match Splitters::quote(in_string.deref(), ' ') {
            Some(vector) => {
                println!("{:?}", vector);
            }
            _ => {
                println!("1, {}", in_string);
            }
        };
        if in_string == "exit" {
            break;
        }
    }
}