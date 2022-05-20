use commands;
use std::collections::HashMap;
use std::rc::Rc;
pub use shell_props::{ReturnStructure, Output};

#[derive(Debug, Clone, Copy)]
pub enum Commands{
    Clear(commands::clear::ClearScreen),
    Exit,
    ChangeDirectory(commands::cd::ChangeDirectory),
    GetChildren(commands::gc::GetChildren),
    Set(commands::set::Set),
    None
}

#[derive(Debug)]
pub enum ShellStatus {
    Terminate(i32),
    Maintain(ReturnStructure)
}

struct CreateCommand {
    return_object: ReturnStructure
}

pub struct CallCommand {
    command_creator: Option<CreateCommand>,
    map: HashMap<&'static str, Commands>
}

impl CreateCommand {
    pub fn new(return_struct: ReturnStructure) -> Self {
        Self{
            return_object: return_struct
        }
    }

    pub fn clear_output(&mut self) -> () {
        self.return_object.output = Output::StandardOutput(String::new());
    }

    pub fn run<'a>(
        &'a mut self,
        command: &Commands,
        command_arguments: &Vec<String>
    ) -> ShellStatus {
        match command {
            Commands::Clear(c) => {
                ShellStatus::Maintain(c.run(
                    command_arguments,
                    &mut self.return_object
                ))
            },
            Commands::Exit => {
                ShellStatus::Terminate(
                    self.return_object.exit_code
                )
            },
            Commands::ChangeDirectory(c) => {
                ShellStatus::Maintain(c.run(
                    command_arguments,
                    &mut self.return_object
                ))
            },
            Commands::GetChildren(c) => {
                ShellStatus::Maintain(c.run(
                    command_arguments,
                    &mut self.return_object
                ))
            }
            Commands::Set(c) => {
                ShellStatus::Maintain(c.run(
                    command_arguments,
                    &mut self.return_object
                ))
            }
            Commands::None => {
                self.return_object = ReturnStructure {
                    exit_code: 127,
                    vars: Rc::clone(&self.return_object.vars),
                    output: Output::StandardOutput(
                        "Error: could not find the command specified\n"
                        .to_string()
                    )
                };
                ShellStatus::Maintain(self.return_object.clone())
            }
        }
    }
}

impl CallCommand {

    pub fn new() -> Self {
        Self {
            command_creator: None,
            map: HashMap::new()
        }
    }

    pub fn init(&mut self, return_struct: ReturnStructure) -> () {
        self.map = HashMap::from([
            ("exit", Commands::Exit),
            ("clear", Commands::Clear(commands::clear::ClearScreen)),
            ("cd", Commands::ChangeDirectory(commands::cd::ChangeDirectory)),
            ("gc", Commands::GetChildren(commands::gc::GetChildren)),
            ("set", Commands::Set(commands::set::Set))
        ]);
        if let None = self.command_creator {
            self.command_creator = Some(CreateCommand::new(return_struct));
        }
        else {
            panic!("Command creater initialized more than once");
        }
    }

    pub fn run<'a>(
        &'a mut self, command:&'a str,
        command_arguments: Vec<String>,
    ) -> Result<ShellStatus, &'a str> {
        if let Some(command_object) = &mut self.command_creator {
            command_object.clear_output();
            Ok(command_object.run(
                if let Some(cmd) = self.map.get(command) {
                    cmd
                } else {
                    &Commands::None
                },
                &command_arguments
            ))
        }
        else {
            Err("No command creator found, is struct init called?")
        }
    }
}
