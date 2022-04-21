use rustyline::error::ReadlineError;

fn main() {
    loop {
        let mut reader = rustyline::Editor::<()>::new();
        let in_string = match reader.readline("prompt> ") {
            Ok(s) => s,
            Err(e) => {
                match e {
                    ReadlineError::Eof => {
                        break;
                    }
                    _ => {
                        eprintln!("Err: {}", e);
                        String::new()
                    }
                }
            }
        };
        if in_string == "exit" {
            break;
        }
    }
}