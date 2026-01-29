use crate::storage;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub done: bool,
}

pub struct TodoList {
    todos: Vec<Todo>,
}

#[derive(Debug)]
pub enum TodoError {
    Io(std::io::Error),
    NotFound(usize),
    Json(serde_json::Error),
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::Io(e) => write!(f, "IO error: {}", e),
            TodoError::Json(e) => write!(f, "Json error: {}", e),
            TodoError::NotFound(id) => write!(f, "Todo with id {} not found", id),
        }
    }
}

impl std::error::Error for TodoError {}

impl TodoList {
    pub fn load() -> Result<Self, TodoError> {
        let todos = storage::load().map_err(TodoError::Io)?;
        Ok(Self { todos })
    }

    fn save(&self) -> Result<(), TodoError> {
        storage::save(&self.todos).map_err(TodoError::Io)
    }

    pub fn add(&mut self, text: String) -> Result<(), TodoError> {
        let id = self.todos.len() + 1;
        self.todos.push(Todo {id, text, done: false });
        self.save()
    }

    pub fn print(&self) {
        if self.todos.is_empty() {
            println!("No todos yet");
            return;
        }
        for todo in &self.todos {
            let status = if todo.done { "✔" } else { " " };
            println!("[{}] {}: {}", status, todo.id, todo.text);
        }
    }

    pub fn mark_done(&mut self, id: usize) -> Result<(), TodoError> {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id){
            todo.done = true;
            self.save()
        } else {
            Err(TodoError::NotFound(id))
        }
    }

    pub fn remove(&mut self, id: usize) -> Result<(), TodoError>{
        if let Some(pos) = self.todos.iter().position(|t| t.id == id){
            self.todos.remove(pos);
            self.save()
        } else {
            Err(TodoError::NotFound(id))
        }
    }


    pub fn list(&self) -> &[Todo] {
        &self.todos
    }
}























