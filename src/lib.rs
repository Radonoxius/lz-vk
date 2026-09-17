use std::{ffi::CStr, sync::atomic::AtomicBool};

use ash::vk::{API_VERSION_1_1, api_version_major, api_version_minor};

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
pub const LZVK_BASELINE_VULKAN_API_VERSION: u32 = API_VERSION_1_1;

/// Represents the major of the baseline Vulkan API version required by `lz-vk`
pub(crate) const LZVK_BASELINE_MAJOR: u32 = api_version_major(LZVK_BASELINE_VULKAN_API_VERSION);
/// Represents the minor of the baseline Vulkan API version required by `lz-vk`
pub(crate) const LZVK_BASELINE_MINOR: u32 = api_version_minor(LZVK_BASELINE_VULKAN_API_VERSION);

pub(crate) const LAYER_KHRONOS_VALIDATION_NAME: &CStr = c"VK_LAYER_KHRONOS_validation";

/// The INVALID string value.
/// 
/// Usually returned if a certain string is invalid
pub const INVALID: &str = "INVALID";

/// Specifies whether debug logs are enabled.
///
/// Disabled by default
pub(crate) static DEBUG_LOGS: AtomicBool = AtomicBool::new(false);