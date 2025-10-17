use crate::commands::{handle_config, handle_list};
use crate::i18n::{tr, tr_s};
use crate::tr_with;
use crate::utils::print_err;
use clap::{Arg, Command, Subcommand};
use rusqlite::Connection;

/// Costruisce la CLI localizzata usando le stringhe già caricate in memoria.
pub fn build_cli() -> Command {
    // Disabilitiamo help/subcommand automatici per poter localizzare/spiegare noi
    Command::new(tr_s("app_name"))
        .version(env!("CARGO_PKG_VERSION"))
        .about(tr_s("app_about"))
        .disable_help_flag(true)
        .disable_help_subcommand(true)
        .arg(
            Arg::new("help")
                .short('h')
                .long("help")
                .help(tr_s("help_flag_about"))
                .action(clap::ArgAction::Help)
                .global(true),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .global(true)
                .help(tr_s("help_verbose"))
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .global(true)
                .num_args(1)
                .help(tr_s("help_lang")),
        )
        .subcommand(
            Command::new("list")
                .about(tr_s("list_about"))
                .arg(
                    Arg::new("short")
                        .long("short")
                        .help(tr_s("help.list.short"))
                        .action(clap::ArgAction::SetTrue)
                        .num_args(0),
                )
                .arg(
                    Arg::new("id")
                        .long("id")
                        .help(tr_s("help.list.id")) // es: "Specify the record ID to show"
                        .value_name("ID")
                        .num_args(1)
                        .value_parser(clap::value_parser!(i32)),
                )
                .arg(
                    Arg::new("details")
                        .long("details")
                        .help(tr_s("help.list.details")) // es: "Show all fields of the specified record (requires --id)"
                        .action(clap::ArgAction::SetTrue)
                        .num_args(0),
                ),
        )
        .subcommand(
            Command::new("config")
                .about(tr_s("config_about"))
                .arg(
                    Arg::new("init")
                        .long("init")
                        .help(tr_s("config_init_help"))
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("print")
                        .long("print")
                        .help(tr_s("config_print_help"))
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("edit")
                        .long("edit")
                        .help(tr_s("config_edit_help"))
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("editor")
                        .long("editor")
                        .requires("edit")
                        .num_args(1)
                        .help(tr_s("config_editor_help")),
                ),
        )
        .subcommand(
            Command::new("backup").about(tr_s("backup_about")).arg(
                Arg::new("compress")
                    .long("compress")
                    .help(tr_s("backup_compress_help"))
                    .action(clap::ArgAction::SetTrue),
            ),
        )
        .subcommand(
            Command::new("export")
                .about(tr_s("export_about"))
                .arg(
                    Arg::new("csv")
                        .long("csv")
                        .help(tr_s("export_csv_help"))
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("xlsx")
                        .long("xlsx")
                        .help(tr_s("export_xlsx_help"))
                        .conflicts_with_all(["csv", "json"])
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help(tr_s("export_json_help"))
                        .conflicts_with_all(["csv", "xlsx"])
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help(tr_s("export_output_help"))
                        .value_name("FILE")
                        .required(false),
                ),
        )
        .subcommand(
            Command::new("import")
                .about(tr_s("import_about"))
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .help(tr_s("import_file_help"))
                        .required(true)
                        .value_name("PATH"),
                )
                .arg(
                    Arg::new("csv")
                        .long("csv")
                        .help(tr_s("import_csv_help"))
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help(tr_s("import_json_help"))
                        .conflicts_with("csv")
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("delimiter")
                        .short('d')
                        .long("delimiter")
                        .help(tr_s("import_delimiter_help"))
                        .num_args(1)
                        .value_name("CHAR")
                        .required(false)
                        .value_parser(clap::builder::NonEmptyStringValueParser::new()),
                ),
        )
        .subcommand(
            Command::new("add").about(tr("help.add.about")).subcommand(
                Command::new("book").about(tr("help.add.book.about")).arg(
                    Arg::new("isbn")
                        .long("isbn")
                        .help(tr("help.add.book.isbn"))
                        .required(true)
                        .value_name("ISBN"),
                ),
            ),
        )
        // help come subcommand dedicato (es: `librius help config`)
        .subcommand(
            Command::new("help").about(tr_s("help_flag_about")).arg(
                Arg::new("command")
                    .value_name("COMMAND")
                    .help(tr_s("help_lang")) // riuso chiave esistente per avere testo localizzato
                    .num_args(1),
            ),
        )
}

/// Esegue il parsing della CLI localizzata
pub fn parse_cli() -> clap::ArgMatches {
    build_cli().get_matches()
}

/// Esegue il comando selezionato
pub fn run_cli(
    matches: &clap::ArgMatches,
    conn: &mut Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(matches) = matches.subcommand_matches("list") {
        let short = matches.get_flag("short");
        let id = matches.get_one::<i32>("id").copied();
        let details = matches.get_flag("details");
        handle_list(conn, short, id, details)?;
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
    } else if let Some(("backup", sub_m)) = matches.subcommand() {
        let compress = sub_m.get_flag("compress");
        // esegue backup plain o compresso (zip su Windows, tar.gz su Unix)
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
        // 🔹 Recupera il percorso del file da importare
        let file_path = sub_m.get_one::<String>("file").cloned();
        if file_path.is_none() {
            print_err(&tr("import.error.missing_file"));
            return Ok(());
        }

        let file = file_path.unwrap();

        // 🔹 Determina il formato (default CSV)
        let _import_csv = sub_m.get_flag("csv");
        let import_json = sub_m.get_flag("json");

        // 🔹 Recupera delimitatore opzionale (solo CSV)
        let delimiter_char = if let Some(delim_str) = sub_m.get_one::<String>("delimiter") {
            delim_str.chars().next().unwrap_or(',')
        } else {
            ','
        };

        // 🔹 Esegui l’import nel formato corretto
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
        if let Some(cmd_name) = sub_m.get_one::<String>("command") {
            // Stampa help del sotto-comando se esiste
            if let Some(sc) = build_cli().find_subcommand(cmd_name) {
                sc.clone().print_help()?; // clone perché Command non implementa Copy
                println!();
                return Ok(());
            }
        }
        build_cli().print_help()?;
        println!();
        Ok(())
    } else {
        Ok(())
    }
}

/// Enum di compatibilità con i moduli dei comandi
#[derive(Subcommand)]
pub enum Commands {
    List,
    Config {
        init: bool,
        print: bool,
        edit: bool,
        editor: Option<String>,
    },
    Help,
}

#[cfg(test)]
mod tests_cli {
    use super::*;
    use crate::i18n::load_language;

    #[test]
    fn config_help_flags_no_value_placeholders() {
        load_language("en");
        let mut cmd = build_cli();
        // Trova subcommand config
        let sc = cmd
            .find_subcommand_mut("config")
            .expect("subcommand config esiste");
        let mut help_buf: Vec<u8> = Vec::new();
        sc.write_help(&mut help_buf).expect("help scritto");
        let help = String::from_utf8(help_buf).unwrap();
        assert!(
            !help.contains("<init>"),
            "--init non deve richiedere valore: {}",
            help
        );
        assert!(
            !help.contains("<print>"),
            "--print non deve richiedere valore: {}",
            help
        );
        assert!(
            !help.contains("<edit>"),
            "--edit non deve richiedere valore: {}",
            help
        );
    }
}
