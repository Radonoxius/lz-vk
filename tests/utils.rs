use std::ffi::CStr;

use ash::vk::ApplicationInfo;
use lz_vk::LZVK_BASELINE_VULKAN_API_VERSION;

pub fn test_name(name: &'_ CStr) -> ApplicationInfo<'_> {
    ApplicationInfo {
        api_version: LZVK_BASELINE_VULKAN_API_VERSION,
        p_application_name: name.as_ptr(),
        ..Default::default()
    }
}