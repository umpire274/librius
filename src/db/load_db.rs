use crate::config::AppConfig;
use crate::db::migrate_db;
use crate::i18n::{tr, tr_with};
use crate::utils::{is_verbose, print_err, print_info, print_ok, write_log};
use rusqlite::{Connection, Result};
use std::path::Path;

use std::fs;
use std::path::PathBuf;

/// Restituisce il percorso completo del file di database.
///
/// La logica è:
/// - Se esiste `LIBRIUS_DB_PATH` nelle variabili d’ambiente → usa quello.
/// - Altrimenti crea (se necessario) la directory predefinita dell’app:
///     - Linux/macOS: `~/.local/share/librius/librius.db`
///     - Windows: `%APPDATA%\\Librius\\librius.db`
pub fn get_db_path() -> PathBuf {
    // 1️⃣ Se definita, rispetta la variabile d’ambiente
    if let Ok(custom) = std::env::var("LIBRIUS_DB_PATH") {
        return PathBuf::from(custom);
    }

    // 2️⃣ Altrimenti scegli la directory dati predefinita
    let base_dir = dirs::data_dir()
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    let librius_dir = base_dir.join("librius");
    if !librius_dir.exists() {
        let _ = fs::create_dir_all(&librius_dir);
    }

    librius_dir.join("librius.db")
}

/// Opens or initializes the SQLite database.
///
/// - If `config.database` exists, that path is used.
/// - Otherwise, falls back to `get_db_path()`.
/// - If the DB file does not exist, it is created and initialized.
/// - Pending migrations are applied.
/// - Each operation is logged in the `log` table.
pub fn start_db(config: &AppConfig) -> Result<Connection> {
    // 1️⃣ Determina il percorso del database
    let db_path = if config.database.trim().is_empty() {
        get_db_path()
    } else {
        Path::new(&config.database).to_path_buf()
    };

    let db_exists = db_path.exists();

    // 2️⃣ Log di apertura o creazione
    if db_exists {
        print_info(
            &tr_with(
                "db.open.existing",
                &[("db_path", &db_path.display().to_string())],
            ),
            is_verbose(),
        );
    } else {
        print_info(
            &tr_with(
                "db.create.new_db",
                &[("db_path", &db_path.display().to_string())],
            ),
            is_verbose(),
        );
    }

    // 3️⃣ Apertura connessione
    let conn = Connection::open(&db_path)?;

    // 4️⃣ Tabella log (sempre disponibile)
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

    // 5️⃣ Inizializza schema se nuovo DB
    if !db_exists {
        print_info(&tr("db.schema.initializing"), is_verbose());
        if let Err(e) = ensure_schema(&conn) {
            print_err(&tr_with(
                "db.schema.init_failed",
                &[("error", &e.to_string())],
            ));
            let _ = write_log(&conn, "DB_INIT_FAIL", "DB", &e.to_string());
            return Err(e);
        }
        print_ok(&tr("db.schema.created"), is_verbose());
        let _ = write_log(&conn, "DB_INIT_OK", "DB", &tr("log.db.schema.init"));
    }

    // 6️⃣ Esegui eventuali migrazioni
    match migrate_db::run_migrations(&conn) {
        Err(e) => {
            print_err(&tr_with("db.migrate.failed", &[("error", &e.to_string())]));
            let _ = write_log(&conn, "DB_MIGRATION_FAIL", "DB", &e.to_string());
        }
        Ok(result) => match result {
            migrate_db::MigrationResult::Applied(patches) => {
                print_ok(&tr("db.migrate.applied"), is_verbose());
                let msg = &tr_with("log.db.patch_applied", &[("patches", &patches.join(", "))]);
                let _ = write_log(&conn, "DB_MIGRATION_OK", "DB", msg);
            }
            migrate_db::MigrationResult::None => {
                print_ok(&tr("db.schema.already_update"), is_verbose());
            }
        },
    }

    Ok(conn)
}

/// Public compatibility function expected by docs and external callers.
/// Previous API used `db::init_db(&cfg)` returning a `Connection`.
pub fn init_db(config: &AppConfig) -> Result<Connection> {
    start_db(config)
}

/// Ensure required tables exist in an opened connection.
pub fn ensure_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            editor TEXT NOT NULL,
            year INTEGER NOT NULL,
            isbn TEXT NOT NULL,
            language TEXT,
            pages INTEGER,
            genre TEXT,
            summary TEXT,
            room TEXT,
            shelf TEXT,
            row TEXT,
            position TEXT,
            added_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        );",
    )?;
    Ok(())
}
