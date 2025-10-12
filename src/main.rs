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

mod commands;
mod config;
mod db;
mod models;

use crate::commands::list::handle_list;
use clap::{Parser, Subcommand};
use colored::*;

/// Command line interface description for `librius`.
///
/// This struct is used by `clap` to parse the command line arguments. It is
/// intentionally simple: it exposes a single optional subcommand at the
/// moment (`list`).
#[derive(Parser)]
#[command(
    name = "librius",
    version,
    about = "Manage your personal book collection easily"
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Supported top-level commands for the CLI.
#[derive(Subcommand)]
enum Commands {
    /// List all books in your library
    List,
}

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
    let config = config::load_or_init().expect("Unable to load config");
    let conn = db::init_db(&config).expect("Unable to initialize database");

    match cli.command {
        Some(Commands::List) => {
            if let Err(e) = handle_list(&conn) {
                eprintln!("{}: {}", "Error".red(), e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("{}", "No command provided. Try --help.".yellow());
        }
    }
}
