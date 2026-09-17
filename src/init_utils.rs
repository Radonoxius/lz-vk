use std::ffi::CStr;

use ash::{Entry, Instance, vk::{PhysicalDevice, ApplicationInfo, ExtensionProperties, KHR_PORTABILITY_ENUMERATION_NAME}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{INVALID, logging::log};

/// Returns the name of the Application or `INVALID` if it isnt valid UTF-8.
/// 
/// SAFETY: `app_info.p_application_name` must be null terminated!
pub unsafe fn get_application_name<'a>(
    app_info: &ApplicationInfo<'a>
) -> &'a str {
    let name = unsafe {
        CStr::from_ptr(app_info.p_application_name)
    }.to_str();

    return if let Err(e) = name {
        log!(
            get_application_name,
            AndroidLogPriority::Error,
            "{:?}",
            e
        );

        INVALID
    } else {
        unsafe { name.unwrap_unchecked() }
    }
}

/// Enumerates all available Instance Extensions.
/// 
/// SAFETY: `entry` must be valid!
pub unsafe fn enumerate_instance_extensions(
    entry: &Entry
) -> Vec<ExtensionProperties> {
    let instance_extensions = unsafe {
        entry.enumerate_instance_extension_properties(None)
    };

    return if let Err(e) = instance_extensions {
        log!(enumerate_instance_extensions, AndroidLogPriority::Error, "{:?}", e);
        Vec::new()
    } else {
        unsafe {
            instance_extensions.unwrap_unchecked()
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
            if unsafe { instance_extension_name.unwrap_unchecked() } == KHR_PORTABILITY_ENUMERATION_NAME {
                return true;
            }
        }
    }

    log!(is_portability_enumeration_supported, "false");
    false
}

/// Returns the names of all extensions supported by the GPU
/// 
/// If an extension name isnt valid UTF-8, `INVALID` is used instead
pub fn get_gpu_extension_names(
    instance: &Instance,
    gpu: &PhysicalDevice
) -> Vec<String> {
    let gpu_extensions = unsafe {
        instance.enumerate_device_extension_properties(*gpu)
    };

    if let Err(e) = gpu_extensions {
        log!(get_gpu_extension_names, AndroidLogPriority::Error, "{:?}", e);
        Vec::new()
    } else {
        let gpu_extensions = unsafe { gpu_extensions.unwrap_unchecked() };
        let mut extension_names = Vec::new();

        for extension in gpu_extensions {
            let extension_name = unsafe {
                CStr::from_ptr(&raw const extension.extension_name[0])
            }.to_str();

            extension_names.push(
                if let Err(e) = extension_name {
                    log!(get_gpu_extension_names, AndroidLogPriority::Error, "{:?}", e);
                    String::from(INVALID)
                } else {
                    unsafe { extension_name.unwrap_unchecked() }.to_string()
                }
            );
        }

        extension_names
    }
}