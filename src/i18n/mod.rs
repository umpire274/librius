//! i18n - Embedded Internationalization module for Librius
mod loader;

pub use loader::{load_language, parse_json_to_map, tr, tr_s, tr_with};
