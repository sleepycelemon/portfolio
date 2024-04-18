use crate::{cli, database, log};

pub fn list() {
    let todos = database::get_all();

    log::print_todos(&todos);
}

pub fn add() {
    let todo = cli::ask("Enter todo text: ");

    database::write_todo(todo);
}

pub fn delete() {
    let mut todos = database::get_all();

    log::print_todos(&todos);

    let to_delete = cli::ask("Please enter the number of the todo you wish to delete.");

    let idx = match to_delete.parse::<usize>() {
        Ok(v) => v,
        Err(_) => panic!("Please enter a number"),
    };

    todos.remove(idx - 1);

    database::overwrite(todos.join("\n"));
}

pub fn clear() {
    database::clear();
}
