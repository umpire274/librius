use crate::cli::Commands;
use crate::config;
use crate::utils::icons::*;
use colored::*;
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
                        println!("\n{}{}\n", INFO, "Current configuration:".bold().green());
                        println!("{}", contents);
                    }
                    Err(e) => eprintln!("{}{} {}", ERR, "Error reading config:".red(), e),
                }
            } else {
                eprintln!(
                    "{}{}",
                    WARN,
                    "Configuration file not found. Run `librius config --init` first.".yellow()
                );
            }
        }

        // --init
        if *init {
            if config_path.exists() {
                println!(
                    "\n{}{}",
                    WARN,
                    "Configuration file already exists.".yellow()
                );
            } else {
                if let Err(e) = config::load_or_init() {
                    eprintln!(
                        "{}{} {}",
                        ERR,
                        "Error creating default configuration:".red(),
                        e
                    );
                }
                println!(
                    "{}{}",
                    OK,
                    format!("Created new configuration at: {}", config_path.display())
                        .bold()
                        .green()
                );
            }
        }

        // --edit
        if *edit {
            println!();
            if !config_path.exists()
                && let Err(e) = config::load_or_init()
            {
                eprintln!("{}{} {}", ERR, "Error creating config file:".red(), e);
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
                    println!(
                        "{}{}",
                        OK,
                        format!(
                            "Configuration file edited successfully with '{}'",
                            editor_path.display()
                        )
                        .bold()
                        .green()
                    );
                }
                Ok(_) | Err(_) => {
                    eprintln!(
                        "{}{}",
                        ERR,
                        format!(
                            "Editor '{}' not available, falling back to '{}'",
                            editor_path.display(),
                            default_editor
                        )
                        .red()
                    );
                    // Retry with the default editor
                    let fallback_status = Command::new(&default_editor).arg(&config_path).status();
                    match fallback_status {
                        Ok(s) if s.success() => {
                            println!(
                                "{}{}",
                                OK,
                                format!(
                                    "Configuration file edited successfully with fallback '{}'",
                                    default_editor
                                )
                                .bold()
                                .green()
                            );
                        }
                        Ok(_) | Err(_) => {
                            eprintln!(
                                "{}{}",
                                ERR,
                                format!(
                                    "Failed to edit configuration file with fallback '{}'",
                                    default_editor
                                )
                                .red()
                            );
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
