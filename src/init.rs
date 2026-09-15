use std::ffi::CStr;

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

        if let Err(e) = instance_extension_name {
            log!(is_portability_enumeration_supported, "{:?}", e);
            return false;
        } else {
            if unsafe { instance_extension_name.unwrap_unchecked() } == KHR_PORTABILITY_ENUMERATION_NAME {
                return true;
            }
        }
    }

    log!(is_portability_enumeration_supported, "false");
    false
}

/// Enumerates all available Instance Extensions
pub fn enumerate_instance_extensions(entry: &Entry) -> Vec<ExtensionProperties> {
    let instance_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None)
    };

    return if let Err(e) = instance_extensions {
        log!(enumerate_instance_extensions, "{:?}", e);
        Vec::new()
    } else {
        unsafe {
            instance_extensions.unwrap_unchecked()
        }
    }
}

/// SAFETY: Application Name must be null terminated & valid UTF-8
pub unsafe fn init(
    entry: &Entry,
    app_info: &ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<Instance> {
    let instance_info;
    let applicaion_name = unsafe { CStr::from_ptr(app_info.p_application_name).to_str().unwrap_unchecked() };

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
    
    let instance = unsafe {
        entry.create_instance(&instance_info, None)
    };

    if let Err(e) = instance {
        log!(
            init,
            "[AppName: {}]: {:?}",
            applicaion_name,
            e
        );
    } else {
        log!(
            init,
            "[AppName: {}]: enable_debug_validation: {}, enbale_portability_enumeration: {}",
            applicaion_name,
            enable_debug_validation,
            enbale_portability_enumeration
        );
    }

    instance
}