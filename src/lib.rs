//! Librius — a small library manager core crate
//!
//! This crate contains the core functionality used by the `librius` binary.
//! It exposes configuration helpers, the database initialization routine
//! and the primary domain model (`Book`).
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

// --- Explicit public API re-exports ---

// Configuration
pub use config::{AppConfig, load_or_init};

// Database
pub use db::{init_db, start_db};

// Domain model
pub use models::Book;

// i18n (needed by internal modules and external callers)
pub use i18n::{load_language, tr, tr_s, tr_with};
