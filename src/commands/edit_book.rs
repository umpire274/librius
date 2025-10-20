use crate::db::books::{get_book_fields, update_book_by_id, update_book_by_isbn};
use crate::fields::EDITABLE_FIELDS;
use crate::{lang_code_to_name, print_err, print_info, print_ok, print_warn, tr, tr_with};
use rusqlite::Connection;
use std::collections::HashMap;

pub fn handle_edit_book(conn: &Connection, matches: &clap::ArgMatches) -> rusqlite::Result<()> {
    let key = matches
        .get_one::<String>("key")
        .expect("Book ID or ISBN is required");

    let mut fields = HashMap::new();

    println!();

    for (field, _, _) in EDITABLE_FIELDS {
        if let Some(value) = matches.get_one::<String>(field) {
            // 👇 mappa language_book → language
            let db_field = if *field == "language_book" {
                "language"
            } else {
                *field
            };
            let val = value.trim();

            // converte codice lingua in nome leggibile
            let final_val = if *field == "language_book" {
                lang_code_to_name(val).to_string()
            } else {
                val.to_string()
            };

            fields.insert(db_field.to_string(), final_val);
        }
    }

    if fields.is_empty() {
        print_warn(&tr("edit.book.error_no_field"));
        return Ok(());
    }

    // Heuristic: if contains letters, dash, or 13+ digits → ISBN; otherwise ID
    let is_isbn = key.len() >= 10 && !key.chars().all(|c| c.is_ascii_digit());

    // Recupera i valori precedenti dal DB
    let old_values = get_book_fields(
        conn,
        key,
        &fields.keys().cloned().collect::<Vec<_>>(),
        is_isbn,
    )?;

    let result = if is_isbn {
        update_book_by_isbn(conn, key, &fields)
    } else {
        match key.parse::<i64>() {
            Ok(id) => update_book_by_id(conn, id, &fields),
            Err(_) => {
                print_err(&tr("edit.book.error_invalid_id"));
                return Ok(());
            }
        }
    };

    match result {
        Ok(rows) if rows > 0 => {
            let mut modified_count = 0;

            // Confronta valori e stampa diff
            for (field, new_val) in &fields {
                let old_val = old_values.get(field).cloned().flatten();

                match old_val {
                    Some(old) if old != *new_val => {
                        print_ok(
                            &tr_with(
                                "edit.field.updated",
                                &[("field", field), ("old", &old), ("new", new_val)],
                            ),
                            true,
                        );

                        modified_count += 1;
                    }
                    None => {
                        print_ok(
                            &tr_with("edit.field.set", &[("field", field), ("new", new_val)]),
                            true,
                        );

                        modified_count += 1;
                    }
                    Some(_) => {
                        print_info(&tr_with("edit.field.unchanged", &[("field", field)]), true);
                    }
                }
            }

            if modified_count > 0 {
                let key_variant = if modified_count == 1 {
                    "edit.book.updated_one"
                } else {
                    "edit.book.updated_many"
                };

                print_ok(
                    &tr_with(
                        key_variant,
                        &[("key", key), ("count", &modified_count.to_string())],
                    ),
                    true,
                );
            } else {
                print_warn(&tr("edit.book.no_changes"));
            }
        }
        Ok(_) => print_warn(&tr("edit.book.not_found")),
        Err(err) => print_err(&tr_with(
            "edit.book.error_updating",
            &[("error", &err.to_string())],
        )),
    }

    Ok(())
}
