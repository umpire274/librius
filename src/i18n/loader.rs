use once_cell::sync::Lazy;
use serde_json::Value;
use std::collections::HashMap;
use std::io;
use std::sync::RwLock;

/// Parses a JSON string and returns a HashMap of key → text pairs.
/// Used for both embedded and (future) external locale loading.
pub fn parse_json_to_map(content: &str) -> io::Result<HashMap<String, String>> {
    // NOTE: For now Librius embeds all locales via `include_str!()`.
    // If in the future external JSON files are reintroduced,
    // implement a `load_from_file()` here using `fs::read_to_string()`.
    //
    // Example:
    // pub fn load_from_file(path: &str) -> io::Result<HashMap<String, String>> {
    //     let data = fs::read_to_string(path)?;
    //     parse_json_to_map(&data)
    // }

    let json: Value =
        serde_json::from_str(content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut map = HashMap::new();

    if let Some(obj) = json.as_object() {
        for (k, v) in obj {
            if let Some(text) = v.as_str() {
                map.insert(k.clone(), text.to_string());
            }
        }
    }

    Ok(map)
}

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

pub fn tr_s(key: &str) -> &'static str {
    Box::leak(tr(key).into_boxed_str())
}
