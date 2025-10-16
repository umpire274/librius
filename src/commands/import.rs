use crate::i18n::tr_with;
use crate::utils::{is_verbose, print_ok};
use crate::{Book, print_err};
use csv::ReaderBuilder;
use rusqlite::Connection;
use std::io::BufReader;

/// 🧩 Importa dati da file CSV (usa `csv` + `serde`)
pub fn handle_import_csv(
    conn: &mut Connection,
    file: &str,
    delimiter: char,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_display = file.to_string();

    // ✅ Attempt to open the file
    let file_handle = crate::utils::open_import_file(file)?;

    // ✅ Build CSV reader
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter as u8)
        .from_reader(file_handle);

    let mut imported = 0;
    let mut failed = 0;

    // ✅ Read and process each record
    for (index, record) in reader.deserialize::<Book>().enumerate() {
        match record {
            Ok(book) => {
                let result = conn.execute(
                    "INSERT INTO books (title, author, editor, year, isbn, genre, language, pages, summary)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    (
                        &book.title,
                        &book.author,
                        &book.editor,
                        &book.year,
                        &book.isbn,
                        &book.genre,
                        &book.language,
                        &book.pages,
                        &book.summary,
                    ),
                );

                crate::utils::handle_import_result(
                    &result,
                    &mut imported,
                    &mut failed,
                    &book.title,
                );
            }
            Err(e) => {
                failed += 1;
                print_err(&tr_with(
                    "import.error.parse_failed",
                    &[("line", &index.to_string()), ("error", &e.to_string())],
                ));
            }
        }
    }

    // ✅ Summary message
    if imported > 0 {
        print_ok(
            &tr_with(
                "import.summary.ok",
                &[
                    ("count", &imported.to_string()),
                    ("file", &file_display),
                    ("delimiter", &delimiter.to_string()),
                ],
            ),
            is_verbose(),
        );
    }

    if failed > 0 {
        print_err(&tr_with(
            "import.summary.failed",
            &[("count", &failed.to_string()), ("file", &file_display)],
        ));
    }

    Ok(())
}

/// Handles importing a JSON file into the database.
/// Expects a top-level array of book objects.
pub fn handle_import_json(
    conn: &mut Connection,
    file: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let file_display = file.to_string();

    // ✅ Open JSON file
    let file_handle = crate::utils::open_import_file(file)?;

    let reader = BufReader::new(file_handle);
    let books: Vec<Book> = match serde_json::from_reader(reader) {
        Ok(data) => data,
        Err(e) => {
            print_err(&tr_with(
                "import.error.json_invalid",
                &[("file", &file_display), ("error", &e.to_string())],
            ));
            return Ok(());
        }
    };

    let mut imported = 0;
    let mut failed = 0;

    // ✅ Iterate through records
    for book in books {
        let result = conn.execute(
            "INSERT INTO books (title, author, editor, year, isbn, genre, language, pages, summary)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            (
                &book.title,
                &book.author,
                &book.editor,
                &book.year,
                &book.isbn,
                &book.genre,
                &book.language,
                &book.pages,
                &book.summary,
            ),
        );

        crate::utils::handle_import_result(&result, &mut imported, &mut failed, &book.title);
    }

    // ✅ Summary output
    if imported > 0 {
        print_ok(
            &tr_with(
                "import.summary.ok_json",
                &[("count", &imported.to_string()), ("file", &file_display)],
            ),
            is_verbose(),
        );
    }

    if failed > 0 {
        print_err(&tr_with(
            "import.summary.failed",
            &[("count", &failed.to_string()), ("file", &file_display)],
        ));
    }

    Ok(())
}
