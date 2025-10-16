# 🌍 Librius — i18n Locales Guide

This directory contains all translation files used by **Librius**  
to display localized messages in the CLI.

---

## 🧩 File structure

src/i18n/locales/
├── en.json ← English (default)
├── it.json ← Italian
└── README.md ← This file

Each JSON file contains a flat JSON object of translation keys → values.

Example:

```json
{
  "app.config.loading": "Loading configuration...",
  "db.init.ok": "Database created successfully.",
  "book.add.fail": "Error adding models '{title}': {error}"
}
```

---

## 🧭 Key naming convention

Keys follow the pattern:

```plain
<module>.<submodule>.<message>
```

### Examples

| Scope  | 	Area     | 	Action	 | Key	                | Description                       |
|--------|-----------|----------|---------------------|-----------------------------------|
| app    | 	config   | 	loading | 	app.config.loading | 	App-wide                         | configuration message|
| config | verified  | —        | config.verified     | Config file successfully loaded   |
| db     | init      | ok       | db.init.ok          | Database initialized successfully |
| db     | migrate   | fail     | db.migrate.fail     | Database migration failed         |
| book   | add       | ok       | book.add.ok         | Book                              | added successfully|
| book   | add       | fail     | book.add.fail       | Book addition failed              |
| cli    | args      | invalid  | cli.args.invalid    | Invalid CLI arguments             |
| error  | db        | —        | error.db            | Database-related error            |
| warn   | migration | —        | warn.migration      | Database migration warning        |

---

## 🏗️ Scopes and areas

| Scope   | 	Typical areas	               | Description                       |
|---------|-------------------------------|-----------------------------------|
| app	    | config, start, exit	          | Application-level messages        |
| config  | 	load, edit, verify	          | YAML configuration operations     |
| db      | 	path, init, migrate, schema	 | Database structure and migrations |
| book	   | add, remove, search	          | Book-level operations             |
| cli     | 	args, commands, help	        | CLI and argument parsing          |
| error   | 	db, config, io               | 	Error messages                   |
| warn	   | migration, path, version      | 	Warnings and fallback messages   |
| info    | update, version, usage        | Informational messages            |
| user    | auth, profile, settings       | User-related messages             |
| network | connect, timeout, request     | Network-related messages          |
| cache   | clear, load, save             | Cache operations                  |
| sync    | start, stop, status           | Synchronization operations        |
| help    | commands, usage, examples     | Help and usage messages           |
| log     | write, read, rotate           | Logging operations                |
| ui      | render, update, error         | User interface messages           |
| notif   | send, receive, error          | Notification messages             |
| file    | read, write, delete           | File operations                   |
| auth    | login, logout, register       | Authentication messages           |
| report  | generate, send, error         | Reporting operations              |
| stats   | calculate, display, error     | Statistics-related messages       |

---

## 🔤 Variables in translations

Translations can include placeholders wrapped in braces {}.

Examples:

```json
{
  "db.path.open_existing": "Opening existing database at: {path}",
  "config.edit.open_with": "Opening configuration file with '{editor}'",
  "book.add.ok": "Book '{title}' added successfully!"
}
```

Use tr_with("db.path.open_existing", &[("path", db_path)]) to substitute variables at runtime.

---

## 🧠 Notes

- Emojis and icons should not be included in translated strings — they are handled separately by utils::print_*.
- Missing keys fall back to their raw key name.
- English (en.json) acts as the fallback language.
- To add a new language:
    1. Copy en.json → fr.json (or another code),
    2. Translate values only,
    3. Keep keys identical.

---

## 🧩 Maintainers note

Run the helper script `extract_translations.py` (optional) to regenerate a list of untranslated strings for new modules.