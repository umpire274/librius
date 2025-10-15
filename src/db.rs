//! Database initialization utilities for Librius.
//!
//! This module provides a small helper to initialize (or open) the SQLite
//! database used by the application. The `init_db` function ensures the
//! required `books` table exists and returns an active `rusqlite::Connection`.
//!
//! The schema is intentionally simple and stores basic metadata for each
//! book (title, author, year, isbn and a timestamp when the record was
//! added).

pub mod migrate;

use crate::config::AppConfig;
use crate::i18n::{tr, tr_with};
use crate::utils::{is_verbose, print_err, print_info, print_ok, write_log};
use rusqlite::{Connection, Result};
use std::path::Path;

/// Opens or initializes the SQLite database.
///
/// - If the database file does not exist, it creates a new one and its tables.
/// - If it exists, it just opens it and applies pending migrations.
/// - Each operation is logged in the `log` table.
///
pub fn start_db(config: &AppConfig) -> Result<Connection> {
    let db_path = Path::new(&config.database);
    let db_exists = db_path.exists();

    if db_exists {
        print_info(
            &tr_with(
                "db.open.existing",
                &[("db_path", &db_path.display().to_string())],
            ),
            is_verbose(),
        );
    } else {
        print_info(
            &tr_with(
                "db.create.new_db",
                &[("db_path", &db_path.display().to_string())],
            ),
            is_verbose(),
        );
    }

    // Try to open connection
    let conn = Connection::open(db_path)?;

    // Create log table immediately (for logging)
    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            operation TEXT NOT NULL,
            target TEXT DEFAULT '',
            message TEXT NOT NULL
        );",
        [],
    )?;

    // Log opening
    let action = if db_exists { "DB_OPENED" } else { "DB_CREATED" };
    let msg = if db_exists {
        &tr_with(
            "log.db.open",
            &[("db_path", &db_path.display().to_string())],
        )
    } else {
        &tr_with(
            "log.db.create",
            &[("db_path", &db_path.display().to_string())],
        )
    };
    let _ = write_log(&conn, action, "DB", &msg);

    // Initialize structure if missing
    if !db_exists {
        print_info(&tr("db.schema.initializing"), is_verbose());
        if let Err(e) = ensure_schema(&conn) {
            print_err(&tr_with(
                "db.schema.init_failed",
                &[("error", &e.to_string())],
            ));
            let _ = write_log(&conn, "DB_INIT_FAIL", "DB", &e.to_string());
            return Err(e);
        }
        print_ok(&tr("db.schema.created"), is_verbose());
        let _ = write_log(&conn, "DB_INIT_OK", "DB", &tr("log.db.schema.init"));
    }

    // Apply migrations
    match migrate::run_migrations(&conn) {
        Err(e) => {
            print_err(&tr_with("db.migrate.failed", &[("error", &e.to_string())]));
            let _ = write_log(&conn, "DB_MIGRATION_FAIL", "DB", &e.to_string());
        }
        Ok(result) => match result {
            migrate::MigrationResult::Applied(patches) => {
                print_ok(&tr("db.migrate.applied"), is_verbose());
                let msg = &tr_with("log.db.patch_applied", &[("patchCreata nuova configurazione in", &patches.join(", "))]);
                let _ = write_log(&conn, "DB_MIGRATION_OK", "DB", &msg);
            }
            migrate::MigrationResult::None => {
                print_ok(&tr("db.schema.already_update"), is_verbose());
            }
        },
    }

    Ok(conn)
}

/// Public compatibility function expected by docs and external callers.
/// Previous API used `db::init_db(&cfg)` returning a `Connection`.
pub fn init_db(config: &AppConfig) -> Result<Connection> {
    start_db(config)
}

/// Ensure required tables exist in an opened connection.
pub fn ensure_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS books (
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
        );",
    )?;
    Ok(())
}
