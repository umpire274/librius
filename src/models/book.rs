// =====================================================
// Librius - models/book.rs
// -----------------------------------------------------
// Modello dati puro per un libro.
// Nessuna dipendenza da i18n o tabled: solo dati,
// serializzazione e costruzione da riga SQLite.
// =====================================================

use chrono::{DateTime, Utc};
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
            added_at: row.get("added_at")?,
        })
    }
}
