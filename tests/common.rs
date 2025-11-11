use rusqlite::Connection;
use std::env;
use std::fs;
use std::path::PathBuf;

/// Restituisce il percorso completo di un database temporaneo.
/// Usa %TEMP% su Windows e /tmp su Unix-like.
pub fn temp_db_path(name: &str) -> PathBuf {
    let mut dir = env::temp_dir();
    dir.push(format!("librius_test_{}.db", name));
    dir
}

/// Crea un database SQLite temporaneo con lo **schema di produzione**.
pub fn setup_temp_db(name: &str) -> Connection {
    let path = temp_db_path(name);

    // Elimina se già esiste
    if path.exists() {
        let _ = fs::remove_file(&path);
    }

    let conn = Connection::open(&path).expect("Impossibile creare il database di test");

    // Replica esatto schema di produzione
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            editor TEXT NOT NULL,
            year INTEGER NOT NULL,
            isbn TEXT NOT NULL,
            language TEXT,
            pages INTEGER,
            genre TEXT,
            summary TEXT,
            room TEXT,
            shelf TEXT,
            row TEXT,
            position TEXT,
            added_at TEXT
        );
        "#,
    )
    .expect("Errore nella creazione dello schema");

    conn
}
