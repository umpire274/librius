use crate::commands::{config::handle_config, list::handle_list};
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
    /// Enable verbose output (debug mode)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// The command to execute
    #[command(subcommand)]
    pub command: Commands,
}

/// Supported top-level commands for the CLI.
#[derive(Subcommand)]
pub enum Commands {
    /// List all books in your library
    List,
    /// Manage Librius configuration
    Config {
        /// Initialize a new default configuration file
        #[arg(long, help = "Initialize a new default configuration file")]
        init: bool,

        /// Print the current configuration file to stdout
        #[arg(long = "print", help = "Print the current configuration file")]
        print: bool,

        /// Edit the configuration file with your preferred editor
        #[arg(
            long = "edit",
            help = "Edit the configuration file (default editor: $EDITOR, or nano/vim/notepad)"
        )]
        edit: bool,

        /// Specify the editor to use (overrides $EDITOR/$VISUAL).
        /// Common choices: vim, nano.
        #[arg(
            long = "editor",
            requires = "edit",
            help = "Specify the editor to use (vim, nano, or custom path)"
        )]
        editor: Option<String>,
    },
}

/// Esegui il comando scelto
pub fn run_cli(cli: Cli, conn: &Connection) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::List => {
            handle_list(conn).unwrap_or_else(|e| {
                eprintln!("{} {}", "Error listing books:".red(), e);
            });
            Ok(())
        }
        Commands::Config { .. } => Ok(handle_config(&cli.command)?),
    }
}
