use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
};

use crate::log;

const DATABASE_PATH: &str = "todos.txt";

pub fn get_file() -> File {
    let result = OpenOptions::new()
        .append(true)
        .create(true)
        .read(true)
        .open(DATABASE_PATH);

    match result {
        Ok(file) => file,
        Err(_) => panic!("Couldn't create file! Exiting..."),
    }
}

pub fn write_todo(todo: String) {
    let mut file = get_file();
    let content = format!("{}\n", todo);

    match file.write_all(content.as_bytes()) {
        Err(_) => panic!("Couldn't write to file. Something is very wrong :S"),
        Ok(_) => log::print("Wrote todo!"),
    }
}

pub fn clear() {
    match fs::remove_file(DATABASE_PATH) {
        Err(_) => panic!("Couldn't remove file for some reason..."),
        Ok(_) => log::print("Cleared todos!"),
    }
}

pub fn get_all() -> Vec<String> {
    let mut file = get_file();

    let mut contents = String::new();

    return match file.read_to_string(&mut contents) {
        Err(err) => {
            log::print("Could not read file");
            panic!("{}", err.to_string())
        }
        Ok(_) => contents
            .split("\n")
            .map(|item| item.trim().to_string())
            .filter(|item| !String::is_empty(item))
            .collect(),
    };
}

pub fn overwrite(value: String) {
    match File::create(DATABASE_PATH) {
        Err(_) => panic!("Could not overwrite file! Exiting..."),
        Ok(mut file) => match file.write_all(value.as_bytes()) {
            Err(_) => panic!("Could not overwrite file! Exiting..."),
            Ok(_) => log::print("Deleted todo..."),
        },
    };
}
