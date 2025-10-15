use librius::cli::{build_cli, run_cli};
use librius::config;
use librius::db;
use librius::i18n::{load_language, tr, tr_with};
use librius::utils::icons::ERR;
use librius::utils::{is_verbose, print_err, print_info, print_ok, set_verbose};

fn main() {
    // ------------------------------------------------------------
    // 1️⃣ Legge eventuali flag --lang e --verbose da CLI minimale
    // ------------------------------------------------------------
    let mut lang_arg: Option<String> = None;
    let mut verbose = false;

    {
        let mut args = std::env::args().peekable();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-l" | "--lang" => {
                    if let Some(val) = args.peek() {
                        lang_arg = Some(val.clone());
                        args.next(); // consuma valore
                    }
                }
                "-v" | "--verbose" => {
                    verbose = true;
                }
                _ => {}
            }
        }
    }

    set_verbose(verbose);

    // ------------------------------------------------------------
    // 2️⃣ Carica o inizializza il file di configurazione
    // ------------------------------------------------------------
    print_info("Loading configuration...", is_verbose());
    let config = config::load_or_init().unwrap_or_else(|_| panic!("{}Unable to load config", ERR));

    // ------------------------------------------------------------
    // 3️⃣ Determina la lingua effettiva
    // ------------------------------------------------------------
    let lang_code = lang_arg
        .or_else(|| {
            // tenta di leggere dal file di configurazione
            Some(config.language.clone())
        })
        .unwrap_or_else(|| "en".to_string());

    // Carica la lingua selezionata
    load_language(&lang_code);
    print_info(
        &tr_with("app.language.loaded", &[("lang", &lang_code)]),
        is_verbose(),
    );

    // ------------------------------------------------------------
    // 4️⃣ Inizializza o apre il database
    // ------------------------------------------------------------
    let mut conn = db::start_db(&config)
        .unwrap_or_else(|_| panic!("{}", &tr_with("db.open.failed", &[("icon-err", ERR)])));

    // ------------------------------------------------------------
    // 5️⃣ Esegue migrazioni DB e config
    // ------------------------------------------------------------
    if let Err(e) = db::migrate::run_migrations(&conn) {
        print_err(&tr_with("db.migrate.failed", &[("error", &e.to_string())]));
    } else {
        print_ok(&tr("db.schema.verified"), is_verbose());
    }

    if let Err(e) = config::migrate::migrate_config(&conn, &config::config_file_path()) {
        print_err(&tr_with(
            "config.migrate.failed",
            &[("error", &e.to_string())],
        ));
    } else {
        print_ok(&tr("config.schema.verified"), is_verbose());
    }

    // ------------------------------------------------------------
    // 6️⃣ CLI localizzata ed esecuzione comandi
    // ------------------------------------------------------------
    let matches = build_cli().get_matches();
    if let Err(e) = run_cli(&matches, &mut conn) {
        print_err(&format!("{} {}", ERR, e));
    }
}
