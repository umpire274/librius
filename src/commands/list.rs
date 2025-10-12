//! `list` command implementation.
//!
//! This module contains the handler used by the CLI to list all books stored
//! in the database. The function performs a simple SELECT query and prints
//! a human-readable list to standard output using colored formatting.

use crate::models::Book;
use chrono::{DateTime, NaiveDateTime, Utc};
use colored::*;
use rusqlite::Connection;
use std::error::Error;

fn parse_added_at(s: &str) -> Option<DateTime<Utc>> {
    // Try RFC3339 first
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    // Try SQLite default format: "YYYY-MM-DD HH:MM:SS"
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    // Try with fractional seconds
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }

    None
}

/// Handle the `list` subcommand.
///
/// The handler queries the `books` table and prints each record to stdout in
/// a compact, colored format. Errors from SQL preparation or iteration are
/// returned to the caller so the binary can decide how to report them.
pub fn handle_list(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn
        .prepare("SELECT id, title, author, editor, year, isbn, language, pages, genre, summary, added_at FROM books ORDER BY id;")?;
    let rows = stmt.query_map([], |row| {
        // Read added_at as an optional string, then parse to DateTime<Utc>
        let added_at_str: Option<String> = row.get("added_at")?;
        let parsed_added_at = match added_at_str {
            Some(ref s) => parse_added_at(s),
            None => None,
        };

        Ok(Book {
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
            added_at: parsed_added_at,
        })
    })?;

    println!("\n{}", "📚 Your Library".bold().green());
    for b in rows {
        let b = b?;
        // Format added_at as YYYY-MM-DD when present
        let added_date = b
            .added_at
            .as_ref()
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{}. {} ({:?}) [{}] [{}]",
            b.id.to_string().blue(),
            b.title.bold(),
            b.author.unwrap_or_else(|| "Unknown".into()),
            b.year.map_or("-".to_string(), |y| y.to_string()),
            added_date
        );
    }

    Ok(())
}
