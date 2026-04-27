// =====================================================
// Librius - models/book.rs
// -----------------------------------------------------
// Modello dati puro per un libro.
// Nessuna dipendenza da i18n o tabled: solo dati,
// serializzazione e costruzione da riga SQLite.
// =====================================================

use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Row;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub editor: String,
    pub year: i32,
    pub isbn: String,
    pub language: Option<String>,
    pub pages: Option<i32>,
    pub genre: Option<String>,
    pub summary: Option<String>,
    pub room: Option<String>,
    pub shelf: Option<String>,
    pub row: Option<String>,
    pub position: Option<String>,
    pub added_at: Option<DateTime<Utc>>,
}

impl Book {
    pub fn from_row(row: &Row) -> rusqlite::Result<Self> {
        // `added_at` is stored by SQLite as either:
        //   - RFC3339 / ISO 8601 with timezone: "2025-10-13T21:32:07+02:00"
        //   - SQLite CURRENT_TIMESTAMP (no timezone): "2025-10-13 21:32:07"
        //   - SQLite CURRENT_TIMESTAMP with sub-seconds: "2025-10-13 21:32:07.123"
        // rusqlite's built-in DateTime<Utc> conversion only accepts RFC3339,
        // so we read as String and parse manually, degrading to None on failure.
        let added_at_str: Option<String> = row.get("added_at")?;
        let added_at = added_at_str.as_deref().and_then(parse_sqlite_datetime);

        Ok(Self {
            id: row.get("id")?,
            title: row.get("title")?,
            author: row.get("author")?,
            editor: row.get("editor")?,
            year: row.get("year")?,
            isbn: row.get("isbn")?,
            language: row.get("language")?,
            pages: row.get("pages")?,
            genre: row.get("genre")?,
            summary: row.get("summary")?,
            room: row.get("room")?,
            shelf: row.get("shelf")?,
            row: row.get("row")?,
            position: row.get("position")?,
            added_at,
        })
    }
}

/// Parse a SQLite timestamp string into `DateTime<Utc>`.
///
/// Tries the following formats in order, returning `None` if none match:
/// 1. RFC3339 / ISO 8601 with timezone  (`2025-10-13T21:32:07+02:00`)
/// 2. SQLite `CURRENT_TIMESTAMP`         (`2025-10-13 21:32:07`)
/// 3. SQLite with sub-second precision   (`2025-10-13 21:32:07.123`)
fn parse_sqlite_datetime(s: &str) -> Option<DateTime<Utc>> {
    // 1. RFC3339 (written by chrono or explicit INSERT)
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    // 2. SQLite CURRENT_TIMESTAMP: "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    // 3. SQLite CURRENT_TIMESTAMP with fractional seconds
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    None
}
