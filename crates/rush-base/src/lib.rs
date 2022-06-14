pub mod engine {
    use rush_utils::lexer;
    use std::io::Write;
    pub fn main() -> i32 {
        loop {
            let mut command_stream = String::new();
            print!("prompt> ");
            std::io::stdout().flush().unwrap();
            let bytes = match std::io::stdin().read_line(&mut command_stream) {
                Ok(bytes) => bytes - 2,
                Err(err) => panic!("Failure while reading, {}", err),
            };
            if bytes == 0 {
                continue;
            }
            command_stream = command_stream.trim().to_string();
            if let Ok(token_stream) = lexer::lexer_charwise(&command_stream) {
                if token_stream[0].value == "exit" {
                    break;
                } else {
                    for token in &token_stream {
                        println!("{}", token);
                    }
                }
            }
        }
        return 0;
    }
}
