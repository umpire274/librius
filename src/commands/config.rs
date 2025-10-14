use crate::cli::Commands;
use crate::config;
use crate::i18n::{tr, tr_with};
use crate::utils::{print_err, print_info, print_ok, print_warn};
use std::fs;
use std::path::Path;
use std::process::Command;

pub fn handle_config(cmd: &Commands) -> rusqlite::Result<()> {
    let config_path = config::config_file_path();

    if let Commands::Config {
        print,
        init,
        edit,
        editor,
    } = cmd
    {
        // --print
        if *print {
            if config_path.exists() {
                match fs::read_to_string(&config_path) {
                    Ok(contents) => {
                        println!();
                        print_info(&tr("config.schema.list"), true);
                        println!("\n{}", contents);
                    }
                    Err(e) => {
                        print_err(&tr_with("config.open.failed", &[("error", &e.to_string())]))
                    }
                }
            } else {
                print_warn(&tr("config.file.not_found"));
            }
        }

        // --init
        if *init {
            if config_path.exists() {
                print_warn(&tr("config.file.exists"));
            } else {
                if let Err(e) = config::load_or_init() {
                    print_err(&tr_with(
                        "config.file.init_failed",
                        &[("error", &e.to_string())],
                    ));
                }
                print_ok(
                    &tr_with(
                        "config.file.created",
                        &[("path", &config_path.display().to_string())],
                    ),
                    true,
                );
            }
        }

        // --edit
        if *edit {
            println!();
            if !config_path.exists()
                && let Err(e) = config::load_or_init()
            {
                print_err(&tr_with(
                    "config.file.create_failed",
                    &[("error", &e.to_string())],
                ));
            }

            // User-requested editor (if provided)
            let requested_editor = editor.clone();

            let default_editor = std::env::var("EDITOR")
                .or_else(|_| std::env::var("VISUAL"))
                .unwrap_or_else(|_| {
                    if cfg!(target_os = "windows") {
                        "notepad".to_string()
                    } else {
                        "nano".to_string()
                    }
                });

            // Use the requested editor if available, otherwise fall back
            let editor_to_use = requested_editor.unwrap_or_else(|| default_editor.clone());
            let editor_path = Path::new(&editor_to_use);

            let status = Command::new(editor_path).arg(&config_path).status();
            match status {
                Ok(s) if s.success() => {
                    print_ok(
                        &tr_with(
                            "config.file.edited_with",
                            &[("editor", &editor_path.display().to_string())],
                        ),
                        true,
                    );
                }
                Ok(_) | Err(_) => {
                    print_err(&tr_with(
                        "config.file.editor_not_available",
                        &[
                            ("editor", &editor_path.display().to_string()),
                            ("defaultEditor", &default_editor),
                        ],
                    ));
                    // Retry with the default editor
                    let fallback_status = Command::new(&default_editor).arg(&config_path).status();
                    match fallback_status {
                        Ok(s) if s.success() => {
                            print_ok(
                                &tr_with(
                                    "config.file.edited_fallback",
                                    &[("editor", &default_editor)],
                                ),
                                true,
                            );
                        }
                        Ok(_) | Err(_) => {
                            print_err(&tr_with(
                                "config.file.edit_failed_fallback",
                                &[("editor", &default_editor)],
                            ));
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
