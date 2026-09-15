use std::ffi::CStr;

use ash::vk::ApplicationInfo;
use lz_vk::LZVK_BASELINE_VULKAN_API_VERSION;

/// Simplifies the creation of test info
pub fn test_info(name: &'_ CStr) -> ApplicationInfo<'_> {
    ApplicationInfo {
        api_version: LZVK_BASELINE_VULKAN_API_VERSION,
        p_application_name: name.as_ptr(),
        ..Default::default()
    }
}