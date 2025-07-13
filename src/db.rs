use rusqlite::Connection;
use anyhow::Result;

pub fn init () -> Result<()> {
    let conn = get_connection()?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            slug TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS note_tags (
            note_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            FOREIGN KEY (note_id) REFERENCES notes(id),
            FOREIGN KEY (tag_id) REFERENCES tags(id),
            PRIMARY KEY (note_id, tag_id)
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS notes_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            note_id INTEGER NOT NULL,
            content TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (note_id) REFERENCES notes(id)
        )",
        [],
    )?;

    conn.close().expect("Failed to close database connection");
    Ok(())
}

pub fn get_connection () -> Result<Connection>  {
    let path = shellexpand::tilde(crate::BASE_DIR).to_string() + "/.db/nottes.db";
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir_all(std::path::Path::new(&path).parent().unwrap())
            .expect("Failed to create database directory");
    }

    let conn = Connection::open(path)?;

    Ok(conn)
}
