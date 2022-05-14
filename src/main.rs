#[path = "utils/processor.rs"] mod processor;
use argparse::{ArgumentParser, StoreTrue};
fn main() {
    let mut interactive = false;
    {
        let mut parser = ArgumentParser::new();
        parser.refer(&mut interactive)
            .add_option(
                &["-i", "--interactive"], 
                StoreTrue, 
                "Launch an interactive session"
            );
        parser.parse_args()
            .expect("Failed to parse arguments");
    }
    if interactive {
        std::process::exit(processor::Process::interactive());
    }
    else {
        println!("Working on non interactive sessions");
        std::process::exit(0);
    }
}
