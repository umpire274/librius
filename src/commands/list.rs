use crate::models::book::Book;
use crate::models::display::{BookFull, BookShort};
use crate::i18n::tr;
use crate::isbn::normalize_isbn;
use crate::utils::{build_table, build_vertical_table, print_err};
use rusqlite::types::ToSql;
use rusqlite::{Connection, Row};
use std::error::Error;

/// Maps a rusqlite::Row into a Book, applying ISBN hyphen formatting on top of
/// the base `Book::from_row()` constructor.
fn row_to_book(row: &Row) -> rusqlite::Result<Book> {
    let mut book = Book::from_row(row)?;
    // Format ISBN with hyphens for display; fall back to plain on error.
    book.isbn = match normalize_isbn(&book.isbn, false) {
        Ok(formatted) => formatted,
        Err(e) => {
            print_err(&e.to_string());
            book.isbn
        }
    };
    Ok(book)
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
    compact: bool,
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
        build_vertical_table(book, compact);
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
