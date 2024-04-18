use std::fmt::Error;

#[derive(PartialEq)]
pub enum Command {
    List,
    Clear,
    Add,
    Delete,
}

pub fn get(cmd: &str) -> Result<Command, Error> {
    return match cmd {
        "list" => Ok(Command::List),
        "clear" => Ok(Command::Clear),
        "add" => Ok(Command::Add),
        "delete" => Ok(Command::Delete),
        _ => panic!("Invalid command!"),
    };
}
