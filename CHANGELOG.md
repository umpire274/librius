# Changelog

All notable changes to this project will be documented in this file.

## [v0.1.1] - 2025-10-12
- 
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
