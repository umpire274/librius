# Librius — Project Structure

> Last updated after full refactor (v0.6.0).
> Each file has a single, well-defined responsibility.

---

## Root layout

```
librius/
├── Cargo.toml          # crate metadata and dependencies
├── Cargo.lock
├── build.rs            # Windows resource embedding (winresource)
├── README.md
├── CHANGELOG.md
├── STRUCTURE.md        # this file
├── LICENSE
├── src/                # application source (see below)
├── tests/              # integration tests
├── res/                # Windows icon / resource files
├── scripts/            # helper scripts (e.g. extract_translations.py)
├── dev_tools/          # build-check and icon-generation scripts
└── tools/              # submodule-check helpers
```

---

## `src/` — Source tree

```
src/
├── main.rs             # binary entry-point: parse args, load config, init DB, dispatch CLI
├── lib.rs              # library root: declares all modules, exposes explicit public API
│
├── cli/                # CLI parsing and command dispatch
│   ├── mod.rs          # re-exports: build_cli, run_cli, EDITABLE_FIELDS
│   ├── args.rs         # builds the full clap::Command tree (localised with tr_s)
│   ├── dispatch.rs     # matches subcommands → calls command handlers
│   └── fields.rs       # EDITABLE_FIELDS: list of book fields editable via CLI
│
├── commands/           # one handler per user-facing command
│   ├── mod.rs          # re-exports all handle_* functions
│   ├── add_book.rs     # handle_add_book — fetches metadata via Google Books API
│   ├── backup.rs       # handle_backup — ZIP/tar database backup
│   ├── config.rs       # handle_config — init / print / edit config file
│   ├── db.rs           # handle_db — DB init, reset, copy
│   ├── del_book.rs     # handle_del_book — delete by ID or ISBN
│   ├── edit_book.rs    # handle_edit_book — update one or more fields
│   ├── export.rs       # handle_export_csv/xlsx/json
│   ├── import.rs       # handle_import_csv/json
│   ├── list.rs         # handle_list — tabular list with optional detail view
│   └── search_book.rs  # handle_search — full-text search across key fields
│
├── config/             # application configuration (YAML)
│   ├── mod.rs          # re-exports: AppConfig, load_or_init, config_file_path, migrate_config
│   ├── load_config.rs  # AppConfig struct, YAML load/save, default path resolution
│   └── migrate_config.rs # config schema migration (adds missing keys to existing files)
│
├── db/                 # SQLite database layer
│   ├── mod.rs          # re-exports: start_db, init_db, ensure_schema, run_migrations,
│   │                   #             search_books, get_book_fields, update_book_by_id/isbn
│   ├── connection.rs   # DB path resolution, connection open, schema init, migration dispatch
│   ├── migrations.rs   # incremental patch system (PATCH_001..N), MigrationResult enum
│   └── books.rs        # CRUD helpers: update_book_by_id/isbn, get_book_fields, search_books
│
├── i18n/               # internationalisation
│   ├── mod.rs          # re-exports: load_language, tr, tr_s, tr_with, parse_json_to_map
│   └── loader.rs       # JSON translation loader, global language map, tr/tr_s/tr_with helpers
│
├── models/             # domain models
│   ├── mod.rs          # re-exports: Book, BookFull, BookShort
│   ├── book.rs         # Book struct (pure data + Serde + from_row) — no i18n / tabled deps
│   └── display.rs      # BookFull, BookShort — Tabled wrappers with localised column headers
│
└── utils/              # generic utilities (one file per concern)
    ├── mod.rs          # aggregator: declares all submodules, explicit re-exports
    ├── verbose.rs      # VERBOSE global flag: set_verbose(), is_verbose()
    ├── print.rs        # icons module (OK/ERR/WARN/INFO) + print_ok/err/warn/info()
    ├── log.rs          # now_str(), write_log() — structured SQLite log entries
    ├── import_helpers.rs # open_import_file(), handle_import_result()
    ├── isbn.rs         # normalize_isbn() — validation + hyphen formatting (isbn2 crate)
    ├── lang.rs         # lang_code_to_name() — ISO 639-1 code → readable name
    └── table.rs        # build_table(), build_vertical_table() — tabled rendering helpers
```

---

## `tests/` — Integration tests

```
tests/
├── common.rs               # shared test helpers (DB setup, temp paths)
├── db_tests.rs             # schema creation, insert + read round-trips
├── isbn_tests.rs           # normalize_isbn: plain, hyphenated, invalid inputs
└── librius_core_tests.rs   # handle_list / handle_list --short end-to-end
```

---

## Key design rules (post-refactor)

| Rule | Where enforced |
|------|---------------|
| **Single responsibility** | Each `.rs` file contains one cohesive concept |
| **No glob re-exports** | `lib.rs` only re-exports the explicit public API |
| **Explicit imports** | All internal modules use full `crate::x::y` paths — no shortcut from crate root |
| **No dead code** | Wrapper modules removed; duplicate functions eliminated |
| **Presentation ≠ data** | `models/book.rs` (data) vs `models/display.rs` (tabled + i18n) |
| **CLI concerns stay in `cli/`** | `EDITABLE_FIELDS` lives in `cli/fields.rs`, not `utils/` |
| **DB migrations run once** | `start_db()` (`db/connection.rs`) runs migrations; `main.rs` does not repeat them |

---

## Public API surface (`lib.rs` re-exports)

```rust
// config
pub use config::{AppConfig, load_or_init};

// db
pub use db::{init_db, start_db};

// models
pub use models::Book;

// i18n
pub use i18n::{load_language, tr, tr_s, tr_with};
```

All other symbols are accessible via their full module path (e.g.
`librius::utils::isbn::normalize_isbn`, `librius::commands::handle_list`).

