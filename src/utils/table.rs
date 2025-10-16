//! Utilities for building and formatting tables in Librius CLI.
//!
//! Provides a unified interface for rendering tabular data using the `tabled` crate,
//! ensuring consistent visual style and alignment across commands.

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
