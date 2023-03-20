use lexer::Lexer;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

fn read_file(path: &str) -> std::io::Result<String> {
    let mut buffer = String::new();
    let file_reader = BufReader::new(File::open(path)?);

    file_reader.lines().try_for_each(|line| match line {
        Err(e) => Err(e),
        Ok(line) => {
            let lxr = Lexer::new(&line);
            lxr.for_each(|t| println!("{:?}", t.unwrap()));
            buffer.push_str(&(line + "\n"));
            Ok(())
        }
    })?;

    Ok(buffer)
}

fn read_prompt() -> std::io::Result<()> {
    println!();

    loop {
        print!("rush> ");
        std::io::stdout().flush()?;

        let mut buf = String::new();
        let bytes = std::io::stdin().read_line(&mut buf)?;

        if buf.trim() == "exit" {
            break;
        }

        let lxr = Lexer::new(&buf);

        lxr.for_each(|c| println!("{:3}: {:?}", bytes, c));
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
