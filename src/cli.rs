use crate::BASE_DIR;

pub fn add_note(title: String) -> anyhow::Result<String> {
    println!("Adding note with title: {}", title);
    // save file using base directory with title slugified
    let file_path = format!("{}/{}.md", BASE_DIR, title.replace(" ", "_"));
    let file_path = shellexpand::tilde(&file_path).to_string();
    if std::path::Path::new(&file_path).exists() {
        return Ok(file_path);
    }

    let content = format!("# {}", title);
    let res = std::fs::write(&file_path, content);

    match res {
        Ok(_) => {
            println!("Note added successfully at: {}", file_path);
            Ok(file_path)
        }
        Err(e) => return Err(anyhow::anyhow!("Failed to write note: {}", e)),
    }
}
