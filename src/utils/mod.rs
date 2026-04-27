// =====================================================
// Librius - utils/mod.rs
// -----------------------------------------------------
// Modulo aggregatore: dichiara i sottomoduli e
// ri-esporta i simboli pubblici in modo esplicito.
// =====================================================

pub mod import_helpers;
pub mod isbn;
pub mod lang;
pub mod log;
pub mod print;
pub mod table;
pub mod verbose;

// --- re-export espliciti ---

// verbose
pub use verbose::{is_verbose, set_verbose};

// print
pub use print::{icons, print_err, print_info, print_ok, print_warn};

// log
pub use log::{now_str, write_log};

// import helpers
pub use import_helpers::{handle_import_result, open_import_file};

// lang
pub use lang::lang_code_to_name;

// table
pub use table::{build_table, build_vertical_table};
