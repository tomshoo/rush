use argparse::ArgumentParser;
use rush_base::base;
use std::process;

fn main() -> Result<(), String> {
    let mut fname = String::new();
    {
        let mut parser: ArgumentParser = ArgumentParser::new();
        parser.set_description("Simple shell written in rust");
        parser.refer(&mut fname).add_option(
            &["-f", "--file"],
            argparse::Store,
            "Read from the given file",
        );
        if let Err(e) = parser.parse_args() {
            return Err(format!("{:?}", e));
        }
    }

    //Parse the file
    if !fname.is_empty() {
        process::exit(base::read_file(&fname));
    } else {
        Err("Cannot read from empty filename")?;
    }

    //Parse user input
    process::exit(base::read_line());
}
