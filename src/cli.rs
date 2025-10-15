use crate::commands::{config::handle_config, list::handle_list};
use crate::i18n::tr_s;
use clap::{Arg, Command, Subcommand};
use colored::Colorize;
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
        .subcommand(Command::new("list").about(tr_s("list_about")))
        .subcommand(
            Command::new("config")
                .about(tr_s("config_about"))
                .arg(Arg::new("init").long("init").help(tr_s("config_init_help")).action(clap::ArgAction::SetTrue))
                .arg(
                    Arg::new("print")
                        .long("print")
                        .help(tr_s("config_print_help"))
                        .action(clap::ArgAction::SetTrue),
                )
                .arg(Arg::new("edit").long("edit").help(tr_s("config_edit_help")).action(clap::ArgAction::SetTrue))
                .arg(
                    Arg::new("editor")
                        .long("editor")
                        .requires("edit")
                        .num_args(1)
                        .help(tr_s("config_editor_help")),
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
    conn: &Connection,
) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(("list", _)) = matches.subcommand() {
        handle_list(conn).unwrap_or_else(|e| {
            eprintln!("{} {}", "Error listing books:".red(), e);
        });
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
        let sc = cmd.find_subcommand_mut("config").expect("subcommand config esiste");
        let mut help_buf: Vec<u8> = Vec::new();
        sc.write_help(&mut help_buf).expect("help scritto");
        let help = String::from_utf8(help_buf).unwrap();
        assert!(!help.contains("<init>"), "--init non deve richiedere valore: {}", help);
        assert!(!help.contains("<print>"), "--print non deve richiedere valore: {}", help);
        assert!(!help.contains("<edit>"), "--edit non deve richiedere valore: {}", help);
    }
}
