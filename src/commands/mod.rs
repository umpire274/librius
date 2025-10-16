//! Command dispatch and subcommand modules for the CLI.
//!
//! This module groups the individual command implementations (for example
//! `list`) so documentation generators can show the available commands and
//! their handlers.

pub mod backup;
pub mod config;
pub mod export;
pub mod import;
pub mod list;

pub use backup::handle_backup;
pub use config::handle_config;
pub use export::handle_export_csv;
pub use export::handle_export_json;
pub use export::handle_export_xlsx;
pub use import::handle_import_csv;
pub use import::handle_import_json;
pub use list::handle_list;
