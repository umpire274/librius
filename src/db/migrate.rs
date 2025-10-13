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
    ];

    for (name, patch_fn) in patches {
        if !is_patch_applied(conn, name)? {
            print_info(&format!("Applying database patch: {}", name), is_verbose());
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
            "All pending migrations applied",
        )?;
        print_ok("All pending migrations applied.", is_verbose());
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
    print_info("Checking for missing columns in 'books'...", is_verbose());

    // Fetch existing column names
    let mut stmt = conn.prepare("PRAGMA table_info(books);")?;
    let column_iter = stmt.query_map([], |row| Ok(row.get::<_, String>(1)?))?;

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
            print_info(
                &format!("Adding column '{}' to books...", col),
                is_verbose(),
            );
            match conn.execute_batch(&sql) {
                Ok(_) => {
                    print_ok(
                        &format!("Column '{}' added successfully.", col),
                        is_verbose(),
                    );
                    let _ = write_log(
                        conn,
                        "DB_MIGRATION",
                        "DB",
                        &format!("Added column '{}'", col),
                    );
                    added_any = true;
                }
                Err(e) => {
                    print_err(&format!("Failed to add column '{}': {}", col, e));
                }
            }
        }
    }

    if !added_any {
        print_ok(
            "All required columns already exist. No changes applied.",
            is_verbose(),
        );
    }

    Ok(())
}
