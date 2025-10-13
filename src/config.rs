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

use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{env, fs, path::PathBuf};

/// Application configuration stored in YAML format.
///
/// This struct is serialized/deserialized with `serde` and persisted in a
/// simple YAML file placed inside the per-user configuration directory.
///
/// Fields:
/// - `db_path`: filesystem path to the SQLite database used by Librius.
#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    /// Path to the local SQLite database
    pub db_path: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        let db_path = default_db_path();
        Self {
            db_path: db_path.to_string_lossy().to_string(),
        }
    }
}

/// Return the directory used to store Librius configuration and database.
///
/// On Unix-like systems this is `$HOME/.librius`. On Windows the function
/// attempts to use `%APPDATA%/Roaming/librius`. If neither environment
/// variable is available, the function falls back to a `.librius` folder in
/// the current working directory.
fn config_dir() -> PathBuf {
    if cfg!(target_os = "windows") {
        if let Ok(appdata) = env::var("APPDATA") {
            let mut path = PathBuf::from(appdata);
            path.push("Roaming");
            path.push("librius");
            return path;
        }
    } else if let Ok(home) = env::var("HOME") {
        let mut path = PathBuf::from(home);
        path.push(".librius");
        return path;
    }
    // fallback to current directory
    PathBuf::from(".librius")
}

/// Construct the default path for the SQLite database file.
///
/// The function ensures the configuration directory exists and returns the
/// full path to `librius.db` inside it.
fn default_db_path() -> PathBuf {
    let mut path = config_dir();
    fs::create_dir_all(&path).ok();
    path.push("librius.db");
    path
}

/// Construct the path to the YAML configuration file used by the app.
///
/// The returned path points to `librius.conf` inside the configuration
/// directory. The function also attempts to create the directory if it does
/// not already exist.
pub(crate) fn config_file_path() -> PathBuf {
    let mut path = config_dir();
    fs::create_dir_all(&path).ok();
    path.push("librius.conf");
    path
}

/// Load the YAML configuration from disk, or create a default one if not
/// present.
///
/// This function returns a fully populated `AppConfig` or an error if there
/// was a problem reading or writing the configuration file. The YAML file is
/// created using `serde_yaml` when missing.
///
/// # Errors
/// Returns an `Err` boxed trait object when IO or serialization fails.
///
/// # Example
/// ```no_run
/// use librius::config::AppConfig;
/// // This will create a default config file in the user's config dir if missing
/// let cfg: AppConfig = librius::config::load_or_init().expect("load config");
/// println!("db path: {}", cfg.db_path);
/// ```
pub fn load_or_init() -> Result<AppConfig, Box<dyn std::error::Error>> {
    let config_path = config_file_path();

    if config_path.exists() {
        let contents = fs::read_to_string(&config_path)?;
        let cfg: AppConfig = serde_yaml::from_str(&contents)?;
        Ok(cfg)
    } else {
        let cfg = AppConfig::default();
        let yaml = serde_yaml::to_string(&cfg)?;
        let mut file = fs::File::create(&config_path)?;
        file.write_all(yaml.as_bytes())?;
        Ok(cfg)
    }
}
