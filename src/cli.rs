use crate::commands::list::handle_list;
use clap::{Parser, Subcommand};
use colored::Colorize;
use rusqlite::Connection;

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
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Supported top-level commands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// List all books in your library
    List,
}

/// Esegui il comando scelto
pub fn run_cli(cli: Cli, conn: &Connection) {
    match cli.command {
        Some(Commands::List) => {
            if let Err(e) = handle_list(conn) {
                eprintln!("{}: {}", "Error".red(), e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("{}", "No command provided. Try --help.".yellow());
        }
    }
}
