use crate::commands::add_book::handle_add_book;
use crate::i18n::tr;
use clap::ArgMatches;
use rusqlite::Connection;

pub fn handle_add(
    conn: &Connection,
    matches: &ArgMatches,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(("book", sub_m)) = matches.subcommand() {
        let isbn = sub_m.get_one::<String>("isbn").unwrap();
        handle_add_book(conn, isbn)?;
    } else {
        println!("{}", tr("help.add.usage"));
    }
    Ok(())
}
