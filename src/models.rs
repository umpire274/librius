//! Domain models used by Librius.
//!
//! This module contains simple data structures that represent the persistent
//! entities stored in the SQLite database. Models are `serde` serializable
//! so they can be easily printed, logged, or converted to JSON if needed.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a single book record in the library.
///
/// Fields correspond to the columns of the `books` table in the SQLite
/// database. Many fields are optional to reflect incomplete metadata for
/// some records. The cataloging/location fields (`room`, `shelf`, `row` and
/// `position`) describe where the physical item is stored and are optional
/// because not all records may have precise shelving information.
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    /// Primary key identifier for the book.
    pub id: i64,
    /// Title of the book (required).
    pub title: String,
    /// Author name, when available.
    pub author: String,
    /// Editor / publisher, when available.
    pub editor: String,
    /// Publication year, when available.
    pub year: i32,
    /// ISBN code, when available.
    pub isbn: String,
    /// Language of the book (e.g., "en", "it"), when available.
    pub language: Option<String>,
    /// Number of pages, when available.
    pub pages: Option<i32>,
    /// Genre or category, when available.
    pub genre: Option<String>,
    /// Short textual summary or notes about the book.
    pub summary: Option<String>,
    /// Room where the item is stored (e.g. "Main", "Annex").
    pub room: Option<String>,
    /// Shelf identifier inside the room, when available.
    pub shelf: Option<String>,
    /// Row identifier on the shelf, when available.
    pub row: Option<String>,
    /// Position within the row/shelf, when available.
    pub position: Option<String>,
    /// Timestamp when the record was added. Represented as `DateTime<Utc>`.
    ///
    /// This field maps the SQLite `CURRENT_TIMESTAMP` value (stored as
    /// text like `YYYY-MM-DD HH:MM:SS`) into a `chrono::DateTime<Utc>`.
    pub added_at: Option<DateTime<Utc>>,
}
