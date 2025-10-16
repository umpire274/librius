//! Utilities for building and formatting tables in Librius CLI.
//!
//! Provides a unified interface for rendering tabular data using the `tabled` crate,
//! ensuring consistent visual style and alignment across commands.

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

pub fn build_vertical_table<T: Serialize>(record: &T) {
    // Converte la struct in una mappa JSON dinamica
    let value = serde_json::to_value(record).expect("Failed to serialize record");

    if let Value::Object(map) = value {
        // Costruiamo un vettore di tuple (campo, valore)
        let mut rows: Vec<(String, String)> = map
            .into_iter()
            .map(|(key, val)| {
                let val_str = match val {
                    Value::Null => String::from("—"),
                    Value::String(s) => s,
                    Value::Number(n) => n.to_string(),
                    Value::Bool(b) => b.to_string(),
                    other => other.to_string(),
                };
                (key, val_str)
            })
            .collect();

        // Ordina alfabeticamente per nome del campo
        rows.sort_by(|a, b| a.0.cmp(&b.0));

        // Crea tabella verticale
        let style = Style::rounded();
        // Build an owned string representation to avoid borrowing temporaries
        let table_str = Table::new(rows).with(style).to_string();

        println!("{}", table_str);
    } else {
        println!("⚠️ Unable to display record: not an object");
    }
}
