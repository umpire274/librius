use crate::config::AppConfig;
use crate::db::connection::ensure_schema;
use crate::i18n::{tr, tr_with};
use crate::utils::{print_err, print_ok, print_warn};
use rusqlite::Connection;
use std::error::Error;
use std::fs;
use std::path::Path;

pub fn handle_db(
    config: &AppConfig,
    init: bool,
    reset: bool,
    copy: bool,
    file: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    if init || reset {
        return init_db(config);
    }

    if copy {
        if let Some(dest) = file {
            return copy_db(config, dest);
        }
        print_err(&tr("db_copy_missing_file"));
        return Ok(());
    }

    print_warn(&tr("db_no_action"));
    Ok(())
}

fn init_db(config: &AppConfig) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&config.database);
    if path.exists() {
        fs::remove_file(path)?;
        print_ok(
            &tr_with("db_reset_done", &[("path", &path.to_string_lossy())]),
            true,
        );
    } else {
        print_ok(
            &tr_with("db_init_creating", &[("path", &path.to_string_lossy())]),
            true,
        );
    }

    let conn = Connection::open(path)?;
    ensure_schema(&conn)?;
    print_ok(
        &tr_with("db_init_done", &[("path", &path.to_string_lossy())]),
        true,
    );
    Ok(())
}

fn copy_db(config: &AppConfig, dest: &str) -> Result<(), Box<dyn Error>> {
    let src = Path::new(&config.database);
    if !src.exists() {
        print_err(&tr("db_no_source"));
        return Ok(());
    }

    fs::copy(src, dest)?;
    print_ok(
        &tr_with(
            "db_copy_done",
            &[("source", &src.to_string_lossy()), ("destination", dest)],
        ),
        true,
    );
    Ok(())
}
