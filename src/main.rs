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

fn delete_todo(todos: &mut Vec<Todo>, index: usize) {
    if index >= todos.len() {
        println!("Index out of bounds");
        return;
    }
    todos.remove(index);
}

fn complete_todo(todos: &mut Vec<Todo>, index: usize) {
    match todos.get_mut(index) {
        Some(todo) => {
            todo.is_complete = true;
        }
        None => {
            println!("Todo not found");
        }
    }
}

fn main() {
    let mut todos = vec![];

    add_todo(&mut todos, "Learn Rust");
    add_todo(&mut todos, "Build a rust project");
    add_todo(&mut todos, "Apply for a job in rust");
    complete_todo(&mut todos, 2);
    delete_todo(&mut todos, 100);
    for todo in todos {
        println!("Todo: {}, Completed: {}", todo.item, todo.is_complete);
    }
}
