use return_structure::ReturnStructure;

#[derive(Debug, Clone, Copy)]
pub struct Exit;

impl Exit {
    pub fn new() -> Self {Self{}}
    pub fn run<'a>(&self, _arguments: &Vec<String>, return_struct: &'a mut ReturnStructure) -> () {
        std::process::exit(return_struct.exit_code);
    }
}