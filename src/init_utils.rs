use std::ffi::CStr;

use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, ExtensionProperties, KHR_PORTABILITY_ENUMERATION_NAME, PhysicalDevice}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::logging::log;

/// Returns the name of the Application.
/// 
/// **SAFETY**: `app_info.p_application_name` must be null terminated & valid UTF-8!
pub unsafe fn get_application_name<'a>(
    app_info: &ApplicationInfo<'a>
) -> &'a str {
    // Safe to do since it is infallible
    unsafe {
        CStr::from_ptr(app_info.p_application_name).to_str().unwrap_unchecked()
    }
}

/// Enumerates all available Instance Extensions.
/// 
/// **SAFETY**: `entry` must be valid!
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

/// Returns the names of all extensions supported by the GPU
pub fn get_gpu_extension_names(
    instance: &Instance,
    selected_gpu: &PhysicalDevice
) -> VkResult<Vec<String>> {
    let gpu_extensions = unsafe {
        instance.enumerate_device_extension_properties(*selected_gpu)
    };

    match gpu_extensions {
        Err(e) => {
            log!(get_gpu_extension_names, AndroidLogPriority::Error, "{:?}", e);
            Err(e)
        },
        Ok(gpu_extensions) => {
            let mut extension_names = Vec::new();

            for extension in gpu_extensions {
                extension_names.push(
                    // Safe to do since it is infallible
                    unsafe {
                        CStr::from_ptr(&raw const extension.extension_name[0]).to_str().unwrap_unchecked().to_string()
                    }
                );
            }

            Ok(extension_names)
        }
    }
}