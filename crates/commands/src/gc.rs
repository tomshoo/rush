use return_structure::ReturnStructure;

#[derive(Debug, Clone, Copy)]
pub struct GetChildren;

impl GetChildren {
    pub fn new() -> Self {Self{}}
    pub fn run(&self, _: &Vec<String>, return_struct: &mut ReturnStructure) -> ReturnStructure {
        let mut current_path = "";
        match std::env::current_dir() {
            Ok(p) => {
                if let Some(c) = p.to_str() {
                    current_path = c;
                }
                match std::fs::read_dir(current_path) {
                    Ok(rd) => {
                        for property in rd {
                            println!("{}", match property.unwrap().file_name().to_str() {
                                Some(c) => c,
                                None => ""
                            });
                        }
                    },
                    Err(e) => {
                        eprintln!("{}", e);
                        return_struct.exit_code = 1;
                    }
                };
            }
            Err(e) => {
                eprintln!("{}", e);
                return_struct.exit_code = 1;
            }
        }
        return_struct.clone()
    }
}