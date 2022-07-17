pub mod base {
    use rush_parser::parsers::{
        syntree::analyzer::SyntaxValidationTree,
        lexer::lexer_charwise
    };
    use std::io::Write;

    pub struct Read {
        validation_tree: SyntaxValidationTree
    }

    impl Read{
        pub fn new() -> Self {
            Self { validation_tree:  SyntaxValidationTree::from(vec![
                ("let",    "let !dyn !mut Token !(?# Token) !(:: DataType) = ($ Token)^Expression^Data"),
                ("func",   "func Token !COLLECTION CODEBLOCK"),
                ("if",     "if EXPRESSION CODEBLOCK else @if CODEBLOCK"),
                ("switch", "switch EXPRESSION : (case EXPRESSION : CODEBLOCK @case !(default : CODEBLOCK))")
            ])}
        }

        pub fn show_tree(&self) {
            for entry in &self.validation_tree.entries() {
                self.validation_tree.show_entry(entry).unwrap();
                println!();
            }
        }

        pub fn show_entry(&self, entry: &str) -> Result<(), String> {
            self.validation_tree.show_entry(entry)
        }

        pub fn read_line(&self) -> i32 {
        
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
            
                match lexer_charwise(&self.validation_tree, &command_stream.trim()) {
                    Ok(analysis) => {
                        if analysis.get(0).map_or(false, |token| token.value.get_string() == Ok("exit".to_string())) {
                            break;
                        }
                        else {
                            for tok in &analysis {
                                println!("{}", tok);
                            }
                        }
                    }
                    Err(why) => {
                        eprintln!("{why}")
                    }
                }
            }
            return 0;
        }

        pub fn read_file(&self, filepath: &str) -> i32 {
            if !filepath.is_empty() {
                let fcontent = std::fs::read_to_string(filepath).unwrap();
                match lexer_charwise(&self.validation_tree, &fcontent) {
                    Ok(vec) => {
                        for tok in &vec {
                            println!("{}", tok);
                        }
                    },
                    Err(why) => {
                        eprintln!("{} in file {}", why, filepath);
                        return 1;
                    }
                };
            }
            return 0;
        }
    }
}
