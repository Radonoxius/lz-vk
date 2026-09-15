use ash::Entry;
use lz_vk::{init::{get_physical_gpus, init}, init_utils::{enumerate_instance_extensions, is_portability_enumeration_supported}, logging::{start_debug_region, stop_debug_region}};
use test_utils::test_info;

// Test `init` with Khronos Validation Layer disabled.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_gpu_test() {
    let entry = Entry::linked();
    let app_info = test_info(c"lz-vk:init_gpu_test");

    start_debug_region();

    let instance_extensions = unsafe { enumerate_instance_extensions(&entry) };
    let instance = unsafe {
        init(
            &entry,
            &app_info,
            false,
            is_portability_enumeration_supported(&instance_extensions)
        )
    }.unwrap();

    let _physical_gpus = get_physical_gpus(&instance)
        .unwrap();

    stop_debug_region();
}