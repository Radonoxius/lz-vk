use std::ffi::c_char;

use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, InstanceCreateInfo}};

pub fn init(
    enable_debug_validation_layer: bool,
    entry: &Entry,
    app_info: &ApplicationInfo
) -> VkResult<Instance> {
    let instance_info;
    let khr_validation_layer_name = c"VK_LAYER_KHRONOS_validation";
    
    if enable_debug_validation_layer {
        instance_info = InstanceCreateInfo {
            p_application_info: app_info,
            enabled_layer_count: 1,
            pp_enabled_layer_names: &khr_validation_layer_name.as_ptr() as *const *const c_char,
            ..Default::default()
        };
    } else {
        instance_info = InstanceCreateInfo {
            p_application_info: app_info,
            ..Default::default()
        }
    }
    
    unsafe { entry.create_instance(&instance_info, None) }
}