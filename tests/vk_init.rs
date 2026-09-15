use ash::{Entry, vk::ApplicationInfo};
use lz_vk::{LZVK_BASELINE_VULKAN_API_VERSION, init::{enumerate_instance_extensions, init, is_portability_enumeration_supported}, logging::{start_debug_region, stop_debug_region}};

// Test `init` with Khronos Validation Layer enabled.
// Requires your machine to have vulkan development headers and related packages
//
// This test fails on Android (Termux), due to Android security policies.
// So its ignored
#[test]
#[cfg_attr(target_os = "android", ignore)]
fn init_test() {
    let entry = Entry::linked();
    let app_info = ApplicationInfo {
        api_version: LZVK_BASELINE_VULKAN_API_VERSION,
        p_application_name: c"lz-vk:init_test".as_ptr(),
        ..Default::default()
    };

    start_debug_region();

    let instance_extensions = enumerate_instance_extensions(&entry);
    let instance = unsafe {
        init(
            &entry,
            &app_info,
            true,
            is_portability_enumeration_supported(&instance_extensions)
        )
    };

    stop_debug_region();

    if let Err(_) = instance {
        instance.unwrap();
    }
}

// Test `init` with Khronos Validation Layer disabled.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_test_no_validation() {
    let entry = Entry::linked();
    let app_info = ApplicationInfo {
        api_version: LZVK_BASELINE_VULKAN_API_VERSION,
        p_application_name: c"lz-vk:init_test_no_validation".as_ptr(),
        ..Default::default()
    };

    start_debug_region();

    let instance_extensions = enumerate_instance_extensions(&entry);
    let instance = unsafe {
        init(
            &entry,
            &app_info,
            true,
            is_portability_enumeration_supported(&instance_extensions)
        )
    };

    stop_debug_region();

    if let Err(_) = instance {
        instance.unwrap();
    }
}