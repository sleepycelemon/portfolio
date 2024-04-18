pub fn print(log: &str) {
    println!("{}", log)
}

pub fn invalid_command() {
    print("Invalid command.")
}

pub fn manual() {
    print("Usage:");
    print("\t list - show all todos");
    print("\t add - add todo");
    print("\t delete - delete todo");
    print("\t clear - clear all todos");
}

pub fn print_todos(todos: &Vec<String>) {
    todos
        .iter()
        .enumerate()
        .for_each(|(i, item)| print(&format!("{}. {}", i + 1, item)))
}
