use std::ptr::null;

use ash::Entry;
use lz_vk::{init::{instance::{get_instance_extensions, is_portability_enumeration_supported, init}, logical_gpu::create_logical_gpu, physical_gpu::get_supported_physical_gpus}, logging::{start_debug_region, stop_debug_region}};
use test_utils::{is_android_host, test_info};

// Test `init`.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_instance() {
    let entry = Entry::linked();
    let app_info = test_info!(init_instance);

    start_debug_region();

    let instance_extensions = get_instance_extensions(&entry).unwrap();
    let _instance =
        // Usage of Vulkan Validation Layers isnt allowed on Android (Termux), due to Android security policies.
        init(
            &entry,
            &app_info,
            !is_android_host(),
            is_portability_enumeration_supported(&instance_extensions)
        ).unwrap();

    stop_debug_region();
}

// Test `init`.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_physical_gpu() {
    let entry = Entry::linked();
    let app_info = test_info!(init_physical_gpu);

    start_debug_region();

    let instance_extensions = get_instance_extensions(&entry).unwrap();
    let instance =
        // Usage of Vulkan Validation Layers isnt allowed on Android (Termux), due to Android security policies.
        init(
            &entry,
            &app_info,
            !is_android_host(),
            is_portability_enumeration_supported(&instance_extensions)
        ).unwrap();

    let physical_gpus = get_supported_physical_gpus(&instance)
        .unwrap();

    let _gpu = unsafe {
        create_logical_gpu(
            &instance,
            &physical_gpus[0],
            &[null()],
            &[0],
            &[1],
            &[&[1.0]],
            null(),
            &[]
        )
    }.unwrap();

    stop_debug_region();
}