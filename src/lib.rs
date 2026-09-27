use std::ffi::CStr;

use ash::vk::API_VERSION_1_1;

pub mod compression;
pub mod decompression;

pub mod logging;
pub mod utils;
pub mod init;

/// Represents the baseline Vulkan API version required by `lz-vk`
pub const LZVK_BASELINE_VULKAN_API_VERSION: u32 = API_VERSION_1_1;

pub(crate) const LAYER_KHRONOS_VALIDATION_NAME: &CStr = c"VK_LAYER_KHRONOS_validation";