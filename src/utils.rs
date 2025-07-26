use std::env;
use std::process::Command;
use anyhow::Result;

pub fn open_in_editor<P: AsRef<str>>(file_path: P) -> Result<()> {
    let editor = env::var("EDITOR").unwrap_or_else(|_| String::from("vi"));
    let filepath = shellexpand::tilde(file_path.as_ref()).to_string();
    Command::new(editor).arg(filepath).status()?;
    Ok(())
}