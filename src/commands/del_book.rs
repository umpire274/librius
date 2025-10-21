use crate::{print_err, print_info, print_ok, print_warn, tr_with, write_log};
use colored::*;
use rusqlite::Connection;
use std::io::{self, Write};

pub fn handle_del_book(conn: &Connection, key: &str, force: bool) -> rusqlite::Result<()> {
    println!();

    // 1️⃣ Determina se è ISBN o ID
    let is_isbn = key.len() >= 10 && !key.chars().all(|c| c.is_ascii_digit());

    // 2️⃣ Controlla se il libro esiste
    let exists_query = if is_isbn {
        "SELECT COUNT(*) FROM books WHERE isbn = ?1"
    } else {
        "SELECT COUNT(*) FROM books WHERE id = ?1"
    };

    let exists: i64 = conn.query_row(exists_query, [key], |row| row.get(0))?;

    if exists == 0 {
        print_warn(&tr_with("del.book.not_found", &[("key", key)]).yellow());
        return Ok(());
    }

    // 3️⃣ Conferma interattiva (solo se il libro esiste), se non forzato
    if !force {
        print!(
            "{} ",
            tr_with("del.book.confirm", &[("key", key)]) // es. "Are you sure you want to delete book {key}? [y/N]:"
        );
        io::stdout().flush().unwrap();

        let mut answer = String::new();
        io::stdin().read_line(&mut answer).unwrap();

        if !matches!(answer.trim().to_lowercase().as_str(), "y" | "yes") {
            print_info(&tr_with("del.book.cancelled", &[("key", key)]).cyan(), true);
            return Ok(());
        }

        println!();
    }

    // 4️⃣ Esegue la DELETE solo dopo conferma
    let delete_sql = if is_isbn {
        "DELETE FROM books WHERE isbn = ?1"
    } else {
        "DELETE FROM books WHERE id = ?1"
    };

    let affected = conn.execute(delete_sql, [key])?;

    if affected > 0 {
        // Log the action
        let action_type = if force { "forced" } else { "confirmed" };
        let log_msg = format!("Book {} deleted ({})", key, action_type);
        if let Err(e) = write_log(conn, "DELETE_BOOK", "", &log_msg) {
            print_err(
                &tr_with(
                    "log.record.unable_to_write",
                    &[("log_error", &e.to_string())],
                )
                .red()
                .bold(),
            );
        }

        print_ok(&tr_with("del.book.success", &[("key", key)]).green(), true);
    } else {
        print_err(&tr_with("del.book.not_found", &[("key", key)]).red().bold());
    }

    Ok(())
}
