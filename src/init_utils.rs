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

    if let Err(e) = instance_extensions {
        log!(enumerate_instance_extensions, AndroidLogPriority::Error, "{:?}", e);
        Err(e)
    } else {
        // Safe to do since we already checked for error
        unsafe {
            Ok(instance_extensions.unwrap_unchecked())
        }
    }
}

/// Finds if `VK_KHR_portability_enumeration` Instance extension is supported.
/// Useful to query support for Non-Conformant drivers & MoltenVK
pub fn is_portability_enumeration_supported(
    instance_extensions: &Vec<ExtensionProperties>
) -> bool {
    for instance_extension in instance_extensions {
        let instance_extension_name = instance_extension
            .extension_name_as_c_str();

        if let Err(e) = instance_extension_name {
            log!(is_portability_enumeration_supported, AndroidLogPriority::Error, "{:?}", e);
            return false;
        } else {
            // Safe to do since we already checked for error
            if unsafe { instance_extension_name.unwrap_unchecked() } == KHR_PORTABILITY_ENUMERATION_NAME {
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

    if let Err(e) = gpu_extensions {
        log!(get_gpu_extension_names, AndroidLogPriority::Error, "{:?}", e);
        Err(e)
    } else {
        // Safe to do since we already checked for error
        let gpu_extensions = unsafe { gpu_extensions.unwrap_unchecked() };
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