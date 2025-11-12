//! Utilities for building and formatting tables in Librius CLI.
//!
//! Provides a unified interface for rendering tabular data using the `tabled` crate,
//! ensuring consistent visual style and alignment across commands.

use crate::{print_warn, tr};
use serde::Serialize;
use serde_json::Value;
use tabled::settings::{Alignment, Modify, Style, object::Rows};
use tabled::{Table, Tabled};

/// Build and format a table with a consistent Librius style.
///
/// # Parameters
/// * `rows` — Any iterator of types implementing [`Tabled`].
///
/// # Example
/// ```no_run
/// use librius::utils::table::build_table;
/// // Esempio illustrativo: crea una struttura che implementi `Tabled` e
/// // passa una collezione a `build_table`.
/// // #[derive(tabled::Tabled)]
/// // struct MyTableRow { col: &'static str }
/// // let data = vec![MyTableRow { col: "value" }];
/// // println!("{}", build_table(data));
/// ```
pub fn build_table<T, I>(rows: I) -> String
where
    T: Tabled,
    I: IntoIterator<Item = T>,
{
    let s = Style::modern();

    Table::new(rows)
        // stile tabellare coerente
        .with(s)
        // allineamento a sinistra per le righe di dati
        .with(Modify::new(Rows::new(1..)).with(Alignment::left()))
        .to_string()
}

#[derive(Tabled)]
struct VerticalRow {
    #[tabled(rename = "Field")]
    field: String,
    #[tabled(rename = "Value")]
    value: String,
}

pub fn build_vertical_table<T: Serialize>(record: &T, compact: bool) {
    // Serializza in mappa dinamica
    let value = serde_json::to_value(record).expect("Failed to serialize record");

    if let Value::Object(map) = value {
        // ✅ Ordine corretto dei campi
        let field_order = [
            "id", "title", "author", "editor", "year", "isbn", "language", "pages", "genre",
            "summary", "room", "shelf", "row", "position", "added_at",
        ];

        // ✅ Etichette localizzate
        let field_labels = [
            tr("list.header.id"),
            tr("list.header.title"),
            tr("list.header.author"),
            tr("list.header.editor"),
            tr("list.header.year"),
            tr("list.header.ISBN"),
            tr("list.header.language"),
            tr("list.header.pages"),
            tr("list.header.genre"),
            tr("list.header.summary"),
            tr("list.header.room"),
            tr("list.header.shelf"),
            tr("list.header.row"),
            tr("list.header.position"),
            tr("list.header.added_at"),
        ];

        // ✅ Costruisci righe ordinate
        let mut rows: Vec<VerticalRow> = Vec::new();

        for (i, key) in field_order.iter().enumerate() {
            let value_str = map
                .get(*key)
                .map(|v| match v {
                    Value::Null => "—".to_string(),
                    Value::String(s) => s.clone(),
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                })
                .unwrap_or_else(|| "—".to_string());

            // ✅ In modalità compatta, salta i campi vuoti
            if compact && value_str == "—" {
                continue;
            }

            rows.push(VerticalRow {
                field: field_labels[i].clone(),
                value: value_str,
            });
        }

        // ✅ Crea e stampa la tabella
        let mut table = Table::new(rows);
        table.with(Style::rounded());
        println!("{}", table);
    } else {
        print_warn(&tr("error.unable_display_record"));
    }
}
