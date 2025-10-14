//! Librius — Personal Library Manager CLI
//!
//! This binary provides a small command line interface to manage a personal
//! collection of books stored in a local SQLite database. The primary
//! functionality implemented today is listing the stored books. The public
//! functions and types are documented to make the project suitable for
//! publishing documentation (for example on docs.rs) and for users reading
//! the source.
//!
//! Usage (example):
//!
//! ```sh
//! librius list
//! ```

use clap::Parser;
use librius::cli::{Cli, run_cli};
use librius::config;
use librius::db;
use librius::i18n::{load_language, tr, tr_with};
use librius::utils::icons::ERR;
use librius::utils::{is_verbose, print_err, print_info, print_ok, set_verbose, write_log};

fn main() {
    let cli = Cli::parse();
    set_verbose(cli.verbose);

    // ------------------------------------------------------------
    // 1️⃣ Load or initialize configuration file
    // ------------------------------------------------------------
    print_info("Load configuration...", is_verbose());
    let config = config::load_or_init().unwrap_or_else(|_| panic!("{}Unable to load config", ERR));

    //------------------------------------------------------------
    // Load the selected language for translations
    //------------------------------------------------------------
    print_info("Initialize language...", is_verbose());
    let lang_code = if let Some(lang) = &cli.lang {
        print_info(
            format!("Overriding language to: {}", lang).as_str(),
            is_verbose(),
        );
        lang.clone()
    } else {
        print_info(
            format!("Using configured language: {}", &config.language).as_str(),
            is_verbose(),
        );
        config::load_language_from_conf().unwrap_or_else(|| "en".to_string())
    };
    load_language(&lang_code);

    // ------------------------------------------------------------
    // 2️⃣ Initialize or open the database (delegated to start_db)
    // ------------------------------------------------------------
    let conn = db::start_db(&config)
        .unwrap_or_else(|_| panic!("{}", &tr_with("db.open.failed", &[("icon-err", ERR)])));

    write_log(
        &conn,
        "LANG_SET",
        "I18N",
        &format!("Language: {}", lang_code),
    )
    .ok();

    // ---------------------------------------------------------------------
    // 3️⃣ Run DB migrations (applies missing patches) and config migrations
    // ---------------------------------------------------------------------
    if let Err(e) = db::migrate::run_migrations(&conn) {
        print_err(&tr_with("db.migrate.failed", &[("error", &e.to_string())]));
    } else {
        print_ok(tr("db.schema.verified").as_str(), is_verbose());
    }
    // Apply config migrations if needed
    if let Err(e) = config::migrate::migrate_config(&conn, &config::config_file_path()) {
        print_err(&tr_with(
            "config.migrate.failed",
            &[("error", &e.to_string())],
        ));
    } else {
        print_ok(tr("config.schema.verified").as_str(), is_verbose());
    }

    // ------------------------------------------------------------
    // 4️⃣ Execute CLI command
    // ------------------------------------------------------------
    let _ = run_cli(cli, &conn);
}
