use std::env;
use std::process::Command;
use clap::{Parser, Subcommand, arg, command};

pub const BASE_DIR: &str = "~/.nottes";

mod cli;

#[derive(Parser)]
#[command(name = "nottes", version, about = "note-taking cli app")]
struct Args {
    #[command(subcommand)]
    command: Option<NoteCommand>,
}

#[derive(Subcommand)]
enum NoteCommand {
    Add {
        #[arg(short, long)]
        title: String,
    },
    List,
    Read {
        #[arg()]
        id: String,
    },
}

fn init() {
    // Initialize the base directory if it doesn't exist
    let base_dir = shellexpand::tilde(BASE_DIR).to_string();
    if !std::path::Path::new(&base_dir).exists() {
        std::fs::create_dir_all(&base_dir).expect("Failed to create base directory");
    }
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init();

    match args.command {
        Some(NoteCommand::Add { title }) => {
            println!("Adding note with title: {}", title);
            let file = cli::add_note(title)?;
            let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));
            Command::new(editor)
                .arg(file)
                .status()?;

        }
        Some(NoteCommand::List) => {
            println!("Listing all notes");
            // Here you would add the logic to list notes
        }
        Some(NoteCommand::Read { id }) => {
            println!("Reading note with ID: {}", id);
            // Here you would add the logic to read a specific note
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }

    Ok(())
}
