use crate::commands::{handle_config, handle_edit_book, handle_list};
use crate::fields::EDITABLE_FIELDS;
use crate::i18n::{tr, tr_s};
use crate::tr_with;
use crate::utils::print_err;
use clap::{Arg, ArgAction, Command, Subcommand};
use rusqlite::Connection;

/// Costruisce la CLI localizzata usando le stringhe già caricate in memoria.
pub fn build_cli() -> Command {
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
                .action(ArgAction::Help)
                .global(true)
                .help_heading("Global options")
                .display_order(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help(tr_s("help_verbose"))
                .action(ArgAction::SetTrue)
                .global(true)
                .help_heading("Global options")
                .display_order(2),
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .help(tr_s("help_lang"))
                .global(true)
                .num_args(1)
                .help_heading("Global options")
                .display_order(3),
        )
        // 📘 list command
        .subcommand(
            Command::new("list")
                .about(tr_s("list_about"))
                .display_order(10)
                .arg(
                    Arg::new("short")
                        .long("short")
                        .help(tr_s("help.list.short"))
                        .action(ArgAction::SetTrue)
                        .help_heading("List-specific options")
                        .display_order(11),
                )
                .arg(
                    Arg::new("id")
                        .long("id")
                        .help(tr_s("help.list.id"))
                        .value_name("ID")
                        .num_args(1)
                        .value_parser(clap::value_parser!(i32))
                        .help_heading("List-specific options")
                        .display_order(12),
                )
                .arg(
                    Arg::new("details")
                        .long("details")
                        .help(tr_s("help.list.details"))
                        .action(ArgAction::SetTrue)
                        .help_heading("List-specific options")
                        .display_order(13),
                ),
        )
        // ⚙️ config command
        .subcommand(
            Command::new("config")
                .about(tr_s("config_about"))
                .display_order(20)
                .arg(
                    Arg::new("init")
                        .long("init")
                        .help(tr_s("config_init_help"))
                        .action(ArgAction::SetTrue)
                        .help_heading("Config-specific options")
                        .display_order(21),
                )
                .arg(
                    Arg::new("print")
                        .long("print")
                        .help(tr_s("config_print_help"))
                        .action(ArgAction::SetTrue)
                        .help_heading("Config-specific options")
                        .display_order(22),
                )
                .arg(
                    Arg::new("edit")
                        .long("edit")
                        .help(tr_s("config_edit_help"))
                        .action(ArgAction::SetTrue)
                        .help_heading("Config-specific options")
                        .display_order(23),
                )
                .arg(
                    Arg::new("editor")
                        .long("editor")
                        .requires("edit")
                        .num_args(1)
                        .help(tr_s("config_editor_help"))
                        .help_heading("Config-specific options")
                        .display_order(24),
                ),
        )
        // 💾 backup command
        .subcommand(
            Command::new("backup")
                .about(tr_s("backup_about"))
                .display_order(30)
                .arg(
                    Arg::new("compress")
                        .long("compress")
                        .help(tr_s("backup_compress_help"))
                        .action(ArgAction::SetTrue)
                        .help_heading("Backup-specific options")
                        .display_order(31),
                ),
        )
        // 📤 export command
        .subcommand(
            Command::new("export")
                .about(tr_s("export_about"))
                .display_order(40)
                .arg(
                    Arg::new("csv")
                        .long("csv")
                        .help(tr_s("export_csv_help"))
                        .action(ArgAction::SetTrue)
                        .conflicts_with_all(["xlsx", "json"])
                        .help_heading("Export-specific options")
                        .display_order(41),
                )
                .arg(
                    Arg::new("xlsx")
                        .long("xlsx")
                        .help(tr_s("export_xlsx_help"))
                        .conflicts_with_all(["csv", "json"])
                        .action(ArgAction::SetTrue)
                        .help_heading("Export-specific options")
                        .display_order(42),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help(tr_s("export_json_help"))
                        .conflicts_with_all(["csv", "xlsx"])
                        .action(ArgAction::SetTrue)
                        .help_heading("Export-specific options")
                        .display_order(43),
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .help(tr_s("export_output_help"))
                        .value_name("FILE")
                        .required(false)
                        .help_heading("Export-specific options")
                        .display_order(44),
                ),
        )
        // 📥 import command
        .subcommand(
            Command::new("import")
                .about(tr_s("import_about"))
                .display_order(50)
                .arg(
                    Arg::new("file")
                        .short('f')
                        .long("file")
                        .help(tr_s("import_file_help"))
                        .required(true)
                        .value_name("PATH")
                        .help_heading("Import-specific options")
                        .display_order(51),
                )
                .arg(
                    Arg::new("csv")
                        .long("csv")
                        .help(tr_s("import_csv_help"))
                        .action(ArgAction::SetTrue)
                        .conflicts_with("json")
                        .help_heading("Import-specific options")
                        .display_order(52),
                )
                .arg(
                    Arg::new("json")
                        .long("json")
                        .help(tr_s("import_json_help"))
                        .conflicts_with("csv")
                        .action(ArgAction::SetTrue)
                        .help_heading("Import-specific options")
                        .display_order(53),
                )
                .arg(
                    Arg::new("delimiter")
                        .short('d')
                        .long("delimiter")
                        .help(tr_s("import_delimiter_help"))
                        .num_args(1)
                        .value_name("CHAR")
                        .required(false)
                        .value_parser(clap::builder::NonEmptyStringValueParser::new())
                        .help_heading("Import-specific options")
                        .display_order(54),
                ),
        )
        // ➕ add book command
        .subcommand(
            Command::new("add")
                .about(tr("help.add.about"))
                .display_order(60)
                .subcommand(
                    Command::new("book")
                        .about(tr("help.add.book.about"))
                        .display_order(61)
                        .arg(
                            Arg::new("isbn")
                                .long("isbn")
                                .help(tr_s("help.add.book.isbn"))
                                .required(true)
                                .value_name("ISBN")
                                .help_heading("Add Book specific options")
                                .display_order(62),
                        ),
                ),
        )
        // ✏️ edit book command (aggiornato con logica ibrida ID/ISBN)
        .subcommand(
            Command::new("edit")
                .about(tr("help.edit.about"))
                .display_order(70)
                .subcommand({
                    let mut cmd = Command::new("book")
                        .about(tr("help.edit.book.about"))
                        .display_order(71)
                        .arg(
                            Arg::new("key")
                                .help(tr_s("help.edit.book.key"))
                                .required(true)
                                .num_args(1)
                                .help_heading("Edit Book required option")
                                .display_order(72),
                        );

                    // ✅ Aggiunta dinamica di tutti i campi editabili
                    for (i, (name, help, short)) in EDITABLE_FIELDS.iter().enumerate() {
                        cmd = cmd.arg(
                            Arg::new(*name)
                                .long(*name)
                                .short(*short)
                                .help(tr_s(help))
                                .num_args(1)
                                .action(ArgAction::Set)
                                .help_heading("Edit Book specific options")
                                .display_order(80 + i),
                        );
                    }

                    cmd
                }),
        )
        .subcommand(
            Command::new("help")
                .about(tr_s("help_flag_about"))
                .display_order(200)
                .arg(
                    Arg::new("command")
                        .value_name("COMMAND")
                        .help(tr_s("help_lang"))
                        .num_args(1)
                        .display_order(201),
                ),
        )
}

/// Parsing CLI
pub fn parse_cli() -> clap::ArgMatches {
    build_cli().get_matches()
}

/// Dispatch principale dei comandi
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
    } else if let Some(("edit", sub_m)) = matches.subcommand() {
        if let Some(("book", book_m)) = sub_m.subcommand() {
            handle_edit_book(conn, book_m)?; // ✅ integrazione comando edit book
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
