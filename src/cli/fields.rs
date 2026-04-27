// =====================================================
// Librius - cli/fields.rs
// -----------------------------------------------------
// Definizione dei campi editabili di un libro.
// Usata da args.rs (costruzione CLI) e da edit_book.rs
// (logica di comando). È una preoccupazione CLI, non
// una utility generica.
// =====================================================

/// Lista dei campi editabili di un libro.
/// Ogni elemento è una tupla (nome_campo, chiave_i18n, shortcut).
pub const EDITABLE_FIELDS: &[(&str, &str, char)] = &[
    ("title", "help.edit.book.title", 't'),
    ("author", "help.edit.book.author", 'a'),
    ("editor", "help.edit.book.editor", 'e'),
    ("year", "help.edit.book.year", 'y'),
    ("language_book", "help.edit.book.lang_book", 'b'),
    ("pages", "help.edit.book.pages", 'p'),
    ("genre", "help.edit.book.genre", 'g'),
    ("summary", "help.edit.book.summary", 's'),
    ("room", "help.edit.book.room", 'r'),
    ("shelf", "help.edit.book.shelf", 'f'),
    ("row", "help.edit.book.row", 'w'),
    ("position", "help.edit.book.position", 'o'),
];
