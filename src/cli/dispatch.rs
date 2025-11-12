use crate::cli::{Commands, build_cli};
use crate::i18n::tr;
use crate::utils::print_err;
use crate::{AppConfig, handle_config, handle_edit_book, handle_list, handle_search, tr_with};
use rusqlite::Connection;

/// Dispatch principale dei comandi
pub fn run_cli(
    config: &AppConfig,
    matches: &clap::ArgMatches,
    conn: &mut Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(matches) = matches.subcommand_matches("list") {
        let short = matches.get_flag("short");
        let id = matches.get_one::<i32>("id").copied();
        let details = matches.get_flag("details");
        handle_list(conn, short, id, details)?;
        Ok(())
    } else if let Some(("search", sub_m)) = matches.subcommand() {
        if let Some(query) = sub_m.get_one::<String>("query") {
            let short = sub_m.get_flag("short");
            handle_search(conn, query, short)?;
        } else {
            print_err(&tr("search_query_help"));
        }
        Ok(())
    } else if let Some(("config", sub_m)) = matches.subcommand() {
        let init = sub_m.get_flag("init");
        let print = sub_m.get_flag("print");
        let edit = sub_m.get_flag("edit");
        let editor = sub_m.get_one::<String>("editor").cloned();

        let cmd = Commands::Config {
            init,
            print,
            edit,
            editor,
        };
        Ok(handle_config(&cmd)?)
    } else if let Some(("db", sub_m)) = matches.subcommand() {
        let init = sub_m.get_flag("init");
        let reset = sub_m.get_flag("reset");
        let copy = sub_m.get_flag("copy");
        let file = sub_m.get_one::<String>("file").map(|s| s.as_str());

        crate::commands::db::handle_db(config, init, reset, copy, file)?;
        Ok(())
    } else if let Some(("edit", sub_m)) = matches.subcommand() {
        if let Some(("book", book_m)) = sub_m.subcommand() {
            handle_edit_book(conn, book_m)?; // ✅ integrazione comando edit book
        }
        Ok(())
    } else if let Some(("del", sub_m)) = matches.subcommand() {
        if let Some(key) = sub_m.get_one::<String>("key") {
            let force = sub_m.get_flag("force");
            crate::commands::handle_del_book(conn, key, force)?;
        }
        Ok(())
    } else if let Some(("backup", sub_m)) = matches.subcommand() {
        let compress = sub_m.get_flag("compress");
        crate::commands::handle_backup(conn, compress)?;
        Ok(())
    } else if let Some(("export", sub_m)) = matches.subcommand() {
        let output_path = sub_m.get_one::<String>("output").cloned();
        let export_csv = sub_m.get_flag("csv");
        let export_xlsx = sub_m.get_flag("xlsx");
        let export_json = sub_m.get_flag("json");

        if export_csv || (!export_xlsx && !export_json) {
            crate::commands::handle_export_csv(conn, output_path)?;
        } else if export_xlsx {
            crate::commands::handle_export_xlsx(conn, output_path)?;
        } else if export_json {
            crate::commands::handle_export_json(conn, output_path)?;
        }
        Ok(())
    } else if let Some(("import", sub_m)) = matches.subcommand() {
        let file_path = sub_m.get_one::<String>("file").cloned();
        if file_path.is_none() {
            print_err(&tr("import.error.missing_file"));
            return Ok(());
        }

        let file = file_path.unwrap();
        let import_json = sub_m.get_flag("json");
        let delimiter_char = sub_m
            .get_one::<String>("delimiter")
            .and_then(|s| s.chars().next())
            .unwrap_or(',');

        let result = if import_json {
            crate::commands::handle_import_json(conn, &file)
        } else {
            crate::commands::handle_import_csv(conn, &file, delimiter_char)
        };

        if let Err(e) = result {
            print_err(&tr_with(
                "import.error.unexpected",
                &[("error", &e.to_string())],
            ));
        }

        Ok(())
    } else if let Some(("add", sub_m)) = matches.subcommand() {
        if let Some(("book", book_m)) = sub_m.subcommand() {
            if let Some(isbn) = book_m.get_one::<String>("isbn") {
                crate::commands::handle_add_book(conn, isbn)?;
            } else {
                print_err(&tr("help.add.book.isbn"));
            }
        }
        Ok(())
    } else if let Some(("help", sub_m)) = matches.subcommand() {
        if let Some(cmd_name) = sub_m.get_one::<String>("command")
            && let Some(sc) = build_cli().find_subcommand(cmd_name)
        {
            sc.clone().print_help()?;
            println!();
            return Ok(());
        }
        build_cli().print_help()?;
        println!();
        Ok(())
    } else {
        build_cli().print_help()?;
        println!();
        Ok(())
    }
}
