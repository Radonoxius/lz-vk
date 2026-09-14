use ash::{Entry, vk::ApplicationInfo};
use lz_vk::{LZVK_BASELINE_VULKAN_API_VERSION, init::{init, is_portability_enumeration_supported}};

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
        ..Default::default()
    };

    let instance_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None).unwrap()
    };
    let instance = init(
        &entry,
        &app_info,
        true,
        is_portability_enumeration_supported(&instance_extensions)
    );

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
        ..Default::default()
    };

    let instance_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None).unwrap()
    };
    let instance = init(
        &entry,
        &app_info,
        false,
        is_portability_enumeration_supported(&instance_extensions)
    );

    if let Err(_) = instance {
        instance.unwrap();
    }
}