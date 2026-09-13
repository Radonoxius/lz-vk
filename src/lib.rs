use std::sync::atomic::{AtomicBool, Ordering::SeqCst};

pub mod compression;

pub mod arm;

pub mod init;

// Specifies whether debug logs are enabled
pub(crate) static DEBUG_LOGS: AtomicBool = AtomicBool::new(false);

/// Enables Debug Logs within a region ended with `disable_debug_region`.
/// Useful to debug a region of code (or everything). Safe for multi-threaded use
pub fn enable_debug_region() {
    DEBUG_LOGS.store(true, SeqCst);
}

/// Closes the Debug Logs region. Safe for multi-threaded use
pub fn disable_debug_region() {
    DEBUG_LOGS.store(false, SeqCst);
}