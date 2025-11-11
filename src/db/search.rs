use crate::models::Book;
use rusqlite::{Connection, Result};

pub fn search_books(conn: &Connection, query: &str) -> Result<Vec<Book>> {
    let like = format!("%{}%", query);

    let mut stmt = conn.prepare(
        r#"
        SELECT id, title, author, editor, year, isbn, language, pages,
               genre, summary, room, shelf, row, position, added_at
        FROM books
        WHERE title   LIKE ?1
           OR author  LIKE ?1
           OR editor  LIKE ?1
           OR genre   LIKE ?1
           OR language LIKE ?1
        ORDER BY title COLLATE NOCASE ASC;
    "#,
    )?;

    let rows = stmt.query_map([like], Book::from_row)?;

    let mut out = Vec::new();
    for r in rows {
        out.push(r?);
    }
    Ok(out)
}
