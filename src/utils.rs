use std::ffi::CStr;

use ash::{Entry, vk::{ApplicationInfo, ExtensionProperties, KHR_PORTABILITY_ENUMERATION_NAME}};

use crate::logging::log;

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
        log!(enumerate_instance_extensions, "{:?}", e);
        Vec::new()
    } else {
        unsafe {
            instance_extensions.unwrap_unchecked()
        }
    }
}

/// Returns the name of the Application or "Unnamed" if name isnt valid UTF-8.
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
            "{:?}",
            e
        );

        "Unnamed"
    } else {
        unsafe { name.unwrap_unchecked() }
    }
}