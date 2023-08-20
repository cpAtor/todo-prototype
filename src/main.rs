fn main() {

    let mut todo_list = TodoList::new();
    println!(r#"
        Hi, This is a todo app.
        You can use this app to manage your todo list.
        You have the following options:
        1. Add a todo
        2. Remove a todo
        3. List all todos
        4. Complete a todo
        Press any other key to exit
    "#);
 
    loop {
        let input = get_input("Please enter your choice: ");
        match input {
            1 => {
                let name: String = get_input("Enter the name of the todo: ");
                todo_list.add(Todo {
                    name,
                    completed: false,
                });                
            },
            2 => {
                let index: usize = get_input("Enter the index of the todo to remove: ");
                todo_list.remove(index);
            },
            3 => {
                todo_list.list();
            },
            4 => {
                let index: usize = get_input("Enter the index of the todo to complete: ");
                todo_list.complete(index)
            },
            _ =>{
                println!("Bye!");
                break;
            },
        }
    }

    
}

use std::str::FromStr;

fn get_input<T>(prompt: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: std::fmt::Debug,
    T::Err: std::fmt::Debug,
{
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let response: T = input.trim().parse().unwrap();
    response
}


struct Todo {
    name: String,
    completed: bool,
}

struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList{
    fn new() -> Self {
        Self {
            todos: Vec::new(),
        }
    }

    fn add(&mut self, todo: Todo) {
        self.todos.push(todo);
    }

    fn remove(&mut self, index: usize) {
        self.todos.remove(index);
    }

    fn list(&self) {
        for (index, todo) in self.todos.iter().enumerate() {
            println!("{} - [{}] {}", index, match todo.completed{true=>"✅", _=>" " }, todo.name);
        }
    }

    fn complete(&mut self, index: usize) {
        self.todos[index].completed = true;
    }
}
