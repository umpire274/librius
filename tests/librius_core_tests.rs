use librius::commands::handle_list;
use rusqlite::Connection;
use std::error::Error;

#[test]
fn exercise_list_handler() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            editor TEXT NOT NULL,
            year INTEGER NOT NULL,
            isbn TEXT NOT NULL,
            language TEXT,
            pages INTEGER,
            genre TEXT,
            summary TEXT,
            room TEXT,
            shelf TEXT,
            row TEXT,
            position TEXT,
            added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    conn.execute(
        "INSERT INTO books (title, author, editor, year, isbn, added_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        [
            "Test Book",
            "Author",
            "Editor",
            "2025",
            "978-88823145698",
            "2020-01-01 12:00:00",
        ],
    )?;

    handle_list(&conn, false, None, false)?;
    Ok(())
}

#[test]
fn exercise_list_handler_short() -> Result<(), Box<dyn Error>> {
    let conn = Connection::open_in_memory()?;
    conn.execute(
        "CREATE TABLE books (
            id INTEGER PRIMARY KEY,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            editor TEXT NOT NULL,
            year INTEGER NOT NULL,
            isbn TEXT NOT NULL,
            language TEXT,
            pages INTEGER,
            genre TEXT,
            summary TEXT,
            room TEXT,
            shelf TEXT,
            row TEXT,
            position TEXT,
            added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;

    conn.execute(
        "INSERT INTO books (title, author, editor, year, isbn, added_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6);",
        [
            "Short Test",
            "Author",
            "Editor",
            "2022",
            "978-0000000000",
            "2020-01-01 12:00:00",
        ],
    )?;

    handle_list(&conn, true, None, false)?;
    Ok(())
}
