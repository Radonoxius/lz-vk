use std::ptr::null;

use ash::Entry;
use lz_vk::{init::{instance::{get_instance_extension_names, get_instance_extensions, get_instance_layer_names, get_instance_layers, init, is_portability_enumeration_supported}, logical_gpu::create_logical_gpu, physical_gpu::{get_gpu_extension_names, get_gpu_extensions, get_supported_physical_gpus}}, logging::{start_debug_region, stop_debug_region}};
use test_utils::{is_android_host, test_info, test_init};

// Test `init`.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_instance() {
    let entry = Entry::linked();
    let app_info = test_info("init_instance");

    start_debug_region();

    let instance_extensions = get_instance_extensions(&entry).unwrap();
    let instance_extension_names = get_instance_extension_names(&instance_extensions);
    println!("{instance_extension_names:#?}\n");

    let instance_layers = get_instance_layers(&entry).unwrap();
    let instance_layer_names = get_instance_layer_names(&instance_layers);
    println!("{instance_layer_names:#?}\n");

    let _app_ctx =
        // Usage of Vulkan Validation Layers isnt allowed on Android (Termux), due to Android security policies.
        init(
            entry,
            app_info,
            !is_android_host(),
            is_portability_enumeration_supported(&instance_extensions)
        ).unwrap();

    stop_debug_region();
}

// Test `init`.
// Requires your machine to have vulkan development headers and related packages
#[test]
fn init_physical_gpu() {
    start_debug_region();
    let app_ctx = test_init("init_physical_gpu");

    let physical_gpus = get_supported_physical_gpus(&app_ctx)
        .unwrap();

    let gpu_extensions = get_gpu_extensions(&app_ctx, &physical_gpus[0]).unwrap();
    let _gpu_extension_names = get_gpu_extension_names(&gpu_extensions);

    let _gpu = unsafe {
        create_logical_gpu(
            &app_ctx,
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