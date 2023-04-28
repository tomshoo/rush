use char_reader::ReadChars;
use lexer::Lexer;
use std::{
    fs::File,
    io::{Cursor, Write},
};

fn read_file(path: &str) -> std::io::Result<String> {
    let mut buffer = String::new();
    let reader = ReadChars::from(File::open(path)?);

    for x in reader {
        buffer.push(x?);
    }

    let reader = ReadChars::from(File::open(path)?);
    let lexer = Lexer::new(reader.map(|r| r.unwrap()));
    for token in lexer {
        println!("{:?}", token);
    }

    Ok(buffer)
}

fn read_prompt() -> std::io::Result<()> {
    let mut line_counter = 0_usize;
    let mut buf = String::new();

    println!();

    loop {
        buf.clear();
        line_counter += 1;

        print!("rush:[{:0>3}]> ", line_counter);
        std::io::stdout().flush()?;

        if std::io::stdin().read_line(&mut buf)? == 0 {
            println!("^D");
            break;
        }

        if buf.trim() == "exit" {
            break;
        }

        for ch in lexer::Lexer::new(ReadChars::from(Cursor::new(buf.clone())).map(|r| r.unwrap())) {
            println!("{:?}", ch);
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    if std::env::args().len() > 2 {
        anyhow::bail!("usage: {} [fpath?]", std::env::args().next().unwrap());
    }

    match std::env::args().nth(1) {
        Some(path) => println!("{}", read_file(&path)?),
        None => read_prompt()?,
    }

    Ok(())
}
