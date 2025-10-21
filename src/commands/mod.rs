//! Command dispatch and subcommand modules for the CLI.
//!
//! This module groups the individual command implementations (for example
//! `list`) so documentation generators can show the available commands and
//! their handlers.

pub mod add;
pub mod add_book;
pub mod backup;
pub mod config;
pub mod del_book;
pub mod edit_book;
pub mod export;
pub mod import;
pub mod list;

pub use add::handle_add;
pub use add_book::handle_add_book;
pub use backup::handle_backup;
pub use config::handle_config;
pub use del_book::handle_del_book;
pub use edit_book::handle_edit_book;
pub use export::handle_export_csv;
pub use export::handle_export_json;
pub use export::handle_export_xlsx;
pub use import::handle_import_csv;
pub use import::handle_import_json;
pub use list::handle_list;
