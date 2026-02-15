use clap::Parser;
use todo_cli::{cli::Cli, todo::TodoList};

fn main() {
    let cli = Cli::parse();

    let mut todos = match TodoList::load() {
        Ok(list) => list,
        Err(err) => {
            eprintln!("Error loading todos: {}", err);
            return;
        }
    };

    match cli.command {
        todo_cli::cli::Command::Add { text, priority, tags, due} => {
            if let Err(err) = todos.add(text, priority, tags, due) {
                eprintln!("Error adding todo: {}", err);
            }
        }
        todo_cli::cli::Command::List { priority, tag, due_before } => {
            todos.print(priority, tag.as_deref(), due_before);
        }
        todo_cli::cli::Command::Done { id } => {
            if let Err(err) = todos.mark_done(id) {
                eprintln!("Error: {}", err);
            }
        }
        todo_cli::cli::Command::Remove { id } => {
            if let Err(err) = todos.remove(id) {
                eprintln!("Error: {}", err);
            }
        }
        todo_cli::cli::Command::Toggle { id } => {
            if let Err(err) = todos.toggle(id) {
                eprintln!("Error: {}", err);
            }
        }
    }
}

