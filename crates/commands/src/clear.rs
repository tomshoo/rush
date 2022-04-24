use return_structure::ReturnStructure;
#[derive(Debug, Clone, Copy)]
pub struct ClearScreen;

impl ClearScreen {
    pub fn new() -> Self {Self{}}
    pub fn run<'a>(
        &self, _arguments: &Vec<String>, return_structure: &'a mut ReturnStructure
    ) -> ReturnStructure{
        print!("{} {}", 27 as char, "2[j");
        print!("\033c");
        print!("\x1bc");
        *return_structure = ReturnStructure{
            exit_code: 0
        };
        return_structure.clone()
    }
}