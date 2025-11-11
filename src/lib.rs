//! Librius — a small library manager core crate
//!
//! This crate contains the core functionality used by the `librius` binary.
//! It is intentionally lightweight and exposes configuration helpers, the
//! database initialization routine and the primary domain model (`Book`).
//!
//! The binary (`src/main.rs`) uses this crate to perform startup and to
//! dispatch command handlers. Including a `lib.rs` makes this project
//! suitable for documentation generation on platforms such as docs.rs.
//!
//! Example
//!
//! ```no_run
//! use librius::config::AppConfig;
//! use librius::db;
//!
//! // load or create config, then init database
//! let cfg: AppConfig = librius::config::load_or_init().unwrap();
//! let conn = db::init_db(&cfg).unwrap();
//! ```

pub mod cli;
pub mod commands;
pub mod config;
pub mod db;
pub mod i18n;
pub mod models;
pub mod utils;

pub use commands::*;
pub use config::*;
pub use db::*;
pub use i18n::*;
pub use models::*;
pub use utils::*;
