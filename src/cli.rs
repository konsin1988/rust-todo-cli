use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Add a new todo
    Add { 
        /// The description of the todo
        text: String,
    },
    /// List all todos
    List,
    /// Mark a todo as done
    Done {
        /// ID of the todo
        id: usize,
    },
    /// Remove a todo
    Remove {
        /// ID of the todo
        id: usize,
    },
}

pub fn parse_args() -> Result<Command, String>{
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("add") => {
            let text = args.next().ok_or("Missing todo text")?;
            Ok(Command::Add { text })
        }
        Some("list") => Ok(Command::List),
        Some(cmd) => Err(format!("Unknown command {}", cmd)),
        None => Err("No command provided".into()),
    }
}
