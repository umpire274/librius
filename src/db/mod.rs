//! Database initialization utilities for Librius.
//!
//! This module provides helpers to initialize (or open) the SQLite database,
//! run schema migrations and perform book-related queries.

pub mod books;
pub mod connection;
pub mod migrations;

pub use books::{get_book_fields, search_books, update_book_by_id, update_book_by_isbn};
pub use connection::{ensure_schema, get_db_path, init_db, start_db};
pub use migrations::{MigrationResult, run_migrations};
