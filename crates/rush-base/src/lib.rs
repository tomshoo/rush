pub mod base {
    use rush_parser::parsers::lexer::lexer_charwise;
    use std::io::Write;

    pub fn read_line() -> i32 {
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

            match lexer_charwise(&command_stream.trim()) {
                Ok(analysis) => {
                    if analysis.get(0).map_or(false, |token| {
                        token
                            .value
                            .get_single()
                            .map_or(false, |stream| stream == "exit")
                    }) {
                        break;
                    } else {
                        for tok in &analysis {
                            println!("{}", tok);
                        }
                    }
                }
                Err(why) => {
                    eprintln!("{why:?}")
                }
            }
        }
        return 0;
    }

    pub fn read_file(filepath: &str) -> i32 {
        if !filepath.is_empty() {
            let fcontent = std::fs::read_to_string(filepath).unwrap();
            match lexer_charwise(&fcontent) {
                Ok(vec) => {
                    for tok in &vec {
                        println!("{}", tok);
                    }
                }
                Err(why) => {
                    eprintln!("{:?} in file {}", why, filepath);
                    return 1;
                }
            };
        }
        return 0;
    }
}
