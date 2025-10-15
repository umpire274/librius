use chrono::Local;
use rusqlite::Connection;
use serde_json::json;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use crate::i18n::tr_with;
use crate::utils::print_ok;

// Alias per ridurre la complessità del tipo (Clippy: type_complexity)
type BookRow = (i64, String, String, Option<i64>);

// 🔧 comuni: directory export + query
fn get_export_path(ext: &str, output: Option<String>) -> io::Result<PathBuf> {
    let export_dir = dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("librius")
        .join("exports");
    fs::create_dir_all(&export_dir)?;

    let filename = output.unwrap_or_else(|| {
        format!(
            "librius_export_{}.{}",
            Local::now().format("%Y-%m-%d_%H-%M-%S"),
            ext
        )
    });

    Ok(export_dir.join(filename))
}

// 🔧 query generica
fn fetch_books(conn: &Connection) -> rusqlite::Result<Vec<BookRow>> {
    let mut stmt = conn.prepare("SELECT id, title, author, year FROM books ORDER BY id;")?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<i64>>(3)?,
            ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(rows)
}

//
// 🧩 CSV EXPORT
//
pub fn handle_export_csv(conn: &Connection, output: Option<String>) -> io::Result<()> {
    let export_path = get_export_path("csv", output)?;
    let rows = fetch_books(conn).map_err(|e| io::Error::other(e.to_string()))?;

    let mut file = File::create(&export_path)?;
    writeln!(file, "id;title;author;year")?;

    for (id, title, author, year) in rows {
        writeln!(file, "{};{};{};{}", id, title, author, year.unwrap_or(0))?;
    }

    println!();
    print_ok(
        &tr_with(
            "export.csv.ok",
            &[("path", &export_path.display().to_string())],
        ),
        true,
    );

    Ok(())
}

//
// 🧩 JSON EXPORT
//
pub fn handle_export_json(conn: &Connection, output: Option<String>) -> io::Result<()> {
    let export_path = get_export_path("json", output)?;
    let rows = fetch_books(conn).map_err(|e| io::Error::other(e.to_string()))?;

    let json_rows: Vec<_> = rows
        .into_iter()
        .map(|(id, title, author, year)| {
            json!({
                "id": id,
                "title": title,
                "author": author,
                "year": year.unwrap_or_default()
            })
        })
        .collect();

    let file = File::create(&export_path)?;
    serde_json::to_writer_pretty(file, &json_rows).map_err(|e| io::Error::other(e.to_string()))?;

    println!();
    print_ok(
        &tr_with(
            "export.json.ok",
            &[("path", &export_path.display().to_string())],
        ),
        true,
    );

    Ok(())
}

//
// 🧩 XLSX EXPORT
//
pub fn handle_export_xlsx(conn: &Connection, output: Option<String>) -> io::Result<()> {
    let export_path = get_export_path("xlsx", output)?;
    let rows = fetch_books(conn).map_err(|e| io::Error::other(e.to_string()))?;

    let mut workbook = umya_spreadsheet::new_file();
    let sheet = workbook.get_sheet_by_name_mut("Sheet1").unwrap();

    // intestazioni
    sheet.get_cell_mut("A1").set_value("ID");
    sheet.get_cell_mut("B1").set_value("Title");
    sheet.get_cell_mut("C1").set_value("Author");
    sheet.get_cell_mut("D1").set_value("Year");

    // righe
    for (i, (id, title, author, year)) in rows.iter().enumerate() {
        let row_index = i + 2; // Excel rows start at 1, first data row is 2
        sheet
            .get_cell_mut(format!("A{}", row_index))
            .set_value(id.to_string());
        sheet
            .get_cell_mut(format!("B{}", row_index))
            .set_value(title);
        sheet
            .get_cell_mut(format!("C{}", row_index))
            .set_value(author);
        sheet
            .get_cell_mut(format!("D{}", row_index))
            .set_value(year.unwrap_or(0).to_string());
    }

    umya_spreadsheet::writer::xlsx::write(&workbook, &export_path)
        .map_err(|e| io::Error::other(e.to_string()))?;

    println!();
    print_ok(
        &tr_with(
            "export.xlsx.ok",
            &[("path", &export_path.display().to_string())],
        ),
        true,
    );

    Ok(())
}
