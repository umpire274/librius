use crate::i18n::tr;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize)]
pub struct Book {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub editor: String,
    pub year: i32,
    pub isbn: String,
    pub language: Option<String>,
    pub pages: Option<i32>,
    pub genre: Option<String>,
    pub summary: Option<String>,
    pub room: Option<String>,
    pub shelf: Option<String>,
    pub row: Option<String>,
    pub position: Option<String>,
    pub added_at: Option<DateTime<Utc>>,
}

pub struct BookFull<'a>(pub &'a Book);
pub struct BookShort<'a>(pub &'a Book);

impl<'a> Tabled for BookFull<'a> {
    const LENGTH: usize = 10;

    fn fields(&self) -> Vec<Cow<'_, str>> {
        let b = self.0;
        /*let added_date = b
        .added_at
        .as_ref()
        .map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "-".into());*/

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
