//! `list` command implementation.
//!
//! This module contains the handler used by the CLI to list all books stored
//! in the database. The function performs a simple SELECT query and prints
//! a human-readable list to standard output using colored formatting.

use crate::models::Book;
use colored::*;
use rusqlite::Connection;
use std::error::Error;

/// Handle the `list` subcommand.
///
/// The handler queries the `books` table and prints each record to stdout in
/// a compact, colored format. Errors from SQL preparation or iteration are
/// returned to the caller so the binary can decide how to report them.
pub fn handle_list(conn: &Connection) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn
        .prepare("SELECT id, title, author, editor, year, isbn, language, pages, genre, summary, added_at FROM books ORDER BY id;")?;
    let rows = stmt
        .query_map([], |row| {
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
                added_at: row.get("added_at")?,
            })
        })?;

    println!("\n{}", "📚 Your Library".bold().green());
    for b in rows {
        let b = b?;
        println!(
            "{}. {} ({:?}) [{}]",
            b.id.to_string().blue(),
            b.title.bold(),
            b.author.unwrap_or_else(|| "Unknown".into()),
            b.year.map_or("-".to_string(), |y| y.to_string())
        );
    }

    Ok(())
}
