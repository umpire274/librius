use crate::i18n::tr;
use crate::models::book::Book;
use crate::utils::lang_code_to_name;
use crate::{is_verbose, print_err, print_info, print_ok, print_warn, tr_with};
use chrono::Utc;
use reqwest::blocking::get;
use rusqlite::Connection;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct GoogleBooksResponse {
    items: Option<Vec<GoogleBookItem>>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default)]
struct GoogleBookItem {
    #[serde(rename = "volumeInfo")]
    volume_info: VolumeInfo,
}

#[derive(Debug, Deserialize, Default)]
#[serde(default, rename_all = "camelCase")]
struct VolumeInfo {
    title: Option<String>,
    authors: Option<Vec<String>>,
    publisher: Option<String>,
    published_date: Option<String>,
    description: Option<String>,
    page_count: Option<i32>,
    language: Option<String>,
    categories: Option<Vec<String>>,
}

pub fn handle_add_book(conn: &Connection, isbn: &str) -> Result<(), Box<dyn Error>> {
    println!("\n{} {}", tr("add.lookup"), isbn);

    let url = format!("https://www.googleapis.com/books/v1/volumes?q=isbn:{isbn}");
    let resp = get(&url)?;

    if !resp.status().is_success() {
        print_err(&tr_with(
            "book.add.http_error",
            &[("status", &resp.status().to_string())],
        ));
        return Ok(());
    }

    let text = resp.text()?;
    let response: Result<GoogleBooksResponse, _> = serde_json::from_str(&text);

    match response {
        Ok(data) => {
            if let Some(items) = data.items {
                let info = &items[0].volume_info;

                // 🧩 Debug temporaneo per vedere i dati ricevuti
                let debug_info = format!("{:#?}", info);
                print_info(
                    &tr_with("book.add.book_info", &[("info", &debug_info)]),
                    is_verbose(),
                );

                let new_book = Book {
                    id: Some(0),
                    title: info.title.clone().unwrap_or_default(),
                    author: info
                        .authors
                        .as_ref()
                        .map(|a| a.join(", "))
                        .unwrap_or_default(),
                    editor: info.publisher.clone().unwrap_or_default(),
                    year: info
                        .published_date
                        .as_ref()
                        .and_then(|d| d.get(0..4))
                        .and_then(|y| y.parse::<i32>().ok())
                        .unwrap_or_default(),
                    isbn: isbn.to_string(),
                    language: info
                        .language
                        .as_ref()
                        .map(|c| lang_code_to_name(c).to_string()),
                    pages: info.page_count,
                    genre: info.categories.as_ref().map(|c| c.join(", ")),
                    summary: info.description.clone(),
                    room: None,
                    shelf: None,
                    row: None,
                    position: None,
                    added_at: Some(Utc::now()),
                };

                conn.execute(
                    "INSERT INTO books (title, author, editor, year, isbn, language, pages, genre, summary, added_at)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, CURRENT_TIMESTAMP)",
                    rusqlite::params![
                        new_book.title,
                        new_book.author,
                        new_book.editor,
                        new_book.year,
                        new_book.isbn,
                        new_book.language,
                        new_book.pages,
                        new_book.genre,
                        new_book.summary,
                    ],
                )?;

                print_ok(&tr_with("add.success", &[("title", &new_book.title)]), true);
            } else {
                print_warn(&tr("add.no_result"));
            }
        }
        Err(e) => {
            print_err(&tr_with("add.decode_error", &[("error", &e.to_string())]));
            // eprintln!("Raw JSON:\n{}", text);
        }
    }

    Ok(())
}
