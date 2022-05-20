use shell_props::{
    ReturnStructure,
    Output
};

#[derive(Debug, Clone, Copy)]
pub struct Set;

impl Set {
    pub fn new () -> Self {Self {  }}
    pub fn run<'a> (
        &'a self,
        arguments: &'a Vec<String>,
        return_struct: &'a mut ReturnStructure
    ) -> ReturnStructure {
        if arguments.len() == 0 {
            return_struct.exit_code = 1;
            return_struct.output = Output::StandardOutput(
                String::from("No arguments found\n")
            );
            return return_struct.to_owned();
        } else if arguments.len() % 2 != 0 {
            return_struct.exit_code = 1;
            return_struct.output = Output::StandardOutput(
                String::from("Unexpected length of arguments\n")
            );
            // println!("{:?}", return_struct.to_owned());
            return return_struct.to_owned();
        } else {
            let mut idx: usize = 0;
            while idx < arguments.len() {
                println!("{}", arguments[idx]);
                return_struct
                    .vars
                    .borrow_mut()
                    .insert(
                        arguments[idx].to_string(),
                        arguments[idx+1].to_string()
                    );
                idx+=2;
            }
            return_struct.to_owned()
        }
    }
}