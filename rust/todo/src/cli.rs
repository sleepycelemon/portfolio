use std::io;

use crate::log;

pub fn ask(prompt: &str) -> String {
    loop {
        log::print(prompt);

        let mut input = String::new();

        if io::stdin().read_line(&mut input).is_err() {
            log::print("Something went wrong. Please try again.");
            continue;
        }

        if input.trim().len() != 0 {
            return String::from(input.trim());
        }

        log::print("Please enter something.");
    }
}
