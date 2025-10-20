use crate::book::{Book, BookFull, BookShort};
use crate::i18n::tr;
use crate::isbn::normalize_isbn;
use crate::utils::{build_table, build_vertical_table, print_err};
use chrono::{DateTime, NaiveDateTime, Utc};
use rusqlite::types::ToSql;
use rusqlite::{Connection, Row};
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

// Helper to map a rusqlite::Row into a Book instance.
fn row_to_book(row: &Row) -> rusqlite::Result<Book> {
    let added_at_str: Option<String> = row.get("added_at")?;
    let parsed_added_at = added_at_str.as_deref().and_then(parse_added_at);

    // Recupera ISBN dal DB (senza trattini)
    let isbn_plain: String = row.get("isbn")?;

    // Prova a formattarlo con trattini (se valido)
    let isbn_formatted = match normalize_isbn(&isbn_plain, false) {
        Ok(formatted) => formatted,
        Err(e) => {
            print_err(&e.to_string());
            isbn_plain.clone()
        } // fallback in caso di ISBN non valido
    };

    Ok(Book {
        id: row.get("id")?,
        title: row.get("title")?,
        author: row.get("author")?,
        editor: row.get("editor")?,
        year: row.get("year")?,
        isbn: isbn_formatted,
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
}

/// Handle the `list` subcommand.
///
/// Lists all books from the database using localized tabular output.
/// Supports the `--short` flag for compact view.
pub fn handle_list(
    conn: &Connection,
    _short: bool,
    id: Option<i32>,
    _details: bool,
) -> Result<(), Box<dyn Error>> {
    // If user asked for details without specifying an id, show a localized
    // error message and do not display the list.
    if _details && id.is_none() {
        println!();
        print_err(&tr("list.error.details_requires_id"));
        return Ok(());
    }

    // Build base query and optionally filter by id if provided
    let base_query = "SELECT id, title, author, editor, year, isbn, language, pages, genre, summary, room, shelf, row, position, added_at FROM books";
    let query = if id.is_some() {
        format!("{} WHERE id = ?1 ORDER BY id;", base_query)
    } else {
        format!("{} ORDER BY id;", base_query)
    };

    let mut stmt = conn.prepare(&query)?;
    let mut books: Vec<Book> = Vec::new();

    // Build owned params: store boxed ToSql trait objects so ownership is
    // guaranteed, and we can build a slice of `&dyn ToSql` for the query.
    let mut params_owned: Vec<Box<dyn ToSql>> = Vec::new();
    if let Some(v) = id {
        params_owned.push(Box::new(v));
    }
    // Create a slice of references to pass to rusqlite
    let params_refs: Vec<&dyn ToSql> = params_owned.iter().map(|b| b.as_ref()).collect();

    let mapped = stmt.query_map(params_refs.as_slice(), row_to_book)?;
    for r in mapped {
        books.push(r?);
    }

    if books.is_empty() {
        if let Some(book_id) = id {
            println!("⚠️  No book found with ID {book_id}");
        } else {
            println!("\n📚  {}", tr("list.no_books_found"));
        }
        return Ok(());
    }

    // If an ID was requested, show detailed vertical view for that specific record.
    if id.is_some() {
        let book = &books[0];
        println!("\n📖  {} {:?}\n", tr("list.book_details_for_id"), book.id);
        build_vertical_table(book);
    } else {
        // Otherwise show the list (short or full)
        println!("\n{}\n", tr("app.library.info"));

        let table = if _short {
            build_table(books.iter().map(BookShort))
        } else {
            build_table(books.iter().map(BookFull))
        };

        println!("{}", table);
    }

    Ok(())
}
