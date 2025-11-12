use crate::{AppConfig, print_err, print_ok, print_warn, tr, tr_with};
use rusqlite::Connection;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn handle_db(
    config: &AppConfig,
    init: bool,
    reset: bool,
    copy: bool,
    file: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    if init || reset {
        return init_db(config);
    }

    if copy {
        if let Some(dest) = file {
            return copy_db(config, dest);
        }
        print_err(&tr("db_copy_missing_file"));
        return Ok(());
    }

    print_warn(&tr("db_no_action"));
    Ok(())
}

fn init_db(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.database);
    if path.exists() {
        fs::remove_file(path)?;
        print_ok(
            &tr_with("db_reset_done", &[("path", &path.to_string_lossy())]),
            true,
        );
    } else {
        print_ok(
            &tr_with("db_init_creating", &[("path", &path.to_string_lossy())]),
            true,
        );
    }

    let conn = Connection::open(path)?;
    create_schema(&conn)?;
    print_ok(
        &tr_with("db_init_done", &[("path", &path.to_string_lossy())]),
        true,
    );
    Ok(())
}

/// Crea le tabelle di base nel database appena inizializzato.
///
/// Al momento include:
/// - books → archivio principale dei libri
/// - log → tabella di log operazioni
fn create_schema(conn: &Connection) -> Result<(), Box<dyn Error>> {
    // Tabella principale "books"
    conn.execute_batch(
        "
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
            added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );

        CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            operation TEXT NOT NULL,
            target TEXT DEFAULT '',
            message TEXT NOT NULL
        );
        ",
    )?;

    print_ok(&tr("db.schema.created"), true);
    Ok(())
}

fn copy_db(config: &AppConfig, dest: &str) -> Result<(), Box<dyn Error>> {
    let src = Path::new(&config.database);
    if !src.exists() {
        print_err(&tr("db_no_source"));
        return Ok(());
    }

    fs::copy(src, dest)?;
    print_ok(
        &tr_with(
            "db_copy_done",
            &[("source", &src.to_string_lossy()), ("destination", dest)],
        ),
        true,
    );
    Ok(())
}
