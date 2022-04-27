use commands;
use std::collections::HashMap;
pub use return_structure::ReturnStructure;

#[derive(Debug, Clone, Copy)]
pub enum Commands{
    Clear(commands::clear::ClearScreen),
    Exit(commands::exit::Exit),
    ChangeDirectory(commands::cd::ChangeDirectory),
    GetChildren(commands::gc::GetChildren),
    None
}

struct CreateCommand {
    return_object: ReturnStructure
}

pub struct CallCommand {
    command_creator: Option<CreateCommand>
}

impl CreateCommand {
    pub fn new() -> Self {
        Self{ 
            return_object: ReturnStructure {
                exit_code: 0
            }
        }
    }

    pub fn run(&mut self, command: &Commands, command_arguments: &Vec<String>) -> ReturnStructure {
        println!("{:?}", command);
        match command {
            Commands::Clear(c) => {
                c.run(command_arguments, &mut self.return_object)
            },
            Commands::Exit(_) => {
                self.return_object.clone()
            },
            Commands::ChangeDirectory(c) => {
                c.run(command_arguments, &mut self.return_object)
            },
            Commands::GetChildren(c) => {
                c.run(command_arguments, &mut self.return_object)
            }
            Commands::None => {
                self.return_object = ReturnStructure {
                    exit_code: 127
                };
                self.return_object.clone()
            }
        }
    }
}

impl CallCommand {

    pub fn new() -> Self {Self {command_creator: None}}

    pub fn init(&mut self) -> HashMap<&'static str, Commands> {
        let command_map = HashMap::from([
            ("exit", Commands::Exit(commands::exit::Exit)),
            ("clear", Commands::Clear(commands::clear::ClearScreen)),
            ("cd", Commands::ChangeDirectory(commands::cd::ChangeDirectory)),
            ("gc", Commands::GetChildren(commands::gc::GetChildren))
        ]);
        if let None = self.command_creator {
            self.command_creator = Some(CreateCommand::new());
        }
        return command_map;
    }

    pub fn run<'a>(&mut self, command:&'a str, command_arguments: Vec<String>, map: &HashMap<&str, Commands>) -> Result<(&'a str, ReturnStructure), &str> {
        let command_enum = match map.get(command) {
            Some(object) => object,
            None => &Commands::None
        };
        match &mut self.command_creator {
            Some(command_object) => {
                Ok((command, command_object.run(command_enum, &command_arguments)))
            },
            None => {
                Err("Failed to locate the command creator object, make sure you called the init function before using this function")
            }
        }
    }
}
