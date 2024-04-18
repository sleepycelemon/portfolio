use std::{env::args, fmt::Error};

use crate::command;

pub fn read() -> Vec<String> {
    let args: Vec<String> = args().collect();

    args[1..].to_vec()
}

pub fn to_command(args: Vec<String>) -> Result<command::Command, Error> {
    if args.len() == 0 {
        return Err(Error {});
    }

    return command::get(&args[0]);
}
