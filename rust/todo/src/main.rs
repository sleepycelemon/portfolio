mod args;
mod cli;
mod command;
mod database;
mod log;
mod todos;

fn main() {
    let args: Vec<String> = args::read();

    match args::to_command(args) {
        Ok(command) => match command {
            command::Command::List => todos::list(),
            command::Command::Add => todos::add(),
            command::Command::Delete => todos::delete(),
            command::Command::Clear => todos::clear(),
        },
        Err(_) => {
            log::invalid_command();
            log::manual();
        }
    }
}
