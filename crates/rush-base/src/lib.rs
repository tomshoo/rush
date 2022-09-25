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

            match lexer_charwise(command_stream.trim()) {
                Ok(analysis) => {
                    if analysis.get(0).map_or(false, |token| {
                        token
                            .value
                            .as_single()
                            .map_or(false, |stream| stream == "exit")
                    }) {
                        return 0;
                    }
                    analysis.iter().for_each(|tok| {
                        println!("{}", tok);
                    });
                }
                Err(why) => {
                    eprintln!("{why:?}")
                }
            }
        }
    }

    pub fn read_file(filepath: &str) -> i32 {
        std::fs::read_to_string(filepath).map_or(1, |fcontent| {
            lexer_charwise(&fcontent).map_or_else(
                |err| {
                    eprintln!("{:?} in file {}", err, filepath);
                    1
                },
                |vec| {
                    vec.iter().for_each(|tok| println!("{}", tok));
                    0
                },
            )
        })
    }
}
