use char_reader::ReadChars;
use lexer::Lexer;
use std::{
    fs::File,
    io::{Cursor, IsTerminal, Read, Write},
};

fn read_file(path: &str) -> std::io::Result<String> {
    let mut buffer = String::new();
    File::open(path)?.read_to_string(&mut buffer)?;

    let reader = ReadChars::from(Cursor::new(buffer.as_str()));

    Lexer::new(Box::new(reader.filter_map(|r| dbg!(r).ok()))).for_each(|x| match x {
        Ok(token) => println!("{token}"),
        Err(e) => eprintln!("{e}"),
    });

    Ok(buffer)
}

fn read_prompt() -> std::io::Result<()> {
    let mut line_counter = 0usize;
    let mut buf = String::new();

    println!();

    loop {
        buf.clear();
        line_counter += 1;

        print!("rush:[{:0>3}]> ", line_counter);
        std::io::stdout().flush()?;

        if std::io::stdin().read_line(&mut buf)? == 0 || buf.trim() == "exit" {
            eprintln!("\nReached end of file, quitting!");
            break;
        }

        Lexer::new(Box::new(buf.as_str().chars())).for_each(|x| match x {
            Ok(token) => println!("{token}"),
            Err(e) => eprintln!("{e}"),
        });
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    if std::env::args().len() > 2 {
        anyhow::bail!("usage: {} [fpath?]", std::env::args().next().unwrap());
    }

    match std::env::args().nth(1) {
        Some(path) => println!("{}", read_file(&path)?),
        None if std::io::stdin().is_terminal() => read_prompt()?,
        None => {
            let mut string = String::new();
            std::io::stdin().read_to_string(&mut string)?;
            Lexer::new(Box::new(string.chars())).for_each(|t| match t {
                Ok(token) => println!("{token}"),
                Err(e) => eprintln!("{e}"),
            });
        },
    }

    Ok(())
}
