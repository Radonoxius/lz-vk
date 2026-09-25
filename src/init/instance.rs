use std::ffi::CStr;

use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, ExtensionProperties, InstanceCreateFlags, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{LAYER_KHRONOS_VALIDATION_NAME, logging::log};

/// Returns the name of the Application.
/// 
/// # Safety
/// `app_info.p_application_name` must be null terminated & valid UTF-8!
pub unsafe fn get_application_name<'a>(
    app_info: &ApplicationInfo<'a>
) -> &'a str {
    unsafe {
        CStr::from_ptr(app_info.p_application_name).to_str().unwrap_unchecked()
    }
}

/// Enumerates all available Instance Extensions.
/// 
/// # Safety
/// `entry` must be valid!
pub unsafe fn get_instance_extensions(
    entry: &Entry
) -> VkResult<Vec<ExtensionProperties>> {
    let instance_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None)
    };

    match instance_extensions {
        Err(e) => {
            log!(enumerate_instance_extensions, AndroidLogPriority::Error, "{:?}", e);
            Err(e)
        },
        Ok(instance_extensions) => Ok(instance_extensions)
    }
}

/// Finds if `VK_KHR_portability_enumeration` Instance extension is supported.
/// Useful to query support for Non-Conformant drivers & MoltenVK
pub fn is_portability_enumeration_supported(
    instance_extensions: &[ExtensionProperties]
) -> bool {
    for instance_extension in instance_extensions {
        let instance_extension_name = instance_extension
            .extension_name_as_c_str();

        match instance_extension_name {
            Err(e) =>
                log!(is_portability_enumeration_supported, AndroidLogPriority::Error, "{:?}", e),
            Ok(instance_extension_name) =>
                if instance_extension_name == KHR_PORTABILITY_ENUMERATION_NAME {
                    return true;
                }
        }
    }

    log!(is_portability_enumeration_supported, "false");
    false
}

/// Initializes a Vulkan instance based on the given parameters
/// 
/// # Safety
/// `entry` must be valid & `app_info.p_application_name` must be null terminated & valid UTF-8!
pub unsafe fn init(
    entry: &Entry,
    app_info: &ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<Instance> {
    let mut instance_info = InstanceCreateInfo::default();
    let applicaion_name = unsafe { get_application_name(app_info) };

    let enabled_layers = [LAYER_KHRONOS_VALIDATION_NAME.as_ptr()];
    let enabled_instance_extensions = [KHR_PORTABILITY_ENUMERATION_NAME.as_ptr()];
    
    if enable_debug_validation {
        instance_info = instance_info.enabled_layer_names(&enabled_layers);
    }
    if enbale_portability_enumeration {
        instance_info = instance_info
            .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
            .enabled_extension_names(&enabled_instance_extensions);
    }
    
    let instance = unsafe {
        entry.create_instance(&instance_info, None)
    };

    match instance {
        Err(e) => log!(
            init,
            AndroidLogPriority::Error,
            "[AppName: {}]: {:?}",
            applicaion_name,
            e
        ),
        _ => log!(
            init,
            AndroidLogPriority::Info,
            "[AppName: {}]: enable_debug_validation: {}, enbale_portability_enumeration: {}",
            applicaion_name,
            enable_debug_validation,
            enbale_portability_enumeration
        )
    }

    instance
}