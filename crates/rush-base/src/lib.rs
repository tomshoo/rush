pub mod base {
    use rush_parser::analyzer::SyntaxValidationTree;
    use rush_utils::lexer::lexer_charwise;
    use std::io::Write;

    pub fn main() -> i32 {
        let syntax_tree = SyntaxValidationTree::from(vec![
            ("let",    "let !dyn !mut Token !(:: DataType) = ($ Token)^Expression^Data"),
            ("func",   "func Token !COLLECTION CODEBLOCK"),
            ("if",     "if EXPRESSION CODEBLOCK else @if CODEBLOCK"),
            ("switch", "switch EXPRESSION : (case EXPRESSION : CODEBLOCK @case !(default : CODEBLOCK))"),
        ]);

        syntax_tree.show_entry("let").unwrap();
        syntax_tree.show_entry("func").unwrap();
        syntax_tree.show_entry("if").unwrap();
        syntax_tree.show_entry("switch").unwrap();
        
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
            
            match lexer_charwise(&syntax_tree, &command_stream.trim()) {
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
