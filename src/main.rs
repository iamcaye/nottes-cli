use clap::{command, Parser, Subcommand, arg};

#[derive(Parser)]
#[command(name = "nottes", version, about = "note-taking cli app")]
struct Args {
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    match args.command {
        Some(Command::Add { title }) => {
            println!("Adding note with title: {}", title);
            // Here you would add the logic to create a note
        }
        Some(Command::List) => {
            println!("Listing all notes");
            // Here you would add the logic to list notes
        }
        Some(Command::Read { id }) => {
            println!("Reading note with ID: {}", id);
            // Here you would add the logic to read a specific note
        }
        None => {
            println!("No command provided. Use --help for more information.");
        }
    }

    Ok(())
}