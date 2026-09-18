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

pub mod android {
    use std::ffi::CStr;

    #[cfg(target_os = "android")]
    use std::{ffi::c_char, sync::atomic::Ordering::Relaxed};
    #[cfg(target_os = "android")]
    use crate::DEBUG_LOGS;

    /// Represents the log tag of this library in Android logs
    pub const LOG_TAG: &CStr = c"lz-vk";

    /// Represents the ID of the `LOG_ID_MAIN` log buffer in Android
    #[allow(unused)]
    pub const LOG_ID_MAIN: i32 = 0;

    /// Represents the priority of the Andoird log
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
            match self {
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
    #[cfg(target_os = "android")]
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

    #[cfg(target_os = "android")]
    unsafe extern "C" {
        fn __android_log_buf_write(
            log_id: i32,
            prio: i32,
            tag: *const c_char,
            text: *const c_char
        ) -> i32;
    }
}

/// Generic logging macro that prints the log to `stdout`.
/// Additionally, on Android this logs to the `LOG_ID_MAIN` Log buffer.
/// 
/// Default Android Log priority is `Warn`
/// 
/// Safe for multi-threaded use.
/// 
/// SAFETY: Make sure that the log message size is less than 4KB!
/// 
/// ## Examples
/// ```rust,ignore
/// #[allow(unused)]
/// use crate::logging::android::AndroidLogPriority;
/// use crate::logging::log;
/// 
/// fn abcd() {
///     // Default Android Log priority is `AndroidLogPriority::Warn`
///     log!(abcd, "Hello!");
///     log!(abcd, "{}, {}", "Hello", "World!");
/// 
///     // Advanced logging (effective on Android only)
///     log!(abcd, AndroidLogPriority::Info, "Hello!");
///     log!(abcd, AndroidLogPriority::Error, "{}, {}", "Hello", "World!");
/// }
/// ```
macro_rules! log {
    ($function_name:ident, $fmt:literal $(, $($arg:tt)*)?) => {
        if crate::DEBUG_LOGS.load(std::sync::atomic::Ordering::Relaxed) {
            #[cfg(target_os = "android")]
            unsafe {
                crate::logging::android::android_log(
                    crate::logging::android::AndroidLogPriority::Warn,
                    stringify!($function_name),
                    &format!($fmt $(, $($arg)*)?)
                );
            }

            println!("[{}]: {}", stringify!($function_name), format!($fmt $(, $($arg)*)?));
        }
    };
    
    ($function_name:ident, $android_prio:path, $fmt:literal $(, $($arg:tt)*)?) => {
        if crate::DEBUG_LOGS.load(std::sync::atomic::Ordering::Relaxed) {
            #[cfg(target_os = "android")]
            unsafe {
                crate::logging::android::android_log(
                    $android_prio,
                    stringify!($function_name),
                    &format!($fmt $(, $($arg)*)?)
                );
            }

            println!("[{}]: {}", stringify!($function_name), format!($fmt $(, $($arg)*)?));
        }
    };
}

pub(crate) use log;