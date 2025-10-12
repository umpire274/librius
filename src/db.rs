//! Database initialization utilities for Librius.
//!
//! This module provides a small helper to initialize (or open) the SQLite
//! database used by the application. The `init_db` function ensures the
//! required `books` table exists and returns an active `rusqlite::Connection`.
//!
//! The schema is intentionally simple and stores basic metadata for each
//! book (title, author, year, isbn and a timestamp when the record was
//! added).

use crate::config::AppConfig;
use rusqlite::{Connection, Result};

/// Initialize or open the SQLite database and ensure required tables exist.
///
/// This function will open the SQLite file pointed by `cfg.db_path` and
/// execute a `CREATE TABLE IF NOT EXISTS` statement to ensure the `books`
/// table is available. On success it returns an active `Connection` that
/// callers can use for queries and commands.
///
/// # Example
/// ```no_run
/// use librius::config::AppConfig;
/// // load or create config and then initialize the database
/// let cfg: AppConfig = librius::config::load_or_init().unwrap();
/// let conn = librius::db::init_db(&cfg).unwrap();
/// ```
///
/// # Schema (summary)
/// - `id`: INTEGER PRIMARY KEY AUTOINCREMENT
/// - `title`: TEXT NOT NULL
/// - `author`: TEXT
/// - `editor`: TEXT
/// - `year`: INTEGER
/// - `isbn`: TEXT
/// - `language`: TEXT
/// - `pages`: INTEGER
/// - `genre`: TEXT
/// - `summary`: TEXT
/// - `added_at`: TIMESTAMP DEFAULT CURRENT_TIMESTAMP
///
/// # Errors
/// Returns a `rusqlite::Error` if the database file cannot be opened or the
/// initialization SQL fails.
pub fn init_db(cfg: &AppConfig) -> Result<Connection> {
    let conn = Connection::open(&cfg.db_path)?;
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT,
			editor TEXT,
            year INTEGER,
            isbn TEXT,
			language TEXT,
			pages INTEGER,
			genre TEXT,
			summary TEXT,
			added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
    )?;
    Ok(conn)
}
