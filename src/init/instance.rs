use std::ffi::CStr;

use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, ExtensionProperties, InstanceCreateFlags, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME, LayerProperties}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{LAYER_KHRONOS_VALIDATION_NAME, LZVK_BASELINE_VULKAN_API_VERSION, logging::log};

/// Helper to create minimal `ApplicationInfo`
pub fn create_application_info(name: &'_ CStr) -> ApplicationInfo<'_> {
    ApplicationInfo {
        api_version: LZVK_BASELINE_VULKAN_API_VERSION,
        p_application_name: name.as_ptr(),
        ..Default::default()
    }
}

/// Returns the name of the Application.
/// 
/// # Safety
/// `app_info.p_application_name` must not be a nullptr!
/// The app name should be null terminated & contain valid UTF-8!
pub unsafe fn get_application_name<'a>(
    app_info: &ApplicationInfo<'a>
) -> &'a str {
    unsafe {
        CStr::from_ptr(app_info.p_application_name)
            .to_str()
            .unwrap_unchecked()
    }
}

/// Enumerates all available Instance Extensions
pub fn get_instance_extensions(
    entry: &Entry
) -> VkResult<Vec<ExtensionProperties>> {
    let instance_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None)
    };

    match instance_extensions {
        Err(e) => {
            log!(get_instance_extension_properties, AndroidLogPriority::Error, "{:?}", e);
            Err(e)
        },
        Ok(instance_extensions) => Ok(instance_extensions)
    }
}

/// Enumerates all available Instance Layers
pub fn get_instance_layers(
    entry: &Entry
) -> VkResult<Vec<LayerProperties>> {
    // Safe to do since it is infallible
    let instance_layers = unsafe {
        entry.enumerate_instance_layer_properties()
    };

    match instance_layers {
        Err(e) => {
            log!(get_instance_layer_properties, AndroidLogPriority::Error, "{:?}", e);
            Err(e)
        },
        Ok(instance_layers) => Ok(instance_layers)
    }
}

/// Returns `true` if `VK_KHR_portability_enumeration` Instance extension is supported.
/// 
/// Required to support Non-Conformant drivers & MoltenVK
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
/// `entry` must be valid!
///
/// `app_info.p_application_name` must not be a nullptr!
/// The app name should be null terminated & contain valid UTF-8!
pub unsafe fn init(
    entry: &Entry,
    app_info: &ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<Instance> {
    let mut instance_info = InstanceCreateInfo {
        p_application_info: app_info,
        ..Default::default()
    };
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