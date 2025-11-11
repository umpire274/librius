pub mod args;
pub mod dispatch;

pub use args::build_cli;
pub use dispatch::run_cli;

use clap::Subcommand;

/// Parsing CLI
pub fn parse_cli() -> clap::ArgMatches {
    build_cli().get_matches()
}

/// Enum di compatibilità con i moduli dei comandi
#[derive(Subcommand)]
pub enum Commands {
    List,
    Config {
        init: bool,
        print: bool,
        edit: bool,
        editor: Option<String>,
    },
    Help,
}
