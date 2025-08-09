use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(author, version, about = "RustyLife CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Chemin vers le fichier de stockage (optionnel)
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
    },
    List,
    Done {
        id: u64,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Fichier à utiliser : soit celui passé, soit "tasks.json" par défaut
    let path = cli.file.unwrap_or_else(|| PathBuf::from("tasks.json"));

    match cli.command {
        Commands::Add { title } => {
            let task = rustylife::add_task(&path, title)?;
            println!("Added: {} (id={})", task.title, task.id);
        }
        Commands::List => {
            let tasks = rustylife::load_tasks(&path)?;
            for t in tasks {
                println!("[{}] {} - {}", t.id, t.title, if t.done { "done" } else { "todo" });
            }
        }
        Commands::Done { id } => {
            if let Some(t) = rustylife::mark_done(&path, id)? {
                println!("Marked done: {}", t.title);
            } else {
                println!("No task with id {}", id);
            }
        }
    }

    Ok(())
}