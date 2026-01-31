use clap::{Parser, Subcommand};
use crate::todo::Priority;

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

        /// Optional priority: high, medium, low
        #[arg(short, long)]
        priority: Option<Priority>,

        /// Optional tags (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },
    /// List all todos
    List {
        /// Oprional priority: high, medium, low
        #[arg(short, long)]
        priority: Option<Priority>,

        /// Optional tags (comma-separated)
        #[arg(short, long)] 
        tag: Option<String>,
    },
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
    /// Toggle done/undone status
    Toggle {
        /// ID of the todo
        id: usize,
    },
}

pub fn parse_args() -> Result<Command, String>{
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("add") => {
            let text = args.next().ok_or("Missing todo text")?;
            let mut priority: Option<Priority> = None;
            let mut tags: Vec<String> = Vec::new();

            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--priority" => {
                        let value = args.next().ok_or("Missing value for --priority")?;
                        priority = Some(value.parse()?);
                    }
                    "--tag" => {
                        tags.push(args.next().ok_or("Missing value for --tag")?);
                    }
                    _ => return Err(format!("Unknown argument: {}", arg)),
                }
            }
            Ok(Command::Add { text, priority, tags })
        }
        Some("list") => {
            let mut priority: Option<Priority> = None;
            let mut tag: Option<String> = None; 
            while let Some(arg) = args.next() {
                match arg.as_str() {
                    "--priority" => {
                        let value = args.next().ok_or("Missing value for --priority")?;
                        priority = Some(value.parse()?);
                    }
                    "--tag" => {
                        tag = Some(args.next().ok_or("Missing value for --tag")?);
                    }
                    _ => return Err(format!("Unknown argument: {}", arg)),
                }
            }
            Ok(Command::List{ priority, tag })
        },
        Some(cmd) => Err(format!("Unknown command {}", cmd)),
        None => Err("No command provided".into()),
    }
}
