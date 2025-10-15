# Changelog

All notable changes to this project will be documented in this file.

## [0.2.4] - 2025-10-15

### Added

- Full internationalization (i18n) for CLI help, subcommands, and arguments.
- Embedded JSON language files (`en.json`, `it.json`) — no external dependencies required.
- Dynamic language selection via `--lang <code>` or the `language` key in `librius.conf` (YAML).

### Fixed

- Resolved duplicated `--help` and `--version` flag conflicts in Clap.
- Restored proper `--help` propagation for subcommands (`list`, `config`).
- Ensured fallback to English if an unknown language code is provided.

### Changed

- Pre-language bootstrap messages (e.g., “Load configuration...”) remain in English for clarity.
- Improved initialization order: configuration and DB migrations now run after language setup.

### Notes

- Default language remains English unless overridden by `--lang` or the `language` field in `librius.conf` (YAML).
- This version finalizes the transition to a fully localized CLI core.

---

## [v0.2.3] - 2025-10-14

### Added

- **Multilanguage support (i18n)**:
    - Added `i18n` module with `load_language()` and `tr()` functions.
    - Introduced translation files under `src/i18n/locales/` (`en.json`, `it.json`).
    - Added `src/i18n/locales/README.md` with key naming conventions.
    - CLI option `--lang` (or `-l`) allows overriding the language from configuration.
    - Config file can define a `language:` field for persistent preference.
    - Added `tr_with()` utility for runtime placeholder substitution (e.g., `{path}`, `{title}`).
- **Fallback system**: if a translation key is missing or the language file is unavailable, English (`en.json`) is used
  automatically.
- **Python helper script** (`scripts/extract_translations.py`):
    - Scans Rust source files for user-facing strings.
    - Updates `en.json` with any missing entries without overwriting existing ones.

### Changed

- All user-facing messages (`print_info`, `print_ok`, `print_err`, `println!`, etc.) are now translatable.
- Main startup sequence (`main.rs`) loads the selected language before configuration and database initialization.

### Improved

- Added helper in `config.rs` to read language preference directly from `librius.conf`.
- Enhanced verbosity filtering to respect localized messages.
- Clearer structure for future locale additions (fr, es, de, ...).

---

## [v0.2.2] - 2025-10-14

### Added

- **Structured database migration system**
    - Introduced `MigrationResult` enum replacing the previous boolean return value.
    - `run_migrations()` now returns explicit results (`Applied([...])` or `None`).
    - Each applied patch is now logged in the database with detailed information.
- **Patch safety improvements**
    - `PATCH_002` now checks for the existence of columns (`language`, `pages`, `genre`, `summary`) before adding them.
    - Prevents duplicate column errors on repeated runs.
- **Enhanced migration logging**
    - Migrations now record results under `DB_MIGRATION_OK`, `DB_MIGRATION_FAIL`, and `MIGRATIONS_COMPLETED` events in
      the `log` table.
    - Clear distinction between actual migrations and up-to-date states.

### Changed

- Refactored `run_migrations()` logic for clarity and maintainability.
- Updated database initialization flow (`start_db()`) to react dynamically to migration results.
- Improved internal output messages for migrations and startup consistency.

### Fixed

- Ensured safe re-execution of migration patches on existing databases.
- Removed redundant success messages when no migrations were needed.
- Unified patch logging behavior across all modules.

---

## [v0.2.1] - 2025-10-13

### Added

- **Database migrations**: introduced `db/migrate.rs` with incremental patch system and automatic execution at startup.
- **Configuration migrations**: added `config/migrate.rs` to automatically upgrade YAML configuration files.
- **Logging system**: new `utils::write_log()` function records database and migration events into the `log` table.
- **Verbose mode**: added global `--verbose` flag for diagnostic and debug output.
    - Normal mode: silent operation, only command results shown.
    - Verbose mode: shows configuration loading, database opening, and migration messages.
- **`db::start_db()`**: unified entry point that handles opening, creating, and migrating the database automatically.
- **Timestamp utility**: new `utils::now_str()` returns ISO 8601 formatted timestamps (`%+`).

### Changed

- Initialization messages are now hidden in normal mode.
- `main.rs` simplified: initialization logic moved into `db::start_db()`.
- `print_info()` and `print_ok()` now depend on verbose mode.

### Fixed

- Prevented redundant database initialization messages on repeated launches.
- Ensured migrations and configuration updates are idempotent and silent if up-to-date.

---

## [v0.2.0] - 2025-10-13

- feat(cli): add `config` command with `--print`, `--init`, `--edit`, and `--editor` options
- fix(windows): correctly handle editors with spaces in their path using `Path::new(&editor_to_use)`
- refactor(cli): move command and subcommand definitions into dedicated `cli.rs` module
- feat(utils): introduce `utils.rs` module with standard CLI icons and colored output helpers
- docs(readme): update structure and configuration examples
- style: minor formatting and consistency improvements across CLI output

---

## [v0.1.1] - 2025-10-12

- feat(models): add location fields to `Book` and `books` table schema (room, shelf, row, position)

## [v0.1.0] - 2025-10-12

- docs: add crate, module and item-level documentation for docs.rs
- feat(models): update `Book` struct to match `books` table schema (editor, language, pages, genre, summary, added_at)
- feat(models): change `added_at` type to `chrono::DateTime<Utc>` and enable `chrono` `serde` feature in `cargo.toml`
- feat(cli): make `list` handler return `Result` and handle errors in `main`
- feat(list): parse `added_at` from DB (RFC3339 / SQLite formats) and display `added_at` in the list as `YYYY-MM-DD`
- feat(crate): add `src/lib.rs` to expose crate API and improve docs generation on docs.rs
- docs: add docs.rs badge and local documentation instructions to `README.md`
- style: run `rustfmt` and fix Clippy warnings (code style and minor refactors)
