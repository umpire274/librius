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
use librius::cli::Cli;
use librius::cli::run_cli;

/// Application entry point.
///
/// The `main` function performs the following steps:
/// 1. Parse command line arguments.
/// 2. Load or initialize the YAML configuration file.
/// 3. Initialize the SQLite database connection.
/// 4. Dispatch the requested subcommand (currently only `list`).
///
/// Errors during configuration loading or database initialization will cause
/// the program to terminate with a short error message. Command handlers
/// return `Result` and are handled here to provide a consistent exit code.
fn main() {
    let cli = Cli::parse();

    // Inizializza DB e config
    let config = librius::config::load_or_init().expect("Unable to load config");
    let conn = librius::db::init_db(&config).expect("Unable to initialize database");

    let _ = run_cli(cli, &conn);
}
