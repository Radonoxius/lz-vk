#[macro_export]
/// Simplifies the creation of test info
macro_rules! test_info {
    ($name:ident) => {
        lz_vk::init::instance::create_application_info(
            std::ffi::CStr::from_bytes_with_nul(
                std::concat!("lz-vk:", std::stringify!($name), "\0").as_bytes()
            )
            .unwrap()
        )
    };
}

/// Returns true if the host system is running Android
pub fn is_android_host() -> bool {
    if cfg!(target_os = "android") {
        true
    } else {
        false
    }
}