use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, ExtensionProperties, InstanceCreateFlags, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME}};

use crate::{LAYER_KHRONOS_VALIDATION_NAME, logging::log};

/// Finds if `VK_KHR_portability_enumeration` Instance extension is supported.
/// Useful to query support for Non-Conformant drivers & MoltenVK
pub fn is_portability_enumeration_supported(
    instance_extensions: &Vec<ExtensionProperties>
) -> bool {
    for instance_extension in instance_extensions {
        let instance_extension_name = instance_extension
            .extension_name_as_c_str();

        if let Err(_) = instance_extension_name {
            log!(is_portability_enumeration_supported, "false");
            return false;
        } else {
            if unsafe { instance_extension_name.unwrap_unchecked() } == KHR_PORTABILITY_ENUMERATION_NAME {
                log!(is_portability_enumeration_supported, "true");
                return true;
            }
        }
    }

    log!(is_portability_enumeration_supported, "false");
    false
}

pub fn init(
    entry: &Entry,
    app_info: &ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<Instance> {
    let instance_info;

    let enabled_layers = [LAYER_KHRONOS_VALIDATION_NAME.as_ptr()];
    let enabled_instance_extensions = [KHR_PORTABILITY_ENUMERATION_NAME.as_ptr()];
    
    if enable_debug_validation && enbale_portability_enumeration {
        instance_info = InstanceCreateInfo::default()
            .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
            .application_info(app_info)
            .enabled_layer_names(&enabled_layers)
            .enabled_extension_names(&enabled_instance_extensions);
    } else if enable_debug_validation && !enbale_portability_enumeration {
        instance_info = InstanceCreateInfo::default()
            .application_info(app_info)
            .enabled_layer_names(&enabled_layers)
    } else if !enable_debug_validation && enbale_portability_enumeration {
        instance_info = InstanceCreateInfo::default()
            .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
            .application_info(app_info)
            .enabled_extension_names(&enabled_instance_extensions);
    } else {
        instance_info = InstanceCreateInfo::default()
            .application_info(app_info);
    }
    
    unsafe { entry.create_instance(&instance_info, None) }
}