use crate::{db, BASE_DIR};

pub fn add_note(title: String) -> anyhow::Result<String> {
    let file_path = format!("{}/{}.md", BASE_DIR, title.replace(" ", "_"));
    let file_path = shellexpand::tilde(&file_path).to_string();
    if std::path::Path::new(&file_path).exists() {
        return Ok(file_path);
    }

    let content = format!("# {}", title);
    let res = std::fs::write(&file_path, content);

    let conn = db::get_connection()?;
    conn.execute(
        "INSERT INTO notes (title, slug) VALUES (?1, ?2)",
        rusqlite::params![title, title.replace(" ", "_")],
    )?;

    match res {
        Ok(_) => {
            Ok(file_path)
        }
        Err(e) => return Err(anyhow::anyhow!("Failed to write note: {}", e)),
    }
}
