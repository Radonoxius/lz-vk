use std::ffi::CStr;

use ash::{Instance, prelude::VkResult, vk::{PhysicalDevice, PhysicalDeviceProperties2, PhysicalDeviceType, QueueFamilyProperties2, QueueFlags, api_version_major, api_version_minor}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{LZVK_BASELINE_MAJOR, LZVK_BASELINE_MINOR, logging::log};


/// Returns `true` if the given device Vulkan API version
/// is compatible with `lz-vk`
fn is_vulkan_baseline_compatible(
    device_properties2: PhysicalDeviceProperties2
) -> bool {
    api_version_major(device_properties2.properties.api_version) >= LZVK_BASELINE_MAJOR &&
    api_version_minor(device_properties2.properties.api_version) >= LZVK_BASELINE_MINOR
}

/// Returns `true` if the given device is a GPU
fn is_gpu(
    device_properties2: PhysicalDeviceProperties2
) -> bool {
    matches!(
        device_properties2.properties.device_type,
        PhysicalDeviceType::DISCRETE_GPU | PhysicalDeviceType::INTEGRATED_GPU
    )
}

/// Returns `true` if the given device supports Compute queues/stages
fn is_compute_queue_supported(
    instance: &Instance,
    physical_device: &PhysicalDevice
) -> bool {
    let queue_family_properties2_count = unsafe {
        instance.get_physical_device_queue_family_properties2_len(*physical_device)
    };
    if queue_family_properties2_count == 0 {
        return false;
    }

    let mut queue_family_properties2 = Vec::with_capacity(queue_family_properties2_count);
    for _ in 0..queue_family_properties2.capacity() {
        queue_family_properties2.push(QueueFamilyProperties2::default());
    }

    unsafe {
        instance.get_physical_device_queue_family_properties2(
            *physical_device,
            &mut queue_family_properties2
        );
    }

    // Here size equals capacity
    for i in 0..queue_family_properties2.capacity() {
        if
            queue_family_properties2[i].queue_family_properties.queue_flags & QueueFlags::COMPUTE == QueueFlags::COMPUTE &&
            queue_family_properties2[i].queue_family_properties.queue_count >= 1
        {
            return true;
        }
    }

    false
}

/// Returns a list of GPUs that are `lz-vk` compatible
pub fn get_supported_physical_gpus(
    instance: &Instance
) -> VkResult<Vec<PhysicalDevice>> {
    let all_physical_devices = unsafe { instance.enumerate_physical_devices() };

    match all_physical_devices {
        Err(e) => {
            log!(
                get_supported_gpus,
                AndroidLogPriority::Error,
                "{:?}",
                e
            );
            Err(e)
        },
        Ok(all_physical_devices) => {
            let supported_gpus = all_physical_devices.into_iter()
                .filter(|physical_device| {
                    let mut device_properties2 = PhysicalDeviceProperties2::default();

                    unsafe {
                        instance
                            .get_physical_device_properties2(
                                *physical_device,
                                &mut device_properties2
                            )
                    };

                    if is_vulkan_baseline_compatible(device_properties2) {
                        if
                            is_gpu(device_properties2) &&
                            is_compute_queue_supported(instance, physical_device)
                        {
                            log!(get_supported_gpus, AndroidLogPriority::Info, "true");
                            true
                        } else {
                            log!(get_supported_gpus, "false");
                            false
                        }
                    } else {
                        log!(get_supported_gpus, "false");
                        false
                    }
                })
                .collect();

            Ok(supported_gpus)
        }
    }
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