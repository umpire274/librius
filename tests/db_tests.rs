mod common;
use common::setup_temp_db;

#[test]
fn test_temporary_db_with_full_schema() {
    let conn = setup_temp_db("integration_schema");

    // Verifica che lo schema di produzione sia presente
    let count: i64 = conn
        .query_row(
            "SELECT COUNT(*) FROM pragma_table_info('books') WHERE name = 'isbn';",
            [],
            |r| r.get(0),
        )
        .unwrap();

    assert_eq!(count, 1, "La colonna 'isbn' non è presente nello schema");
}

#[test]
fn test_insert_and_read_book_full_schema() {
    let conn = setup_temp_db("insert_full");

    conn.execute(
        "INSERT INTO books (title, author, editor, year, isbn, language, pages, genre, summary, room, shelf, row, position, added_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, datetime('now'))",
        (
            "Il Nome della Rosa",
            "Umberto Eco",
            "Bompiani",
            1980,
            "9788845254283",
            "it",
            512,
            "Romanzo storico",
            "Un romanzo ambientato in un monastero medievale...",
            "Studio",
            "A",
            "1",
            "3",
        ),
    ).unwrap();

    let mut stmt = conn
        .prepare("SELECT title, author, genre FROM books WHERE author = 'Umberto Eco'")
        .unwrap();
    let row = stmt
        .query_row([], |r| {
            Ok((
                r.get::<_, String>(0)?,
                r.get::<_, String>(1)?,
                r.get::<_, String>(2)?,
            ))
        })
        .unwrap();

    assert_eq!(row.0, "Il Nome della Rosa");
    assert_eq!(row.1, "Umberto Eco");
    assert_eq!(row.2, "Romanzo storico");
}

#[test]
fn test_from_row_parses_sqlite_current_timestamp() {
    // Regression test for: Book::from_row must not fail when added_at is stored
    // as SQLite CURRENT_TIMESTAMP format ("YYYY-MM-DD HH:MM:SS") instead of RFC3339.
    use librius::models::Book;

    let conn = setup_temp_db("timestamp_regression");

    // Insert with SQLite CURRENT_TIMESTAMP (produces "YYYY-MM-DD HH:MM:SS")
    conn.execute(
        "INSERT INTO books (title, author, editor, year, isbn, added_at)
         VALUES ('Dune', 'Frank Herbert', 'Chilton', 1965, '9780441013593', datetime('now'))",
        [],
    )
    .unwrap();

    // Book::from_row must succeed and either parse the date or degrade to None — never error.
    let mut stmt = conn
        .prepare(
            "SELECT id, title, author, editor, year, isbn, language, pages,
                    genre, summary, room, shelf, row, position, added_at
             FROM books WHERE isbn = '9780441013593'",
        )
        .unwrap();

    let book = stmt
        .query_row([], Book::from_row)
        .expect("from_row must not fail on SQLite CURRENT_TIMESTAMP format");

    assert_eq!(book.title, "Dune");
    // added_at should be Some (parsed successfully), not None and not an error
    assert!(
        book.added_at.is_some(),
        "added_at should be parsed from SQLite timestamp format"
    );
}

#[test]
fn test_from_row_parses_rfc3339_timestamp() {
    use librius::models::Book;

    let conn = setup_temp_db("timestamp_rfc3339");

    // Insert with explicit RFC3339 timestamp (written by chrono)
    conn.execute(
        "INSERT INTO books (title, author, editor, year, isbn, added_at)
         VALUES ('Foundation', 'Isaac Asimov', 'Gnome Press', 1951, '9780553293357',
                 '2025-10-13T21:32:07+00:00')",
        [],
    )
    .unwrap();

    let mut stmt = conn
        .prepare(
            "SELECT id, title, author, editor, year, isbn, language, pages,
                    genre, summary, room, shelf, row, position, added_at
             FROM books WHERE isbn = '9780553293357'",
        )
        .unwrap();

    let book = stmt
        .query_row([], Book::from_row)
        .expect("from_row must not fail on RFC3339 timestamp");

    assert_eq!(book.title, "Foundation");
    assert!(book.added_at.is_some(), "RFC3339 timestamp must be parsed");
}
