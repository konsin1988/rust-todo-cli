use crate::storage;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use clap::ValueEnum;
use std::fmt;
use chrono::{DateTime, NaiveDateTime, Local, TimeZone};


#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub text: String,
    pub done: bool,
    pub priority: Option<Priority>,
    pub tags: Vec<String>,
    pub due_date: Option<DateTime<Local>>,
}

pub struct TodoList {
    todos: Vec<Todo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl FromStr for Priority {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "high" => Ok(Priority::High),
            "medium" => Ok(Priority::Medium),
            "low" => Ok(Priority::Low),
            _ => Err(format!("Invalid priority '{}'. Allowed values: high, medium, low", s)),
        }
    }
}

impl fmt::Display for Priority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Priority::High => "high",
            Priority::Medium => "medium",
            Priority::Low => "low",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum TodoError {
    Io(std::io::Error),
    NotFound(usize),
    Json(serde_json::Error),
    InvalidDateTime,
}

impl std::fmt::Display for TodoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoError::Io(e) => write!(f, "IO error: {}", e),
            TodoError::Json(e) => write!(f, "Json error: {}", e),
            TodoError::NotFound(id) => write!(f, "Todo with id {} not found", id),
            TodoError::InvalidDateTime => write!(f, "Invalid DateTime"),
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

    pub fn add(&mut self, text: String, 
               priority: Option<Priority>, 
               tags: Vec<String>, 
               due: Option<NaiveDateTime>) -> Result<(), TodoError> {
        let id = self.todos.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        let due_date: Option<DateTime<Local>> = match due {
            Some(naive) => {
                let local_dt = Local
                    .from_local_datetime(&naive)
                    .single()
                    .ok_or(TodoError::InvalidDateTime)?;

                Some(local_dt)
            }
            None => None,
        };
        self.todos.push(Todo {
            id, 
            text, 
            done: false,
            priority,
            tags,
            due_date: due_date,
        });
        self.save()
    }

    pub fn print(&self, filter_priority: Option<Priority>, 
                 filter_tag: Option<&str>, 
                 filter_due: Option<NaiveDateTime>) {
        let todos = self.todos.iter().filter(|todo| {
            let priority_match = filter_priority.as_ref().map_or(true, |p| todo.priority.as_ref() == Some(p));
            let tag_match = filter_tag.map_or(true, |t| todo.tags.contains(&t.to_string()));
            let filter_due_with_timezone: Option<DateTime<Local>> = filter_due.map(|naive| {
                Local.from_local_datetime(&naive)
                    .single()
                    .unwrap_or_else(|| Local::now()) // fallback for ambiguous/invalid times
            });
            let due_match = filter_due_with_timezone.map_or(true, |date| {
                todo.due_date.map_or(false, |d| d <= date)
            });
            priority_match && tag_match && due_match
        });

        if todos.clone().count() == 0 {
            println!("No todos found");
            return;
        }
        for todo in todos {
            let status = if todo.done { "✔" } else { " " };
            let tags = if todo.tags.is_empty() {"".into()} else {format!(" [{}]", todo.tags.join(",")) };
            let priority = todo.priority.as_ref().map(|p| p.to_string()).unwrap_or_default();
            let due = todo.due_date
                .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_default();
            println!("[{}] {}: {} {} {} {}", status, todo.id, todo.text, priority, tags, due);
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

    pub fn toggle(&mut self, id: usize) -> Result<(), TodoError>{
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id){
            todo.done = !todo.done;
            self.save()?;
            Ok(())
        } else {
            Err(TodoError::NotFound(id))
        }
    }
}























