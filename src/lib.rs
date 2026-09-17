use std::{ffi::CStr, sync::atomic::AtomicBool};

use ash::vk::make_api_version;

pub mod compression;

#[cfg(feature = "arm")]
pub mod arm;
#[cfg(feature = "qualcomm")]
pub mod qualcomm;
#[cfg(feature = "nvidia")]
pub mod nvidia;

pub mod logging;
pub mod init_utils;
pub mod init;

/// Represents the baseline Vulkan API version required by `lz-vk`
pub const LZVK_BASELINE_VULKAN_API_VERSION: u32 = make_api_version(0, 1, 1, 0);

pub(crate) const LAYER_KHRONOS_VALIDATION_NAME: &CStr =
    unsafe { CStr::from_bytes_with_nul_unchecked(b"VK_LAYER_KHRONOS_validation\0") };

// Specifies whether debug logs are enabled
pub(crate) static DEBUG_LOGS: AtomicBool = AtomicBool::new(false);