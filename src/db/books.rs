use rusqlite::types::ValueRef;
use rusqlite::{Connection, Result, params, params_from_iter};
use std::collections::HashMap;

/// Costruisce la parte "SET col1 = ?, col2 = ?, ..." della query SQL
/// e restituisce anche il vettore dei valori corrispondenti.
///
/// Esempio:
/// Input: {"title": "1984", "author": "Orwell"}
/// Output: ("SET title = ?, author = ?", vec!["1984", "Orwell"])
fn build_update_clause(fields: &HashMap<String, String>) -> (String, Vec<String>) {
    let mut sql = String::new();
    let mut params_vec = Vec::new();

    for (i, (key, value)) in fields.iter().enumerate() {
        sql.push_str(&format!("{} = ?", key));
        if i < fields.len() - 1 {
            sql.push_str(", ");
        }
        params_vec.push(value.clone());
    }

    (sql, params_vec)
}

pub fn update_book_by_id(
    conn: &Connection,
    id: i64,
    fields: &HashMap<String, String>,
) -> Result<usize> {
    if fields.is_empty() {
        return Ok(0);
    }

    let (set_clause, mut params_vec) = build_update_clause(fields);

    let sql = format!("UPDATE books SET {} WHERE id = ?", set_clause);
    params_vec.push(id.to_string());

    let mut stmt = conn.prepare(&sql)?;
    let rows_affected = stmt.execute(params_from_iter(params_vec.iter()))?;
    Ok(rows_affected)
}

pub fn update_book_by_isbn(
    conn: &Connection,
    isbn: &str,
    fields: &HashMap<String, String>,
) -> Result<usize> {
    if fields.is_empty() {
        return Ok(0);
    }

    let (set_clause, mut params_vec) = build_update_clause(fields);

    let sql = format!("UPDATE books SET {} WHERE isbn = ?", set_clause);
    params_vec.push(isbn.to_string());

    let mut stmt = conn.prepare(&sql)?;
    let rows_affected = stmt.execute(params_from_iter(params_vec.iter()))?;
    Ok(rows_affected)
}

/// Retrieve current values of the specified fields for a given book (by ID or ISBN).
pub fn get_book_fields(
    conn: &Connection,
    key: &str,
    fields: &[String],
    is_isbn: bool,
) -> Result<HashMap<String, Option<String>>> {
    let mut old_values: HashMap<String, Option<String>> = HashMap::new();

    for field in fields {
        let query = if is_isbn {
            format!("SELECT {} FROM books WHERE isbn = ?", field)
        } else {
            format!("SELECT {} FROM books WHERE id = ?", field)
        };

        let result: Option<String> = conn
            .query_row(&query, params![key], |row| match row.get_ref(0)? {
                ValueRef::Text(t) => Ok(Some(String::from_utf8_lossy(t).to_string())),
                ValueRef::Integer(i) => Ok(Some(i.to_string())),
                ValueRef::Real(f) => Ok(Some(f.to_string())),
                ValueRef::Null => Ok(None),
                _ => Ok(None),
            })
            .unwrap_or_else(|_| None);

        old_values.insert(field.clone(), result);
    }

    Ok(old_values)
}
