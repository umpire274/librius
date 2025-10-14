use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io;

/// Loads a JSON translation file and returns a HashMap of key → value pairs.
pub fn load_from_file(path: &str) -> io::Result<HashMap<String, String>> {
    let data = fs::read_to_string(path)?;
    let json: Value = serde_json::from_str(&data)?;

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
