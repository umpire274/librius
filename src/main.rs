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
use librius::utils::icons::ERR;
use librius::utils::{is_verbose, print_err, print_info, print_ok, set_verbose};

fn main() {
    let cli = Cli::parse();
    set_verbose(cli.verbose);

    // ------------------------------------------------------------
    // 1️⃣ Load or initialize configuration file
    // ------------------------------------------------------------
    print_info("Loading configuration...", is_verbose());
    let config = config::load_or_init().unwrap_or_else(|_| panic!("{}Unable to load config", ERR));

    // ------------------------------------------------------------
    // 2️⃣ Initialize or open the database (delegated to start_db)
    // ------------------------------------------------------------
    let conn = db::start_db(&config).unwrap_or_else(|_| panic!("{}Unable to start database", ERR));

    // ---------------------------------------------------------------------
    // 3️⃣ Run DB migrations (applies missing patches) and config migrations
    // ---------------------------------------------------------------------
    if let Err(e) = db::migrate::run_migrations(&conn) {
        print_err(&format!("Database migration failed: {}", e));
    } else {
        print_ok("Database schema is up-to-date.", is_verbose());
    }
    // Apply config migrations if needed
    if let Err(e) = config::migrate::migrate_config(&conn, &config::config_file_path()) {
        print_err(&format!("Config migration failed: {}", e));
    } else {
        print_ok("Configuration verified.", is_verbose());
    }

    // ------------------------------------------------------------
    // 4️⃣ Execute CLI command
    // ------------------------------------------------------------
    let _ = run_cli(cli, &conn);
}
