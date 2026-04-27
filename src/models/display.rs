// =====================================================
// Librius - models/display.rs
// -----------------------------------------------------
// Wrapper di presentazione per il tipo Book.
// Separa la logica di visualizzazione (Tabled + i18n)
// dal modello dati puro definito in book.rs.
// =====================================================

use crate::i18n::tr;
use crate::models::book::Book;
use std::borrow::Cow;
use tabled::Tabled;

/// Vista completa del libro (10 colonne) per la tabella `list`.
pub struct BookFull<'a>(pub &'a Book);

/// Vista ridotta del libro (6 colonne) per `list --short`.
pub struct BookShort<'a>(pub &'a Book);

impl<'a> Tabled for BookFull<'a> {
    const LENGTH: usize = 10;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        let b = self.0;
        vec![
            Cow::from(b.id.map(|v| v.to_string()).unwrap_or_default()),
            Cow::from(&b.title),
            Cow::from(&b.author),
            Cow::from(&b.editor),
            Cow::from(b.year.to_string()),
            Cow::from(b.isbn.to_string()),
            Cow::from(b.language.as_deref().unwrap_or("-")),
            Cow::from(b.room.as_deref().unwrap_or("-")),
            Cow::from(b.shelf.as_deref().unwrap_or("-")),
            Cow::from(b.position.as_deref().unwrap_or("-")),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::from(tr("list.header.id")),
            Cow::from(tr("list.header.title")),
            Cow::from(tr("list.header.author")),
            Cow::from(tr("list.header.editor")),
            Cow::from(tr("list.header.year")),
            Cow::from(tr("list.header.ISBN")),
            Cow::from(tr("list.header.language")),
            Cow::from(tr("list.header.room")),
            Cow::from(tr("list.header.shelf")),
            Cow::from(tr("list.header.position")),
        ]
    }
}

impl<'a> Tabled for BookShort<'a> {
    const LENGTH: usize = 6;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        let b = self.0;
        vec![
            Cow::from(b.id.map(|v| v.to_string()).unwrap_or_default()),
            Cow::from(&b.title),
            Cow::from(&b.author),
            Cow::from(&b.editor),
            Cow::from(b.year.to_string()),
            Cow::from(b.isbn.to_string()),
        ]
    }

    fn headers() -> Vec<Cow<'static, str>> {
        vec![
            Cow::from(tr("list.header.id")),
            Cow::from(tr("list.header.title")),
            Cow::from(tr("list.header.author")),
            Cow::from(tr("list.header.editor")),
            Cow::from(tr("list.header.year")),
            Cow::from(tr("list.header.ISBN")),
        ]
    }
}

