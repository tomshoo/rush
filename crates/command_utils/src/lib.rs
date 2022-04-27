use commands;
use std::collections::HashMap;
use return_structure::ReturnStructure;
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

impl CreateCommand {
    pub fn new() -> Self {
        Self{ 
            return_object: ReturnStructure {
                exit_code: 0
            }
        }
    }

    pub fn run(&mut self, command: &Commands, command_arguments: &Vec<String>) -> Option<ReturnStructure> {
        println!("{:?}", command);
        match command {
            Commands::Clear(c) => {
                c.run(command_arguments, &mut self.return_object);
            },
            Commands::Exit(c) => {
                c.run(command_arguments, &mut self.return_object);
            },
            Commands::ChangeDirectory(c) => {
                c.run(command_arguments, &mut self.return_object);
            },
            Commands::GetChildren(c) => {
                c.run(command_arguments, &mut self.return_object);
            }
            Commands::None => {
                self.return_object = ReturnStructure {
                    exit_code: 1
                }
            }
        }
        None
    }
}

pub fn init() -> HashMap<&'static str, Commands> {
    let command_map = HashMap::from([
        ("clear", Commands::Clear(commands::clear::ClearScreen)),
        ("exit", Commands::Exit(commands::exit::Exit)),
        ("cd", Commands::ChangeDirectory(commands::cd::ChangeDirectory)),
        ("gc", Commands::GetChildren(commands::gc::GetChildren))
    ]);
    return command_map;
}

pub fn run(command:&str, command_arguments: Vec<String>, map: &HashMap<&str, Commands>) {
    let command_enum = match map.get(command) {
        Some(object) => object,
        None => &Commands::None
    };
    let mut command_object = CreateCommand::new();
    command_object.run(command_enum, &command_arguments);
}