use std::fs::File;
use std::io::Write;

use serde::{Deserialize, Serialize};

const DATA_FILE: &str = "data.json";

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: u32,
    pub title: String,
    pub completed: bool,
}

pub struct Todos {
    todos: Vec<Todo>,
}

impl Todos {
    pub fn from_file() -> Self {
        // Ensure the data file exists, creating it if necessary.
        if !std::path::Path::new(DATA_FILE).exists() {
            // Call the associated function from the PersistTodos trait implementation.
            // This requires `Self` (which is `Todos` here) to implement `PersistTodos`,
            // and `create_file` to be an associated function (not taking `&self` or `&mut self`),
            // which it is.
            <Self as PersistTodos>::create_file();
        }

        let data_file = File::open(DATA_FILE).expect("Failed to open data file");
        let todos: Vec<Todo> = serde_json::from_reader(data_file).expect("Failed to read todos");
        Self { todos }
    }
}

pub trait PersistTodos {
    fn get_all_todos(&self) -> Vec<Todo>;
    fn get_todo_by_id(&self, id: u32) -> Option<Todo>;
    fn add_todo(&mut self, todo: Todo);
    fn update_todo(&mut self, id: u32, todo: Todo) -> bool;
    fn delete_todo(&mut self, id: u32) -> bool;
    fn clear_todos(&mut self);
    fn create_file();
}

impl PersistTodos for Todos {
    fn create_file() {
        if std::path::Path::new(DATA_FILE).exists() {
            return; // File already exists, no need to create it
        }
        let mut data_file = File::create(DATA_FILE).expect("Failed to create data file");
        let initial_data =
            serde_json::to_string(&Vec::<Todo>::new()).expect("Failed to serialize initial data");
        data_file
            .write_all(initial_data.as_bytes())
            .expect("Failed to write to data file");
    }

    fn get_all_todos(&self) -> Vec<Todo> {
        // Ensure the file exists before reading
        if !std::path::Path::new(DATA_FILE).exists() {
            Self::create_file(); // Create the file if it doesn't exist
        }
        let data_file = File::open(DATA_FILE).expect("Failed to open data file");
        let todos: Vec<Todo> = serde_json::from_reader(data_file).expect("Failed to read todos");
        todos
    }

    fn get_todo_by_id(&self, id: u32) -> Option<Todo> {
        if !std::path::Path::new(DATA_FILE).exists() {
            Self::create_file(); // Create the file if it doesn't exist
        }
        let data_file = File::open(DATA_FILE).expect("Failed to open data file");
        let todos: Vec<Todo> = serde_json::from_reader(data_file).expect("Failed to read todos");
        todos.into_iter().find(|todo| todo.id == id)
    }

    fn add_todo(&mut self, todo: Todo) {
        if !std::path::Path::new(DATA_FILE).exists() {
            Self::create_file(); // Create the file if it doesn't exist
        }
        let data_file = File::open(DATA_FILE).expect("Failed to open data file");
        let mut todos: Vec<Todo> =
            serde_json::from_reader(data_file).expect("Failed to read todos");
        todos.push(todo);
        let updated_data = serde_json::to_string(&todos).expect("Failed to serialize todos");
        let mut data_file = File::create(DATA_FILE).expect("Failed to create data file");
        data_file
            .write_all(updated_data.as_bytes())
            .expect("Failed to write updated todos");
        self.todos = todos; // Update the in-memory list
    }

    fn update_todo(&mut self, id: u32, todo: Todo) -> bool {
        if !std::path::Path::new(DATA_FILE).exists() {
            Self::create_file(); // Create the file if it doesn't exist
        }
        let data_file = File::open(DATA_FILE).expect("Failed to open data file");
        let mut todos: Vec<Todo> =
            serde_json::from_reader(data_file).expect("Failed to read todos");

        if let Some(pos) = todos.iter_mut().position(|t| t.id == id) {
            todos[pos] = todo;
            let updated_data = serde_json::to_string(&todos).expect("Failed to serialize todos");
            let mut data_file = File::create(DATA_FILE).expect("Failed to create data file");
            data_file
                .write_all(updated_data.as_bytes())
                .expect("Failed to write updated todos");
            self.todos = todos; // Update the in-memory list
            true
        } else {
            false
        }
    }

    fn delete_todo(&mut self, id: u32) -> bool {
        if !std::path::Path::new(DATA_FILE).exists() {
            Self::create_file(); // Create the file if it doesn't exist
        }
        let data_file = File::open(DATA_FILE).expect("Failed to open data file");
        let mut todos: Vec<Todo> =
            serde_json::from_reader(data_file).expect("Failed to read todos");

        if let Some(pos) = todos.iter().position(|t| t.id == id) {
            todos.remove(pos);
            let updated_data = serde_json::to_string(&todos).expect("Failed to serialize todos");
            let mut data_file = File::create(DATA_FILE).expect("Failed to create data file");
            data_file
                .write_all(updated_data.as_bytes())
                .expect("Failed to write updated todos");
            self.todos = todos; // Update the in-memory list
            true
        } else {
            false
        }
    }
    fn clear_todos(&mut self) {
        if !std::path::Path::new(DATA_FILE).exists() {
            Self::create_file(); // Create the file if it doesn't exist
        }
        let mut data_file = File::create(DATA_FILE).expect("Failed to create data file");
        let empty_data =
            serde_json::to_string(&Vec::<Todo>::new()).expect("Failed to serialize empty data");
        data_file
            .write_all(empty_data.as_bytes())
            .expect("Failed to write empty todos");
        self.todos.clear(); // Clear the in-memory list
    }
}
