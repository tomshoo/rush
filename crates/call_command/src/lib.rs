use commands;
use std::collections::HashMap;
pub use return_structure::{ReturnStructure, Output};

#[derive(Debug, Clone, Copy)]
pub enum Commands{
    Clear(commands::clear::ClearScreen),
    Exit,
    ChangeDirectory(commands::cd::ChangeDirectory),
    GetChildren(commands::gc::GetChildren),
    None
}

pub enum ShellStatus<'shell_stat> {
    Terminate(i32),
    Maintain(ReturnStructure<'shell_stat>)
}

struct CreateCommand<'cmd> {
    return_object: ReturnStructure<'cmd>
}

pub struct CallCommand<'call> {
    command_creator: Option<CreateCommand<'call>>,
    map: HashMap<&'static str, Commands>
}

impl<'call> CreateCommand<'call> {
    pub fn new() -> Self {
        Self{
            return_object: ReturnStructure::from(
                0,
                HashMap::new(),
                Output::StandardOutput(String::new())
            )
        }
    }

    pub fn run(
        &mut self,
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
            Commands::None => {
                self.return_object = ReturnStructure {
                    exit_code: 127,
                    vars: HashMap::new(),
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

impl<'cmd> CallCommand<'cmd> {

    pub fn new() -> Self {
        Self {
            command_creator: None,
            map: HashMap::new()
        }
    }

    pub fn init(&mut self) -> () {
        self.map = HashMap::from([
            ("exit", Commands::Exit),
            ("clear", Commands::Clear(commands::clear::ClearScreen)),
            ("cd", Commands::ChangeDirectory(commands::cd::ChangeDirectory)),
            ("gc", Commands::GetChildren(commands::gc::GetChildren))
        ]);
        if let None = self.command_creator {
            self.command_creator = Some(CreateCommand::new());
        }
    }

    pub fn run<'a>(
        &mut self, command:&'a str,
        command_arguments: Vec<String>,
    ) -> Result<ShellStatus, &str> {
        if let Some(command_object) = &mut self.command_creator {
            Ok(command_object.run(
                if let Some(c) = self.map.get(command) {c} else {&Commands::None},
                &command_arguments
            ))
        }
        else {
            Err("No command creator found, is struct init called?")
        }
    }
}
