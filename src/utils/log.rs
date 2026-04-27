// =====================================================
// Librius - utils/log.rs
// -----------------------------------------------------
// Funzioni per la scrittura del log sul database SQLite
// e per la formattazione della data/ora corrente.
// =====================================================

use chrono::Local;
use rusqlite::{Connection, Result};

/// Returns the current local date-time in full ISO 8601 format.
///
/// Example output:
/// ```text
/// 2025-10-13T21:32:07+02:00
/// ```
pub fn now_str() -> String {
    Local::now().format("%+").to_string()
}

/// Writes an entry into the 'log' table.
/// If the table does not exist, it will be created automatically.
///
/// # Arguments
/// * `conn` - Active SQLite connection
/// * `operation` - Type of action (e.g., "PATCH_001", "ADD_BOOK")
/// * `target` - Logical target (e.g., "DB", "CONFIG", "BOOKS")
/// * `message` - Description of the operation
///
/// # Example
/// ```
/// use librius::utils::write_log;
/// use rusqlite::Connection;
/// // create an in-memory connection for the example
/// let conn = Connection::open_in_memory().unwrap();
/// write_log(&conn, "ADD_BOOK", "BOOKS", "Inserted 'Dune'").unwrap();
/// ```
pub fn write_log(conn: &Connection, operation: &str, target: &str, message: &str) -> Result<()> {
    // Ensure log table exists
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

    let now = now_str();
    conn.execute(
        "INSERT INTO log (date, operation, target, message) VALUES (?1, ?2, ?3, ?4)",
        (&now, &operation, &target, &message),
    )?;

    Ok(())
}

