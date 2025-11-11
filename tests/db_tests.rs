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
