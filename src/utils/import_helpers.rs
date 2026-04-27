// =====================================================
// Librius - utils/import_helpers.rs
// -----------------------------------------------------
// Helper per la gestione dei file e dei risultati
// delle operazioni di import nel database.
// =====================================================

use crate::i18n::tr_with;
use crate::utils::print::print_err;
use rusqlite::Result as SqlResult;
use std::fs::File;
use std::io;

/// Opens a file for import operations and prints a localized error message on failure.
pub fn open_import_file(file: &str) -> Result<File, io::Error> {
    let file_display = file.to_string();

    File::open(file).inspect_err(|e| {
        print_err(&tr_with(
            "import.error.open_failed",
            &[("file", &file_display), ("error", &e.to_string())],
        ));
    })
}

/// Handles the result of a database insert operation for book import.
/// Increments counters and prints localized error messages if necessary.
pub fn handle_import_result(
    result: &SqlResult<usize>,
    imported: &mut u32,
    failed: &mut u32,
    title: &str,
) {
    match result {
        Ok(_) => {
            *imported += 1;
        }
        Err(e) => {
            *failed += 1;
            print_err(&tr_with(
                "import.error.insert_failed",
                &[("title", title), ("error", &e.to_string())],
            ));
        }
    }
}
