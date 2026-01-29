use crate::todo::Todo;
use std::fs;
use std::path::Path;
use std::io::{self, Write};

const FILE_PATH: &str = "todos.json";

pub fn load() -> io::Result<Vec<Todo>> {
    if !Path::new(FILE_PATH).exists() {
        return Ok(Vec::new());
    }

    let data = fs::read_to_string(FILE_PATH)?;
    let todos: Vec<Todo> = serde_json::from_str(&data)?;
    Ok(todos)
}

pub fn save(todos: &[Todo]) -> io::Result<()> {
    let data = serde_json::to_string_pretty(todos)?;
    let mut file = fs::File::create(FILE_PATH)?;
    file.write_all(data.as_bytes())
}
