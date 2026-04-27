// =====================================================
// Librius - utils/verbose.rs
// -----------------------------------------------------
// Gestione della modalità verbose (debug) globale.
// =====================================================

use std::sync::OnceLock;

static VERBOSE: OnceLock<bool> = OnceLock::new();

/// Enables verbose (debug) mode.
pub fn set_verbose(enabled: bool) {
    let _ = VERBOSE.set(enabled);
}

/// Returns true if verbose mode is active.
pub fn is_verbose() -> bool {
    *VERBOSE.get().unwrap_or(&false)
}
