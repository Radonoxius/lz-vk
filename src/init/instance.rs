use std::{ffi::CStr, ptr::null};

use ash::{Entry, prelude::VkResult, vk::{ApplicationInfo, ExtensionProperties, InstanceCreateFlags, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME, LayerProperties}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{LAYER_KHRONOS_VALIDATION_NAME, LZVK_BASELINE_VULKAN_API_VERSION, init::ApplicationContext, logging::log};

/// Helper to create minimal `ApplicationInfo`
pub fn create_application_info(name: &'_ CStr) -> ApplicationInfo<'_> {
    ApplicationInfo {
        api_version: LZVK_BASELINE_VULKAN_API_VERSION,
        p_application_name: name.as_ptr(),
        ..Default::default()
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
            log!(get_instance_extensions, AndroidLogPriority::Error, "{:?}", e);
            Err(e)
        },
        Ok(instance_extensions) => Ok(instance_extensions)
    }
}

/// Returns the names of extensions supported by the instance.
/// 
/// Ignores the extensions whose names arent null terminated or
/// dont contain valid UTF-8
pub fn get_instance_extension_names(
    instance_extension_properties: &[ExtensionProperties]
) -> Vec<&str> {
    instance_extension_properties
        .iter()
        .filter_map(|ext| ext.extension_name_as_c_str().ok()?.to_str().ok())
        .collect()
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
            log!(get_instance_layers, AndroidLogPriority::Error, "{:?}", e);
            Err(e)
        },
        Ok(instance_layers) => Ok(instance_layers)
    }
}

/// Returns the names of layers supported by the instance.
/// 
/// Ignores the layers whose names arent null terminated or
/// dont contain valid UTF-8
pub fn get_instance_layer_names(
    instance_layer_properties: &[LayerProperties]
) -> Vec<&str> {
    instance_layer_properties
        .iter()
        .filter_map(|layer_property| layer_property.layer_name_as_c_str().ok()?.to_str().ok())
        .collect()
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
/// **NOTE**: If the provided application name isnt valid UTF-8, the default string value
/// will be used as its name in the logs
pub fn init(
    entry: Entry,
    app_info: ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<ApplicationContext> {
    let mut instance_info = InstanceCreateInfo {
        p_application_info: &app_info,
        ..Default::default()
    };

    let applicaion_name =
        if app_info.p_application_name == null() {
            Default::default()
        } else {
            let app_name = unsafe {
                CStr::from_ptr(app_info.p_application_name)
            }.to_str();
            match app_name {
                Err(_) => Default::default(),
                Ok(app_name) => app_name
            }
        };

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
        Err(e) => {
            log!(
                init,
                AndroidLogPriority::Error,
                "[AppName: {}]: {:?}",
                applicaion_name,
                e
            );
            Err(e)
        },
        Ok(instance) => {
            log!(
                init,
                AndroidLogPriority::Info,
                "[AppName: {}]: enable_debug_validation: {}, enbale_portability_enumeration: {}",
                applicaion_name,
                enable_debug_validation,
                enbale_portability_enumeration
            );
            Ok(ApplicationContext::new(entry, app_info, instance))
        }
    }
}

#[cfg(test)]
mod tests {
    use test_utils::test_init;
    use crate::{init::{instance::{get_instance_extensions, get_instance_layers}}, logging::{start_debug_region, stop_debug_region}};

    #[test]
    fn test_get_instance_extensions() {
        start_debug_region();
        let app_ctx = test_init("test_get_instance_extensions");

        let instance_extensions = get_instance_extensions(app_ctx.entry());
        assert!(instance_extensions.is_ok());

        let instance_extensions = instance_extensions.unwrap();
        assert!(instance_extensions.len() >= 1);
        stop_debug_region();
    }

    #[test]
    fn test_get_instance_layers() {
        start_debug_region();
        let app_ctx = test_init("test_get_instance_layers");

        let instance_layers = get_instance_layers(&app_ctx.entry());
        assert!(instance_layers.is_ok());

        let _instance_layers = instance_layers.unwrap();
        stop_debug_region();
    }
}