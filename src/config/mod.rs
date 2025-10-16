//! Configuration utilities for Librius.
//!
//! This module contains types and helper functions to locate, create and load
//! the YAML configuration file used by the application. The configuration is
//! intentionally small: it currently only stores the path to the SQLite
//! database file used by the CLI.
//!
//! The functions here are used by the binary at startup to ensure a
//! deterministic configuration directory and to persist a default
//! configuration when none exists.

pub mod load_config;
pub mod migrate_config;

pub use load_config::{AppConfig, config_file_path, load_or_init};
pub use migrate_config::migrate_config;
