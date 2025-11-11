<h1 style="text-align: left;">
  <img src="res/librius.svg" width="90" style="vertical-align: middle; margin-right: 8px;" alt="Librius Logo"/>
  Librius
</h1>

[![Build Status](https://github.com/umpire274/librius/actions/workflows/ci.yml/badge.svg)](https://github.com/umpire274/librius/actions)
[![Docs.rs](https://docs.rs/librius/badge.svg)](https://docs.rs/librius)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Rust Edition](https://img.shields.io/badge/Rust-2024-orange.svg)](https://www.rust-lang.org/)

> **Librius** — A fast, minimalist CLI to manage your personal book collection, built in Rust.

---

## 🧾 Overview

**Librius** is a cross-platform **command-line tool** written in Rust that helps you manage your personal library.  
It uses a **SQLite** database to store your books and a simple **YAML** configuration file
(`librius.conf`) for flexible setup.

This project aims to provide a clean, modular architecture with future extensions such as search, add/remove commands,
and import/export support.

---

### ✨ New in v0.4.5

**🔍 Full-text book search**

- Introduced the new `search` command:
  ```bash
  librius search <query> [--short]
  ```
  #### Example usage:
  ```bash
  $ librius search "dune"
  $ librius search "frank herbert" --short
  ```
- Performs full-text lookup across **title, author, editor, genre, and language** fields.
- Supports both **compact** (`--short`) and **full** table views.
- Uses the same localized message system (`tr()`) as the rest of the CLI.
- Unified output style with `print_info`, `print_ok`, and `print_warn` for consistent visual feedback.

---

## 📦 Installation

### 🐧 AUR (Arch Linux)

[![AUR](https://img.shields.io/aur/version/librius)](https://aur.archlinux.org/packages/librius)

```bash
yay -S librius
# or
paru -S librius
```

### 🍺 Homebrew (macOS/Linux)

[![Homebrew Version](https://img.shields.io/github/v/release/umpire274/librius?label=Homebrew&logo=homebrew&color=brightgreen)](https://github.com/umpire274/homebrew-tap)

```bash
brew tap umpire274/tap
brew install librius
```

### 🦀 Crates.io (Rust)

[![Crates.io](https://img.shields.io/crates/v/librius)](https://crates.io/crates/librius)

```bash
cargo install rtimelogger
```

---

## ⚙️ Features

| Feature                  | Command                          | Description                                                                                                    |
|:-------------------------|:---------------------------------|:---------------------------------------------------------------------------------------------------------------|
| **List**                 | `librius list`                   | Display all books stored in the local database, in full or compact view                                        |
| **Search**               | `librius search <query>`         | Full-text search across title, author, editor, genre, and language fields; supports `--short` for compact view |
| **Config management**    | `librius config`                 | Manage YAML configuration via `--print`, `--init`, `--edit`, `--editor`                                        |
| **Backup**               | `librius backup`                 | Create plain or compressed database backups (`.sqlite`, `.zip`, `.tar.gz`)                                     |
| **Export**               | `librius export`                 | Export data in CSV, JSON, or XLSX format                                                                       |
| **Import**               | `librius import`                 | Import data from CSV or JSON files (duplicate-safe via ISBN)                                                   |
| **Database migrations**  | *(automatic)*                    | Automatic schema upgrades and integrity checks at startup                                                      |
| **Logging system**       | *(internal)*                     | Records all operations and migrations in an internal log table                                                 |
| **Multilanguage (i18n)** | `librius --lang <code>`          | Fully localized CLI (commands, help, messages); `--lang` flag and config key                                   |
| **Add book**             | `librius add book --isbn <ISBN>` | Add new books using ISBN lookup via Google Books API                                                           |
| **Edit book**            | `librius edit book <ID/ISBN>`    | Edit existing records by ID or ISBN; dynamic field generation, language conversion, and plural-aware messages  |
| **Delete book**          | `del <ID/ISBN>`                  | Delete books by ID or ISBN, with interactive confirmation, `--force` flag, and logged deletions                |
| **Dynamic help system**  | `librius help <command>`         | Ordered and grouped help output using `display_order()` and `next_help_heading()`                              |

---

## 📖 Commands Overview

### 📘 list

List all books or a specific book by ID.

```bash
$ librius list [--short] [--id <ID>] [--details]
```

**Options**:

- `--short` Compact view
- `--id` Show book by ID
- `--details` Show extended metadata

### 🔍 search

Search for books by title, author, editor, genre, or language.

```bash
$ librius search <query> [--short]
```

**Options**:

- `--short` Show compact view (ID, Title, Author, Editor, Year, ISBN)
- `<query>` Search term
- `--help` Show command help

### ➕ add book

Add a new book using its ISBN.

```bash
$ librius add book --isbn <ISBN>
```

**Options**:

- `--isbn <ISBN>` ISBN of the book to add
- `--help` Show command help

### ✏️ edit book

Edit an existing book by ID or ISBN.

```bash
$ librius edit book <ID/ISBN> [--title <TITLE>] [--author <AUTHOR>] [--editor <EDITOR>] [--year <YEAR>] [--genre <GENRE>] [--language <LANGUAGE>] [--isbn <ISBN>]
```

**Options**:

- `<ID/ISBN>` ID or ISBN of the book to edit
- `--title <TITLE>` New title
- `--author <AUTHOR>` New author
- `--editor <EDITOR>` New editor
- `--year <YEAR>` New publication year
- `--genre <GENRE>` New genre
- `--language <LANGUAGE>` New language
- `--isbn <ISBN>` New ISBN
- `--help` Show command help

### ❌ delete book

Delete a book by ID or ISBN.

```bash
$ librius del <ID/ISBN> [--force]
```

**Options**:

- `<ID/ISBN>` ID or ISBN of the book to delete
- `--force` Skip confirmation prompt
- `--help` Show command help

### ⚙️ config

Manage application configuration.

```bash
$ librius config [--print] [--init] [--edit] [--editor <EDITOR>]
```

**Options**:

- `--print` Print current configuration
- `--init` Create default config file
- `--edit` Open config file in editor
- `--editor <EDITOR>` Specify editor (default: `$EDITOR` or `nano`

### 💾 backup

Create a backup of the database.

```bash
$ librius backup [--compress]
``` 

**Options**:

- `--compress` Create a compressed backup (`.zip` or `.tar.gz`)
- `--help` Show command help

### 📤 export

Export library data to CSV, JSON, or XLSX.

```bash
$ librius export [--csv | --json | --xlsx] [-o|--output <FILE>]
```

**Options**:

- `--csv` Export as CSV (default)
- `--json` Export as JSON
- `--xlsx` Export as XLSX
- `-o, --output <FILE>` Specify output file path
- `--help` Show command help

### 📥 import

Import library data from CSV or JSON.

```bash
$ librius import --file <FILE> [--json] [--csv] [-d|--delimiter <CHAR>]
```

**Options**:

- `--file <FILE>` Path to input file
- `--json` Specify if the input file is JSON (default is CSV)
- `--csv` Specify if the input file is CSV
- `-d, --delimiter <CHAR>` Specify CSV delimiter (default: `,`)
- `--help` Show command help

### 🧠 Note

- Every command is fully **localized** in english (default) and italian.

---

## 🌍 Multilanguage support (i18n)

Librius now supports a multilingual interface.

| Source       | 	Description                                                    |
|--------------|-----------------------------------------------------------------|
| 🇬🇧 en.json | 	Default English messages                                       |
| 🇮🇹 it.json | 	Italian translation                                            |
| 📄 README.md | 	Located in src/i18n/locales/, describes key naming conventions |

### How it works

- On startup, Librius loads the language defined in:
    1. CLI argument --lang / -l
    2. Configuration file language:
    3. Fallback to English (en)
- All user-visible messages (print_info, print_err, etc.) are translated dynamically.
- Missing keys automatically fall back to their key name or English equivalent.

### Example Usage

```bash
# Default (English)
librius list

# Force Italian interface
librius --lang it list
```

### Example config (`librius.conf`)

```yaml
# librius.conf
database: "C:/Users/YourName/AppData/Roaming/librius/librius.sqlite"
language: "it"  # Set default language to Italian
```

---

## 🧩 Translations

All translations are stored in:

```bash
src/i18n/locales/
├── en.json
├── it.json
└── README.md
```

Each `.json` file contains key–value pairs like:

```json
{
  "app.config.loading": "Loading configuration...",
  "db.init.ok": "Database created successfully.",
  "book.add.ok": "Book '{title}' added successfully!"
}
```

Variables can be inserted at runtime:

```rust
tr_with("db.path.open_existing", & [("path", & db_path)]);
```

---

## 🧱 Project structure

```
src/
├─ main.rs
├─ lib.rs
├─ cli.rs
│
├─ commands/
│   ├─ mod.rs
│   ├─ list.rs
│   ├─ backup.rs
│   ├─ config.rs
│   ├─ export.rs
│   └─ import.rs
│
├─ config/
│   ├─ mod.rs
│   ├─ load_config.rs
│   └─ migrate_config.rs
│
├─ db/
│   ├─ mod.rs
│   ├─ load_db.rs
│   └─ migrate_db.rs
│
├─ i18n/
│   ├─ mod.rs
│   ├─ loader.rs
│   └─ locales/
│       ├─ en.json
│       ├─ it.json
│       └─ README.md
│
├─ models/
│   ├─ mod.rs
│   └─ book.rs
│
└─ utils/
    ├─ mod.rs
    └─ table.rs
```

---

## 🧾 Changelog reference

See [CHANGELOG.md](CHANGELOG.md) for a detailed list of changes and updates.

---

## 📤 Export & 📥 Import

Librius now supports full data import/export functionality.

### Export

You can export your library to multiple formats:

```bash
librius export --csv     # CSV (default)
librius export --json    # JSON
librius export --xlsx    # Excel (XLSX)
```

Exports are automatically saved in your user data directory
(e.g. `~/.config/librius/exports` or `%APPDATA%\librius\exports`).

### Import

Import books from CSV or JSON files:

```bash
librius import --file examples/books.csv
librius import --file examples/books.json --json
```

Features:

- Automatic detection of duplicate records via unique `isbn`
- Skips duplicates gracefully (no interruption)
- Transaction-safe import
- Verbose mode logs skipped ISBNs

Example output:

```bash
📘  Skipped duplicate ISBN: 978-0-345-33968-3
✅ Imported 6 records from CSV file.
```

---

## 🚀 Quick start

### 1️⃣ Clone the repository

```bash
git clone https://github.com/umpire274/librius.git
cd librius
```

### 2️⃣ Build and run

```bash
cargo build
cargo run -- list
```

If this is the first launch, Librius will automatically create:

- The config file at `~/.config/librius/librius.conf`
- A SQLite database at `~/.config/librius/librius.sqlite`

---

## 🧩 Example output

$ librius list

📚 Your Library

1. The Hobbit (J.R.R. Tolkien) [1937]
2. Foundation (Isaac Asimov) [1951]
3. Dune (Frank Herbert) [1965]

---

## ⚙️ Configuration

```yaml
# librius.conf
database: "C:/Users/YourName/AppData/Roaming/librius/librius.sqlite"
language: "en"
```

- Configuration file is automatically migrated if fields are missing or renamed.
- Default path:
    - macOS/Linux → $HOME/.librius/librius.conf
    - Windows → %APPDATA%\Roaming\librius\librius.conf

---

## 🧩 Development notes

Librius now follows a standard Rust modular structure:

- Each domain (commands, db, config, models, utils, i18n) exposes its API via mod.rs.
- Common utilities like build_table() are reused across commands for consistent output.
- The lib.rs re-exports all major modules for cleaner imports in main.rs.

### Example import

```rust
use librius::{build_cli, handle_list; tr};
```

---

## 📚 Documentation

The API and user-facing documentation for Librius is available on docs.rs:

- Online: https://docs.rs/librius

To generate and view the documentation locally run:

```bash
cargo doc --no-deps --open
```

This will build the documentation and open it in your default browser.

---

## 🧰 Dependencies

- **clap** — Command-line argument parsing
- **rusqlite** — SQLite database integration
- **serde / serde_json** — Serialization/deserialization
- **serde_yaml** — YAML config parsing
- **umya-spreadsheet** — XLSX file creation
- **csv** — CSV import/export
- **colored** — Colored terminal output
- **chrono** — Date and time utilities

---

## 🗄️ Database management

Librius automatically verifies and upgrades the SQLite database schema at startup.

The latest migration adds a unique index on `isbn` to guarantee
that duplicate imports are ignored safely.

```sql
CREATE UNIQUE INDEX IF NOT EXISTS idx_books_isbn ON books(isbn);
```

- On first launch → creates books table.
- On subsequent launches → runs pending migrations silently.
- Migration results are recorded in the log table.

Each migration patch (`PATCH_001`, `PATCH_002`, …) is applied once and recorded in the internal log table.
The process is fully idempotent — no duplicate operations are ever performed.

```pgsql
📘  Applying database patch: PATCH_002
✅  All pending migrations applied.
✅  Database schema is up-to-date.
```

### Example table `log`

| id | date                      | operation       | target | message                     |
|----|---------------------------|-----------------|--------|-----------------------------|
| 1  | 2025-10-13T21:45:12+02:00 | DB_CREATED      | DB     | Created new database        |
| 2  | 2025-10-13T21:45:13+02:00 | DB_MIGRATION_OK | DB     | Schema updated successfully |

---

🔍 Verbose mode

Run Librius in diagnostic mode to display all internal initialization steps:

```bash
librius --verbose list
```

Output example:

```bash
📘  Loading configuration...
📘  Opening existing database at: C:\Users\A.Maestri\AppData\Roaming\librius\librius.db
✅ Database schema is up-to-date.
✅ Configuration verified.

📚 Your Library
```

In normal mode, only command output is displayed.

---

## 🧪 Development

### Run in debug mode

```bash
cargo run -- list
```

### Run with logging (future)

```bash
RUST_LOG=debug cargo run -- add "Neuromancer"
```

### Format and lint

```bash
cargo fmt
cargo clippy
```

---

## 🧱 Future roadmap

- Add `add`, `remove`, and `search` commands
- Export/import JSON and CSV
- Add optional TUI (Text UI) with `ratatui`
- Web dashboard sync

---

## 🧑‍💻 Author

Umpire274
GitHub: [@umpire274](https://github.com/umpire274)

---

## 📜 License

This project is licensed under the MIT License — see the LICENSE

---

## ⭐ Contribute

Contributions, feature requests, and ideas are welcome!
If you’d like to contribute, please open a pull request or start a discussion.

---

## 🧡 Support

If you enjoy this project, please ⭐ star the repository — it helps visibility and development motivation!

---

