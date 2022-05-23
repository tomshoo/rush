use shell_props::{
    ReturnStructure,
    Output,
    OptionProperties, Runnable, GetRunnable
};

#[derive(Debug, Clone, Copy)]
pub struct Set;

impl Set {
    pub fn new () -> Self {Self {  }}
}

impl GetRunnable for Set {
    fn create(&self) -> Box<dyn Runnable> {
        return Box::from(Self::new());
    }
}

impl Runnable for Set {
    fn run<'a> (
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
        }

        else if arguments.len() % 2 != 0 {
            return_struct.exit_code = 1;
            return_struct.output = Output::StandardOutput(
                String::from("Unexpected length of arguments\n")
            );
            return return_struct.to_owned();
        }

        else {
            let mut idx: usize = 0;
            while idx < arguments.len() {

                println!("{}", arguments[idx]);
                let variable_map = &mut return_struct
                    .vars
                    .borrow_mut();

                match variable_map
                    .get_mut(&arguments[idx])
                {

                    Some(variable) => {
                        if variable.editable && variable.mutable {
                            variable.value = arguments[idx+1].to_string();
                        }
 
                        else {
                            return_struct.output = Output::StandardOutput(
                                format!(
                                    "{} is not editable by the user or is not mutable\n",
                                    &arguments[idx]
                                )
                            );
                            return_struct.exit_code = 1;
                        }
                    },
 
                    None => {
                        variable_map
                            .insert(
                                arguments[idx].clone(), 
                                OptionProperties::from(
                                    true,
                                    arguments[idx+1].clone()
                                )
                            );
                        return_struct.exit_code = 0;
                    }
                }
                idx+=2;
            }
            return_struct.to_owned()
        }
    }
}