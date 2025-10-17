use std::collections::HashMap;

/// Converts a Google Books language code (ISO 639-1) into a readable name.
pub fn lang_code_to_name(code: &str) -> &str {
    let map = HashMap::from([
        ("en", "English"),
        ("it", "Italian"),
        ("fr", "French"),
        ("de", "German"),
        ("es", "Spanish"),
        ("pt", "Portuguese"),
        ("ru", "Russian"),
        ("zh", "Chinese"),
        ("ja", "Japanese"),
        ("ar", "Arabic"),
        ("el", "Greek"),
        ("la", "Latin"),
    ]);
    map.get(code).copied().unwrap_or(code)
}
