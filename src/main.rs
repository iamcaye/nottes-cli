use std::env;
use std::process::Command;
use clap::{Parser, Subcommand, arg, command};

pub const BASE_DIR: &str = "~/.nottes";

mod cli;
mod db;

#[derive(Parser)]
#[command(name = "nottes", version, about = "note-taking cli app")]
struct Args {
    #[command(subcommand)]
    command: Option<NoteCommand>,
}

#[derive(Subcommand)]
enum NoteCommand {
    Add {
        title: String,
    },
    Edit {
        title: Option<String>,
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

    match db::init() {
        Ok(_) => println!("Local database initialized successfully."),
        Err(e) => eprintln!("Failed to initialize database: {}", e),
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
        Some(NoteCommand::Edit { title }) => {
            let notes = cli::get_notes_by_title(title.as_deref().unwrap_or(""))?;
            if notes.is_empty() {
                println!("No notes found with the title: {}", title.unwrap_or("".to_string()));
                return Ok(());
            }
            
            let mut fzf_command: String = "echo '".to_string();
            for note in notes {
                fzf_command.push_str(&format!("{}\n", note.path));
            }
            fzf_command.push_str("' | fzf");

            let output = Command::new("sh")
                .arg("-c")
                .arg(fzf_command)
                .output()?;

            if output.status.success() {
                let selected_note = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));
                println!("Opening note: {}", selected_note);
                let filepath = shellexpand::tilde(&selected_note).to_string();
                Command::new(editor)
                    .arg(filepath)
                    .status()?;
            } else {
                eprintln!("Error selecting note: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }

    Ok(())
}
