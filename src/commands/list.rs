use crate::book::{Book, BookFull, BookShort};
use crate::i18n::tr;
use crate::utils::build_table;
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::Connection;
use std::error::Error;

fn parse_added_at(s: &str) -> Option<DateTime<Utc>> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
        return Some(dt.with_timezone(&Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(DateTime::from_naive_utc_and_offset(naive, Utc));
    }
    None
}

/// Handle the `list` subcommand.
///
/// Lists all books from the database using localized tabular output.
/// Supports the `--short` flag for compact view.
pub fn handle_list(conn: &Connection, short: bool) -> Result<(), Box<dyn Error>> {
    let mut stmt = conn.prepare(
        "SELECT id, title, author, editor, year, isbn, language, pages, genre, summary, \
         room, shelf, row, position, added_at FROM books ORDER BY id;",
    )?;

    let rows = stmt.query_map([], |row| {
        let added_at_str: Option<String> = row.get("added_at")?;
        let parsed_added_at = added_at_str.as_ref().and_then(|s| parse_added_at(s));

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
            room: row.get("room")?,
            shelf: row.get("shelf")?,
            row: row.get("row")?,
            position: row.get("position")?,
            added_at: parsed_added_at,
        })
    })?;

    let books: Vec<Book> = rows.filter_map(|r| r.ok()).collect();

    if books.is_empty() {
        println!("\n📚  {}", tr("list.no_books_found"));
        return Ok(());
    }

    println!("\n{}\n", tr("app.library.info"));

    let table = if short {
        build_table(books.iter().map(BookShort))
    } else {
        build_table(books.iter().map(BookFull))
    };

    println!("{table}");
    Ok(())
}
