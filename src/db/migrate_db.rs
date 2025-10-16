use crate::i18n::{tr, tr_with};
use crate::utils::{is_verbose, print_err, print_info, print_ok, write_log};
use rusqlite::{Connection, Result};

/// Represents the outcome of a database migration run.
pub enum MigrationResult {
    /// No pending migrations (everything already up to date)
    None,
    /// Some migrations were applied (with a list of patch names)
    Applied(Vec<String>),
}

/// Runs all pending database migrations.
pub fn run_migrations(conn: &Connection) -> Result<MigrationResult> {
    let mut applied_patches = Vec::new();

    // Ensure log table exists
    conn.execute(
        "CREATE TABLE IF NOT EXISTS log (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            date TEXT NOT NULL,
            operation TEXT NOT NULL,
            target TEXT DEFAULT '',
            message TEXT NOT NULL
        );",
        [],
    )?;

    let patches = vec![
        (
            "PATCH_001",
            patch_001_create_books_table as fn(&Connection) -> Result<()>,
        ),
        (
            "PATCH_002",
            patch_002_add_extra_fields as fn(&Connection) -> Result<()>,
        ),
        (
            "PATCH_003",
            patch_003_add_unique_index_books_isbn as fn(&Connection) -> Result<()>,
        ),
    ];

    for (name, patch_fn) in patches {
        if !is_patch_applied(conn, name)? {
            print_info(
                &tr_with("db.patch,applying", &[("patch", name)]),
                is_verbose(),
            );
            patch_fn(conn)?;
            applied_patches.push(name.to_string());
            record_patch(conn, name, "DB", "Migration applied successfully")?;
        }
    }

    if applied_patches.is_empty() {
        Ok(MigrationResult::None)
    } else {
        write_log(
            conn,
            "MIGRATIONS_COMPLETED",
            "DB",
            &tr("db.patch.all_applied"),
        )?;
        print_ok(&tr("db.patch.all_applied"), is_verbose());
        Ok(MigrationResult::Applied(applied_patches))
    }
}

/// Checks if a migration patch was already applied.
fn is_patch_applied(conn: &Connection, patch_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM log WHERE operation = ?1")?;
    let count: i64 = stmt.query_row([patch_name], |r| r.get(0))?;
    Ok(count > 0)
}

/// Records a migration into the log table.
fn record_patch(conn: &Connection, operation: &str, target: &str, message: &str) -> Result<()> {
    write_log(conn, operation, target, message)?;
    Ok(())
}

/// First migration: initial books table.
fn patch_001_create_books_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            editor TEXT NOT NULL,
            year INTEGER NOT NULL,
            isbn TEXT NOT NULL,
            room TEXT,
            shelf TEXT,
            row TEXT,
            position TEXT,
    		added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
        [],
    )?;
    Ok(())
}

/// Second migration: safely add new fields if missing (language, pages, genre, summary)
fn patch_002_add_extra_fields(conn: &Connection) -> Result<()> {
    print_info(&tr("db.check.missing_columns"), is_verbose());

    // Fetch existing column names
    let mut stmt = conn.prepare("PRAGMA table_info(books);")?;
    let column_iter = stmt.query_map([], |row| row.get::<_, String>(1))?;

    let mut existing_cols = Vec::new();
    for name in column_iter.flatten() {
        existing_cols.push(name);
    }

    // Columns to check
    let new_cols = [
        ("language", "TEXT"),
        ("pages", "INTEGER"),
        ("genre", "TEXT"),
        ("summary", "TEXT"),
    ];

    // Track applied changes
    let mut added_any = false;

    for (col, typ) in new_cols {
        if !existing_cols.iter().any(|c| c.eq_ignore_ascii_case(col)) {
            let sql = format!("ALTER TABLE books ADD COLUMN {} {};", col, typ);
            print_info(&tr_with("db.add.column", &[("column", col)]), is_verbose());
            match conn.execute_batch(&sql) {
                Ok(_) => {
                    print_ok(
                        &tr_with("db.column.added", &[("column", col)]),
                        is_verbose(),
                    );
                    let _ = write_log(
                        conn,
                        "DB_MIGRATION",
                        "DB",
                        &tr_with("log.column.added", &[("column", col)]),
                    );
                    added_any = true;
                }
                Err(e) => {
                    print_err(&tr_with(
                        "db.add.column_failed",
                        &[("column", col), ("error", &e.to_string())],
                    ));
                }
            }
        }
    }

    if !added_any {
        print_ok(&tr("db.column.all_extra_exists"), is_verbose());
    }

    Ok(())
}

/// Migrazione: aggiunge indice UNIQUE su ISBN nella tabella books
fn patch_003_add_unique_index_books_isbn(conn: &Connection) -> Result<()> {
    print_info(&tr("db.migrate.checking_isbn_index"), is_verbose());

    // Controlla se l'indice esiste già
    let exists: bool = conn
        .query_row(
            "SELECT EXISTS (
                SELECT 1 FROM sqlite_master
                WHERE type='index' AND name='idx_books_isbn'
            )",
            [],
            |row| row.get(0),
        )
        .unwrap_or(false);

    if exists {
        print_info(&tr("db.migrate.isbn_index_exists"), is_verbose());
        return Ok(());
    }

    // Crea l'indice UNIQUE su isbn
    match conn.execute(
        "CREATE UNIQUE INDEX IF NOT EXISTS idx_books_isbn ON books(isbn)",
        [],
    ) {
        Ok(_) => {
            print_ok(&tr("db.migrate.isbn_index_created"), is_verbose());
            Ok(())
        }
        Err(e) => {
            print_err(&tr_with(
                "db.migrate.isbn_index_failed",
                &[("error", &e.to_string())],
            ));
            Err(e)
        }
    }
}
