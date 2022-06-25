pub mod base {
    use rush_utils::lexer::lexer_charwise;
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
                        return 0;
                    }
                }
                Err(err) => panic!("Failure while reading, {}", err),
            };
            if bytes == 0 {
                continue;
            }
            command_stream = command_stream.trim().to_string();
            match lexer_charwise(&command_stream) {
                Ok(analysis) => {
                    if analysis.get(0).is_some() && analysis.get(0).unwrap().value == "exit" {
                        break;
                    } else {
                        for token in analysis {
                            println!("{token}");
                        }
                    }
                }
                Err(why) => {
                    panic!("{why}")
                }
            }
        }
        return 0;
    }
}
