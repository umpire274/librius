// =====================================================
// Librius - Utilities module
// -----------------------------------------------------
// Contiene funzioni di supporto generali e costanti
// grafiche per output CLI.
// =====================================================

use colored::*;

/// Modulo con le icone standard utilizzate nell'applicazione.
///
/// Le emoji sono seguite da uno spazio per evitare problemi
/// di spaziatura nei terminali (⚠️, ✅, ❌, 📘, ecc.).
pub mod icons {
    pub const OK: &str = "✅ ";
    pub const ERR: &str = "❌ ";
    pub const WARN: &str = "⚠️  ";
    pub const INFO: &str = "📘  ";
}

/// Stampa un messaggio formattato come "OK"
pub fn print_ok(msg: &str) {
    println!("{}{}", icons::OK, msg.green().bold());
}

/// Stampa un messaggio di errore formattato
pub fn print_err(msg: &str) {
    eprintln!("{}{}", icons::ERR, msg.red().bold());
}

/// Stampa un messaggio di avviso formattato
pub fn print_warn(msg: &str) {
    println!("{}{}", icons::WARN, msg.yellow().bold());
}

/// Stampa un messaggio informativo
pub fn print_info(msg: &str) {
    println!("{}{}", icons::INFO, msg.blue().bold());
}
