use std::sync::atomic::{AtomicBool, Ordering::{Relaxed, SeqCst}};

pub mod compression;

pub mod arm;

pub mod init;

// Specifies whether debug logs are enabled
pub(crate) static DEBUG_LOGS: AtomicBool = AtomicBool::new(false);

/// Enables Global Debug Logs. Safe for multi-threaded use
pub fn enable_debug_logs() {
    DEBUG_LOGS.store(true, Relaxed);
}

/// Disables Global Debug Logs. Safe for multi-threaded use
pub fn disable_debug_logs() {
    DEBUG_LOGS.store(false, Relaxed);
}

/// Enables a Debug Mode region, closed by `disable_debug_region`.
/// Useful to debug a region of code instead of everything. Safe for multi-threaded use
pub fn enable_debug_region() {
    DEBUG_LOGS.store(true, SeqCst);
}

/// Closes a Debug Mode region. Safe for multi-threaded use
pub fn disable_debug_region() {
    DEBUG_LOGS.store(false, SeqCst);
}
