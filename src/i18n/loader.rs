use serde_json::Value;
use std::collections::HashMap;
use std::io;

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
