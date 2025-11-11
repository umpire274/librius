use crate::db::search_books;
use crate::i18n::tr;
use crate::models::{Book, BookFull, BookShort};
use crate::print_warn;
use crate::utils::table::build_table;
use rusqlite::Connection;
use std::error::Error;

pub fn handle_search(conn: &Connection, query: &str, short: bool) -> Result<(), Box<dyn Error>> {
    let results: Vec<Book> = search_books(conn, query)?;

    if results.is_empty() {
        print_warn(&tr("search.no_results"));
        return Ok(());
    }

    if short {
        let wrapped: Vec<BookShort> = results.iter().map(BookShort).collect();
        println!("{}", build_table(&wrapped));
    } else {
        let wrapped: Vec<BookFull> = results.iter().map(BookFull).collect();
        println!("{}", build_table(&wrapped));
    }

    Ok(())
}
