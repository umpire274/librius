use crate::utils::{print_ok, write_log};
use rusqlite::{Connection, Result};

/// Runs all pending database migrations.
pub fn run_migrations(conn: &Connection) -> Result<()> {
    let mut migrated = false;

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
            patch_fn(conn)?;
            migrated = true;
            record_patch(conn, name, "DB", "Migration applied successfully")?;
        }
    }

    if migrated {
        write_log(
            conn,
            "MIGRATIONS_COMPLETED",
            "DB",
            "All pending migrations applied",
        )?;
        print_ok("All pending migrations applied", true);
    }

    Ok(())
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

/// Second migration: add new fields (language, pages, genre, summary)
fn patch_002_add_extra_fields(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        ALTER TABLE books ADD COLUMN language TEXT;
        ALTER TABLE books ADD COLUMN pages INTEGER;
        ALTER TABLE books ADD COLUMN genre TEXT;
        ALTER TABLE books ADD COLUMN summary TEXT;
        ",
    )?;
    Ok(())
}
