// =====================================================
// Librius - Utilities module
// -----------------------------------------------------
// Contiene funzioni di supporto generali e costanti
// grafiche per output CLI.
// =====================================================

use chrono::Local;
use colored::*;
use rusqlite::{Connection, Result};
use std::sync::OnceLock;

static VERBOSE: OnceLock<bool> = OnceLock::new();

/// Enables verbose (debug) mode.
pub fn set_verbose(enabled: bool) {
    let _ = VERBOSE.set(enabled);
}

/// Returns true if verbose mode is active.
pub fn is_verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}

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

/// Modulo con le icone standard utilizzate nell'applicazione.
///
/// Le emoji sono seguite da uno spazio per evitare problemi
/// di spaziatura nei terminali (⚠️, ✅, ❌, 📘, ecc.).
pub mod icons {
    pub const OK: &str = "✅ ";
    pub const ERR: &str = "❌ ";
    pub const WARN: &str = "⚠️  ";
    pub const INFO: &str = "📘  ";
}

/// Stampa un messaggio formattato come "OK"
pub fn print_ok(msg: &str, verbose: bool) {
    if !verbose {
        return;
    }
    println!("{}{}", icons::OK, msg.green().bold());
}

/// Stampa un messaggio di errore formattato
pub fn print_err(msg: &str) {
    eprintln!("{}{}", icons::ERR, msg.red().bold());
}

/// Stampa un messaggio di avviso formattato
pub fn print_warn(msg: &str) {
    println!("{}{}", icons::WARN, msg.yellow().bold());
}

/// Stampa un messaggio informativo
pub fn print_info(msg: &str, verbose: bool) {
    if !verbose {
        return;
    }
    println!("{}{}", icons::INFO, msg.blue().bold());
}
