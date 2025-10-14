#!/usr/bin/env python3
"""
Librius Translation Extractor

Scans all .rs source files under src/ and extracts user-facing strings
used in print_info, print_ok, print_err, print_warn, println!, and eprintln!.

Compares them against src/i18n/locales/en.json and appends any new entries.
"""

import os
import re
import json
from collections import OrderedDict

SRC_DIR = "src"
EN_JSON = os.path.join(SRC_DIR, "i18n", "locales", "en.json")

# Regex pattern to capture string literals inside print_*/println!/eprintln!
PATTERN = re.compile(
    r'(?:print_info|print_ok|print_err|print_warn|println!|eprintln!|panic!)\(\s*"([^"]+)"'
)

def scan_source_files():
    """Scan all .rs files and return a list of extracted strings."""
    found = set()
    for root, _, files in os.walk(SRC_DIR):
        for file in files:
            if not file.endswith(".rs"):
                continue
            path = os.path.join(root, file)
            with open(path, "r", encoding="utf-8") as f:
                for i, line in enumerate(f, start=1):
                    for match in PATTERN.findall(line):
                        match = match.strip()
                        if len(match) > 0:
                            found.add(match)
    return sorted(found)

def load_existing_translations():
    """Load current en.json file, or return empty dict if missing."""
    if not os.path.exists(EN_JSON):
        return OrderedDict()
    with open(EN_JSON, "r", encoding="utf-8") as f:
        try:
            return json.load(f, object_pairs_hook=OrderedDict)
        except json.JSONDecodeError:
            print("⚠️  Warning: en.json is not valid JSON, starting fresh.")
            return OrderedDict()

def generate_key(base_index, text):
    """Generate a generic key name for a new translation entry."""
    # example: auto.msg.001
    return f"auto.msg.{base_index:03d}"

def update_translations(existing, extracted):
    """Add new extracted strings to existing translations if missing."""
    current_values = set(existing.values())
    next_index = len(existing) + 1

    for text in extracted:
        if text not in current_values:
            key = generate_key(next_index, text)
            existing[key] = text
            next_index += 1

    return existing

def save_translations(translations):
    """Write the updated en.json file."""
    os.makedirs(os.path.dirname(EN_JSON), exist_ok=True)
    with open(EN_JSON, "w", encoding="utf-8") as f:
        json.dump(translations, f, indent=4, ensure_ascii=False)
        f.write("\n")

def main():
    print("🔍 Scanning Rust source files for translatable strings...")
    extracted = scan_source_files()
    print(f"   → Found {len(extracted)} unique strings")

    existing = load_existing_translations()
    before = len(existing)

    updated = update_translations(existing, extracted)
    added = len(updated) - before

    save_translations(updated)

    print(f"✅ Done. Added {added} new strings to {EN_JSON}")
    print(f"   Total keys: {len(updated)}")

if __name__ == "__main__":
    main()
