//! Database initialization utilities for Librius.
//!
//! This module provides a small helper to initialize (or open) the SQLite
//! database used by the application. The `init_db` function ensures the
//! required `books` table exists and returns an active `rusqlite::Connection`.
//!
//! The schema is intentionally simple and stores basic metadata for each
//! models (title, author, year, isbn and a timestamp when the record was
//! added).

pub mod books;
pub mod load_db;
pub mod migrate_db;
pub mod search;

pub use books::{get_book_fields, update_book_by_id, update_book_by_isbn};
pub use load_db::{ensure_schema, get_db_path, init_db, start_db};
pub use migrate_db::{MigrationResult, run_migrations};
pub use search::search_books;
