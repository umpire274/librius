//! i18n - Embedded Internationalization module for Librius

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::RwLock;

mod loader;
use loader::parse_json_to_map;

/// Global translation map
static TRANSLATIONS: Lazy<RwLock<HashMap<String, String>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Loads translations for the selected language (embedded JSON)
pub fn load_language(lang_code: &str) {
    let mut map = TRANSLATIONS.write().unwrap();
    map.clear();

    // Select embedded JSON string
    let content = match lang_code {
        "it" => include_str!("locales/it.json"),
        "en" => include_str!("locales/en.json"),
        _ => include_str!("locales/en.json"),
    };

    // Parse JSON using shared helper
    *map = parse_json_to_map(content).expect("Invalid embedded locale JSON");
}

/// Returns the translation for the given key.
pub fn tr(key: &str) -> String {
    TRANSLATIONS
        .read()
        .unwrap()
        .get(key)
        .cloned()
        .unwrap_or_else(|| key.to_string())
}

/// Same as `tr`, but with runtime placeholder substitution.
pub fn tr_with(key: &str, vars: &[(&str, &str)]) -> String {
    let mut s = tr(key);
    for (k, v) in vars {
        let placeholder = format!("{{{}}}", k);
        s = s.replace(&placeholder, v);
    }
    s
}
