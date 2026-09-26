struct Todo {
    item: String,
    is_complete: bool,
}

fn add_todo(todos: &mut Vec<Todo>, item: &str) {
    todos.push(Todo {
        item: String::from(item),
        is_complete: false,
    });
}

fn complete_todo(todos: &mut Vec<Todo>, index: usize) {
    todos[index].is_complete = true;
}

fn main() {
    let mut todos = vec![];

    add_todo(&mut todos, "Learn Rust");
    add_todo(&mut todos, "Build a rust project");
    add_todo(&mut todos, "Apply for a job in rust");
    complete_todo(&mut todos, 2);
    for todo in todos {
        println!("Todo: {}, Completed: {}", todo.item, todo.is_complete);
    }
}
