use std::ptr::null;

use ash::Entry;
use lz_vk::{init::{get_supported_gpus, init}, init_utils::{get_instance_extensions, is_portability_enumeration_supported}, logging::{start_debug_region, stop_debug_region}, utils::get_logical_gpu};
use test_utils::test_info;

// Test `init` with Khronos Validation Layer disabled.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_gpu_test() {
    let entry = Entry::linked();
    let app_info = test_info(c"lz-vk:init_gpu_test");

    start_debug_region();

    let instance_extensions = unsafe { get_instance_extensions(&entry) }.unwrap();
    let instance = unsafe {
        init(
            &entry,
            &app_info,
            false,
            is_portability_enumeration_supported(&instance_extensions)
        )
    }.unwrap();

    let physical_gpus = get_supported_gpus(&instance)
        .unwrap();

    let _gpu = unsafe {
        get_logical_gpu(
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