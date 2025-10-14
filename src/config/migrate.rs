use crate::i18n::tr;
use crate::utils::icons::ERR;
use crate::utils::{print_ok, write_log};
use rusqlite::Connection;
use serde_yaml::{Mapping, Value};
use std::fs;
use std::io::{self};
use std::path::Path;

/// Checks and updates the configuration file structure if needed.
pub fn migrate_config(conn: &Connection, conf_path: &Path) -> io::Result<()> {
    let content = fs::read_to_string(conf_path)?;
    let mut yaml: Value = serde_yaml::from_str(&content)
        .unwrap_or_else(|_| panic!("{}{}", ERR, &tr("app.yaml.parse_error")));

    // Ensure we have a Mapping; if not, replace yaml with an empty mapping
    let map: &mut Mapping = match yaml.as_mapping_mut() {
        Some(m) => m,
        None => {
            yaml = Value::Mapping(Mapping::new());
            // Now it's guaranteed to be a mapping
            yaml.as_mapping_mut()
                .expect(&tr("app.mapping.just_created"))
        }
    };

    // Add missing keys with defaults
    let migrated = insert_if_missing(map, "language", "en");

    let updated = serde_yaml::to_string(&yaml)
        .unwrap_or_else(|_| panic!("{}{}", ERR, &tr("app.yaml.serialize_error")));
    fs::write(conf_path, updated)?;

    if migrated {
        // Log the config migration
        write_log(
            conn,
            "MIGRATE_CONFIG",
            "CONFIG",
            &tr("config.file.migrated"),
        )
        .unwrap_or_else(|_| panic!("{}Could not write log message", ERR));
        print_ok(&tr("config.file.migrated"), true);
    }
    Ok(())
}

fn insert_if_missing(map: &mut Mapping, key: &str, default: &str) -> bool {
    let k = Value::String(key.to_string());
    if !map.contains_key(&k) {
        map.insert(k, Value::String(default.to_string()));
        return true;
    }
    false
}
