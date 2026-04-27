pub mod args;
pub mod dispatch;
pub mod fields;

pub use args::build_cli;
pub use dispatch::run_cli;
pub use fields::EDITABLE_FIELDS;

/// Parsing CLI
pub fn parse_cli() -> clap::ArgMatches {
    build_cli().get_matches()
}
