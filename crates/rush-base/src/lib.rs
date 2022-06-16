pub mod engine {
    use rush_utils::lexer;
    use std::io::Write;
    pub fn main() -> i32 {
        loop {
            let mut command_stream = String::new();
            print!("prompt> ");
            std::io::stdout().flush().unwrap();
            let bytes = match std::io::stdin().read_line(&mut command_stream) {
                Ok(bytes) => {
                    let reducer = if cfg!(unix) { 1 } else { 2 };
                    if bytes >= reducer {
                        bytes - reducer
                    } else {
                        return 1;
                    }
                }
                Err(err) => panic!("Failure while reading, {}", err),
            };
            if bytes == 0 {
                continue;
            }
            command_stream = command_stream.trim().to_string();
            match lexer::lexer_charwise(&command_stream) {
                Ok(token_stream) => {
                    if token_stream[0].value == "exit" {
                        break;
                    } else {
                        for token in &token_stream {
                            print!("{:?} ", token.token_type);
                        }
                        println!();
                    }
                }
                Err(err) => {
                    println!("{err}")
                }
            }
        }
        return 0;
    }
}
