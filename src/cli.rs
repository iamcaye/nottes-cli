use crate::{
    BASE_DIR,
    db::{self, Note},
};

pub fn add_note(title: String) -> anyhow::Result<String> {
    let file_path = format!("{}/{}.md", BASE_DIR, title.replace(" ", "_"));
    let file_path = shellexpand::tilde(&file_path).to_string();
    if std::path::Path::new(&file_path).exists() {
        return Ok(file_path);
    }

    let content = format!(
        "---
title: {}
---

",
        title
    );
    let res = std::fs::write(&file_path, content);

    match res {
        Ok(_) => {
            let conn = db::get_connection()?;
            conn.execute(
                "INSERT INTO notes (title, slug) VALUES (?1, ?2)",
                rusqlite::params![title, title.replace(" ", "_")],
            )
            .expect("Failed to insert note into database");

            conn.close().expect("Failed to close database connection");
            Ok(file_path)
        }
        Err(e) => return Err(anyhow::anyhow!("Failed to write note: {}", e)),
    }
}

pub fn get_all_notes() -> anyhow::Result<Vec<Note>> {
    let conn = db::get_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM notes ORDER BY created_at DESC")?;
    let notes_iter = stmt.query_map([], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(3)?,
            created_at: row.get(2)?,
            path: format!("{}/{}.md", BASE_DIR, row.get::<_, String>(3)?),
        })
    })?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note?);
    }

    drop(stmt);
    conn.close().expect("Failed to close database connection");
    Ok(notes)
}

pub fn get_notes_by_title(title: &str) -> anyhow::Result<Vec<Note>> {
    let conn = db::get_connection()?;
    let mut stmt = conn.prepare("SELECT * FROM notes WHERE title LIKE ?1")?;
    let notes_iter = stmt.query_map([format!("%{}%", title)], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            slug: row.get(2)?,
            created_at: row.get(3)?,
            path: format!("{}/{}.md", BASE_DIR, row.get::<_, String>(2)?),
        })
    })?;

    let mut notes = Vec::new();
    for note in notes_iter {
        notes.push(note?);
    }

    drop(stmt);
    conn.close().expect("Failed to close database connection");
    Ok(notes)
}
