//! Librius — a small library manager core crate
//!
//! This crate contains the core functionality used by the `librius` binary.
//! It is intentionally lightweight and exposes configuration helpers, the
//! database initialization routine and the primary domain model (`Book`).
//!
//! The binary (`src/main.rs`) uses this crate to perform startup and to
//! dispatch command handlers. Including a `lib.rs` makes this project
//! suitable for documentation generation on platforms such as docs.rs.
//!
//! Example
//!
//! ```no_run
//! use librius::config::AppConfig;
//! use librius::db;
//!
//! // load or create config, then init database
//! let cfg: AppConfig = librius::config::load_or_init().unwrap();
//! let conn = db::init_db(&cfg).unwrap();
//! ```

pub mod cli;
pub mod commands;
pub mod config;
pub mod db;
pub mod i18n;
pub mod models;
pub mod utils;

pub use cli::build_cli;
pub use commands::*;
pub use config::*;
pub use db::*;
pub use i18n::*;
pub use models::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::error::Error;

    #[test]
    fn exercise_list_handler() -> Result<(), Box<dyn Error>> {
        // Crea un DB in-memory e la tabella `books` con le colonne usate dal codice
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE books (
                id INTEGER PRIMARY KEY,
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
            );",
            [],
        )?;

        conn.execute(
            "INSERT INTO books (title, author, editor, year, isbn, added_at) VALUES (?1, ?2, ?3,?4, ?5, ?6);",
            ["Test Book", "Author", "Editor", "2025", "978-88823145698", "2020-01-01 12:00:00"],
        )?;

        // Chiama la funzione handle_list per esercitare la logica di mapping e formattazione
        // default view in tests: non-short (full)
        handle_list(&conn, false, None, false)?;

        Ok(())
    }

    #[test]
    fn exercise_list_handler_short() -> Result<(), Box<dyn Error>> {
        // stessa preparazione DB, ma verifichiamo la vista corta (short=true)
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE books (
                id INTEGER PRIMARY KEY,
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
            );",
            [],
        )?;

        conn.execute(
            "INSERT INTO books (title, author, editor, year, isbn, added_at) VALUES (?1, ?2, ?3,?4, ?5, ?6);",
            ["Short Test", "Author", "Editor", "2022", "978-0000000000", "2020-01-01 12:00:00"],
        )?;

        // Chiama la funzione handle_list per verificare la vista corta (non deve panicare)
        handle_list(&conn, true, None, false)?;

        Ok(())
    }
}
