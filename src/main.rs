extern crate argparse;
extern crate rush_base;

use argparse::ArgumentParser;
use rush_base::base;
use std::process;

fn main() {
    let mut fname = String::new();
    let mut show_tree = false;
    {
        let mut parser = ArgumentParser::new();
        parser.set_description("Simple shell written in rust");
        parser.refer(&mut fname).add_option(
            &["-f", "--file"],
            argparse::Store,
            "Read from the given file",
        );
        parser.refer(&mut show_tree).add_option(
            &["-s", "--show-tree"],
            argparse::StoreTrue,
            "Show the syntax tree",
        );
        if let Err(e) = parser.parse_args() {
            eprintln!("{:?}", e);
            process::exit(1);
        }
    }
    // Initialize the reader
    let reader = base::Read::new();

    // Show the syntax validation tree
    if show_tree {
        reader.show_tree();
        process::exit(0)
    }

    //Parse the file
    if !fname.is_empty() {
        process::exit(reader.read_file(&fname));
    }

    //Parse user input
    process::exit(reader.read_line());
}
