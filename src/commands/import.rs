use crate::i18n::tr_with;
use crate::utils::{is_verbose, print_info, print_ok, write_log};
use rusqlite::{Connection, Transaction};
use serde::Deserialize;
use std::fs::File;
use std::io;

use csv::ReaderBuilder;

/// Struttura dati comune per importazione (CSV/JSON)
#[derive(Debug, Deserialize)]
pub struct BookRecord {
    pub title: String,
    pub author: String,
    pub editor: String,
    pub year: i64,
    pub isbn: String,
    pub language: Option<String>,
    pub pages: Option<i64>,
    pub genre: Option<String>,
    pub summary: Option<String>,
    pub room: Option<String>,
    pub shelf: Option<String>,
    pub row: Option<String>,
    pub position: Option<String>,
}

/// Funzione helper per convertire errori in `io::Error`
fn to_io<E: std::fmt::Display>(err: E) -> io::Error {
    io::Error::other(err.to_string())
}

/// 🧩 Helper: inserisce un record nella tabella `books`, gestendo i duplicati via OR IGNORE.
/// Ritorna `true` se il record è stato inserito, `false` se saltato (duplicato).
fn insert_book_record(tx: &Transaction, rec: &BookRecord) -> io::Result<bool> {
    let affected = tx
        .execute(
            "INSERT OR IGNORE INTO books (title, author, editor, year, isbn, language, pages, genre, summary, room, shelf, row, position)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &rec.title,
                &rec.author,
                &rec.editor,
                &rec.year,
                &rec.isbn,
                &rec.language,
                &rec.pages,
                &rec.genre,
                &rec.summary,
                &rec.room,
                &rec.shelf,
                &rec.row,
                &rec.position,
            ),
        )
        .map_err(to_io)?;

    Ok(affected > 0)
}

/// 🧩 Importa dati da file CSV (usa `csv` + `serde`)
pub fn handle_import_csv(conn: &mut Connection, file_path: &str) -> io::Result<()> {
    let mut reader = ReaderBuilder::new()
        .delimiter(b';')
        .from_path(file_path)
        .map_err(to_io)?;

    let tx = conn.transaction().map_err(to_io)?;
    let mut inserted = 0;

    for result in reader.deserialize() {
        let rec: BookRecord = result.map_err(to_io)?;
        let inserted_ok = insert_book_record(&tx, &rec)?;
        if inserted_ok {
            inserted += 1;
        } else if is_verbose() {
            print_info(
                &tr_with("import.db.skipped_isbn", &[("isbn", &rec.isbn)]),
                is_verbose(),
            );
        }
    }

    tx.commit().map_err(to_io)?;
    print_ok(
        &tr_with("import.csv.ok", &[("count", &inserted.to_string())]),
        true,
    );
    write_log(
        conn,
        "IMPORT_CSV_COMPLETED",
        "DB",
        &tr_with("log.import.completed", &[("count", &inserted.to_string())]),
    )
    .map_err(to_io)?;
    Ok(())
}

/// 🧩 Importa dati da file JSON (usa `serde_json`)
pub fn handle_import_json(conn: &mut Connection, file_path: &str) -> io::Result<()> {
    let file = File::open(file_path).map_err(to_io)?;
    let data: Vec<BookRecord> = serde_json::from_reader(file)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

    let tx = conn.transaction().map_err(to_io)?;
    let mut inserted = 0;

    for rec in data {
        let inserted_ok = insert_book_record(&tx, &rec)?;
        if inserted_ok {
            inserted += 1;
        } else if is_verbose() {
            print_info(
                &tr_with("import.db.skipped_isbn", &[("isbn", &rec.isbn)]),
                is_verbose(),
            );
        }
    }

    tx.commit().map_err(to_io)?;
    print_ok(
        &tr_with("import.json.ok", &[("count", &inserted.to_string())]),
        true,
    );
    write_log(
        conn,
        "IMPORT_JSON_COMPLETED",
        "DB",
        &tr_with("log.import.completed", &[("count", &inserted.to_string())]),
    )
    .map_err(to_io)?;

    Ok(())
}
