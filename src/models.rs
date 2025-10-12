//! Domain models used by Librius.
//!
//! This module contains simple data structures that represent the persistent
//! entities stored in the SQLite database. Models are `serde` serializable
//! so they can be easily printed, logged, or converted to JSON if needed.

use serde::{Deserialize, Serialize};

/// Represents a single book record in the library.
///
/// Fields correspond to the columns of the `books` table in the SQLite
/// database. Many fields are optional to reflect incomplete metadata for
/// some records.
#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    /// Primary key identifier for the book.
    pub id: i64,
    /// Title of the book (required).
    pub title: String,
    /// Author name, when available.
    pub author: Option<String>,
    /// Editor / publisher, when available.
    pub editor: Option<String>,
    /// Publication year, when available.
    pub year: Option<i32>,
    /// ISBN code, when available.
    pub isbn: Option<String>,
    /// Language of the book (e.g., "en", "it"), when available.
    pub language: Option<String>,
    /// Number of pages, when available.
    pub pages: Option<i32>,
    /// Genre or category, when available.
    pub genre: Option<String>,
    /// Short textual summary or notes about the book.
    pub summary: Option<String>,
    /// Timestamp when the record was added. Stored as text from SQLite's
    /// `CURRENT_TIMESTAMP` default; represented here as an optional string.
    pub added_at: Option<String>,
}
