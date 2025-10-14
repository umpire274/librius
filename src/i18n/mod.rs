//! i18n - Internationalization module for Librius
//!
//! Provides language loading and translation lookup via JSON files.
//! Supported languages are stored in `src/i18n/locales/`.

use std::collections::HashMap;
use std::sync::RwLock;

use once_cell::sync::Lazy;

mod loader;
use crate::utils::{is_verbose, print_info, print_warn};
use loader::load_from_file;

/// Global translations map (language loaded at runtime)
static TRANSLATIONS: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Loads a language file (e.g. "en", "it") from `src/i18n/locales/`.
///
/// If the requested language cannot be found, it falls back to English.
pub fn load_language(lang_code: &str) {
    let mut map = TRANSLATIONS.write().unwrap();
    map.clear();

    let lang_file = format!("src/i18n/locales/{}.json", lang_code);
    match load_from_file(&lang_file) {
        Ok(translations) => {
            *map = translations;
            print_info(
                &tr_with("app.language.loaded", &[("lang", lang_code)]),
                is_verbose(),
            );
        }
        Err(_) => {
            print_warn(&tr_with("app.language.not_found", &[("lang", lang_code)]));
            if let Ok(fallback) = load_from_file("src/i18n/locales/en.json") {
                *map = fallback;
            }
        }
    }
}

/// Translates a given key using the loaded language map.
///
/// If the key is missing, returns the key itself.
pub fn tr(key: &str) -> String {
    TRANSLATIONS
        .read()
        .unwrap()
        .get(key)
        .cloned()
        .unwrap_or_else(|| key.to_string())
}

// src/i18n/mod.rs
pub fn tr_with(key: &str, vars: &[(&str, &str)]) -> String {
    let mut s = tr(key);
    for (k, v) in vars {
        let needle = format!("{{{}}}", k);
        s = s.replace(&needle, v);
    }
    s
}
