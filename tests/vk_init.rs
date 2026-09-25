use ash::Entry;
use lz_vk::{init::instance::{get_instance_extensions, is_portability_enumeration_supported, init}, logging::{start_debug_region, stop_debug_region}};
use test_utils::test_info;

// Test `init` with Khronos Validation Layer enabled.
// Requires your machine to have vulkan development headers and related packages
//
// This test fails on Android (Termux), due to Android security policies.
// So its ignored
#[test]
#[cfg_attr(target_os = "android", ignore)]
fn init_test() {
    let entry = Entry::linked();
    let app_info = test_info(c"lz-vk:init_test");

    start_debug_region();

    let instance_extensions = unsafe { get_instance_extensions(&entry) }.unwrap();
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
    let app_info = test_info(c"lz-vk:init_test_no_validation");

    start_debug_region();

    let instance_extensions = unsafe { get_instance_extensions(&entry) }.unwrap();
    let instance = unsafe {
        init(
            &entry,
            &app_info,
            false,
            is_portability_enumeration_supported(&instance_extensions)
        )
    };

    stop_debug_region();

    if let Err(_) = instance {
        instance.unwrap();
    }
}