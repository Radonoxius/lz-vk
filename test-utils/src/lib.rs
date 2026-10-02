use ash::{Entry, vk::ApplicationInfo};
use lz_vk::init::{ApplicationContext, instance::{init, is_portability_enumeration_supported, get_instance_extensions, create_application_info}};

use std::ffi::{CStr, CString};

/// Returns true if the host system is running Android
pub fn is_android_host() -> bool {
    if cfg!(target_os = "android") {
        true
    } else {
        false
    }
}

/// Simplifies the creation of test info
///
/// Usage: `let app_info = test_info("my_test");`
pub fn test_info(name: &str) -> ApplicationInfo<'static> {
    let name = CString::new(format!("lz-vk:{name}"))
        .unwrap_or_default();

    // The macro version got a `'static` CStr for free from a string literal.
    // At runtime we have to leak the allocation so the returned info can
    // borrow it for `'static`. That's fine for tests (a few bytes per call).
    let name: &'static CStr = Box::leak(name.into_boxed_c_str());

    create_application_info(name)
}

/// Creates the test info for `name` and initializes an `ApplicationContext` from it.
///
/// Usage: `let ctx = test_init("my_test");`
pub fn test_init(name: &'static str) -> ApplicationContext<'static> {
    let entry = Entry::linked();
    let app_info = test_info(name);

    let instance_extensions = get_instance_extensions(&entry).unwrap();

    // Usage of Vulkan Validation Layers isnt allowed on Android (Termux), due to Android security policies.
    init(
        entry,
        app_info,
        !is_android_host(),
        is_portability_enumeration_supported(&instance_extensions)
    ).unwrap()
}