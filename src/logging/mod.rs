use std::sync::atomic::Ordering::SeqCst;

use crate::DEBUG_LOGS;

/// Enables Debug Logs within a region ended with `stop_debug_region`.
/// Useful to debug a region of code (or everything). Safe for multi-threaded use
pub fn start_debug_region() {
    DEBUG_LOGS.store(true, SeqCst);
}

/// Closes the Debug Logs region. Safe for multi-threaded use
pub fn stop_debug_region() {
    DEBUG_LOGS.store(false, SeqCst);
}

#[cfg(target_os = "android")]
pub mod android {
    use std::{ffi::{CStr, c_char}, sync::atomic::Ordering::Relaxed};

    use crate::DEBUG_LOGS;

    /// Represents the log tag of this library in Android logs
    pub const LOG_TAG: &CStr = c"lz-vk";

    /// Represents the ID of the `LOG_ID_MAIN` log buffer in Android
    const LOG_ID_MAIN: i32 = 0;

    /// Represents the priority of the log in Android logs
    #[allow(unused)]
    pub(crate) enum AndroidLogPriority {
        Unknown,
        Default,
        Verbose,
        Debug,
        Info,
        Warn,
        Error,
        Fatal,
        Silent
    }

    impl Into<i32> for AndroidLogPriority {
        fn into(self) -> i32 {
            return match self {
                Self::Unknown => 0,
                Self::Default => 1,
                Self::Verbose => 2,
                Self::Debug => 3,
                Self::Info => 4,
                Self::Warn => 5,
                Self::Error => 6,
                Self::Fatal => 7,
                Self::Silent => 8
            }
        }
    }

    /// Logs to the `LOG_ID_MAIN` Android Log buffer.
    /// Safe for multi-threaded use
    ///
    /// SAFETY: Make sure that the log message size is less than 4KB!
    pub(crate) unsafe fn android_log(priority: AndroidLogPriority, function_name: &str, message: &str) {
        if DEBUG_LOGS.load(Relaxed) {
            unsafe {
                let log = format!("[{function_name}]: {message}\0");

                let _errno = __android_log_buf_write(
                    LOG_ID_MAIN,
                    priority.into(),
                    LOG_TAG.as_ptr(),
                    log.as_ptr() as *const c_char
                );
            }
        }
    }

    unsafe extern "C" {
        fn __android_log_buf_write(
            log_id: i32,
            prio: i32,
            tag: *const c_char,
            text: *const c_char
        ) -> i32;
    }
}

/// Generic logging macro
/// that prints the log to `stdout`.
/// 
/// Additionally, on Android this logs to the `LOG_ID_MAIN` Log buffer as well.
/// 
/// The first argument is the function name (identifier) and the second is
/// the message string literal. Also supports format! semantics and additional args.
/// 
/// Safe for multi-threaded use
/// 
/// SAFETY: Make sure that the log message size is less than 4KB!
macro_rules! log {
    ($function_name:ident, $message:literal) => {
        if crate::DEBUG_LOGS.load(std::sync::atomic::Ordering::Relaxed) {
            #[cfg(target_os = "android")]
            unsafe {
                crate::logging::android::android_log(
                    crate::logging::android::AndroidLogPriority::Info,
                    stringify!($function_name),
                    $message
                );
            }

            println!("[{}]: {}", stringify!($function_name), $message);
        }
    };
    ($function_name:ident, $fmt:literal, $($arg:tt)*) => {
        if crate::DEBUG_LOGS.load(std::sync::atomic::Ordering::Relaxed) {
            #[cfg(target_os = "android")]
            unsafe {
                crate::logging::android::android_log(
                    crate::logging::android::AndroidLogPriority::Info,
                    stringify!($function_name),
                    &format!($fmt, $($arg)*)
                );
            }

            println!("[{}]: {}", stringify!($function_name), format!($fmt, $($arg)*));
        }
    };
}

pub(crate) use log;