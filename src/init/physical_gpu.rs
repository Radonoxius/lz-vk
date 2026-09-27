use ash::{Instance, prelude::VkResult, vk::{ExtensionProperties, PhysicalDevice, PhysicalDeviceProperties2, PhysicalDeviceType, QueueFamilyProperties2, QueueFlags}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{LZVK_BASELINE_VULKAN_API_VERSION, init::ApplicationContext, logging::log};

/// Returns `true` if the given device Vulkan API version
/// is compatible with `lz-vk`
fn is_vulkan_baseline_compatible(
    device_properties2: &PhysicalDeviceProperties2
) -> bool {
    device_properties2.properties.api_version >= LZVK_BASELINE_VULKAN_API_VERSION
}

/// Returns `true` if the given device is a GPU
fn is_gpu(
    device_properties2: &PhysicalDeviceProperties2
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
    for queue_family_property in queue_family_properties2 {
        if
            queue_family_property.queue_family_properties.queue_flags & QueueFlags::COMPUTE == QueueFlags::COMPUTE &&
            queue_family_property.queue_family_properties.queue_count >= 1
        {
            return true;
        }
    }

    false
}

/// Returns a list of Physical GPUs that are `lz-vk` compatible
/// 
/// **NOTE**: If the provided application name isnt valid UTF-8, the default string value
/// will be used as its name in the logs
pub fn get_supported_physical_gpus(
    app_ctx: &ApplicationContext
) -> VkResult<Vec<PhysicalDevice>> {
    let all_physical_devices = unsafe { app_ctx.instance.enumerate_physical_devices() };

    match all_physical_devices {
        Err(e) => {
            log!(
                get_supported_physical_gpus,
                AndroidLogPriority::Error,
                "[AppName: {}]: {:?}",
                app_ctx.get_application_name(),
                e
            );
            Err(e)
        },
        Ok(all_physical_devices) => {
            let supported_gpus = all_physical_devices.into_iter()
                .filter(|physical_device| {
                    let mut device_properties2 = PhysicalDeviceProperties2::default();

                    unsafe {
                        app_ctx.instance
                            .get_physical_device_properties2(
                                *physical_device,
                                &mut device_properties2
                            )
                    };

                    if is_vulkan_baseline_compatible(&device_properties2) {
                        if
                            is_gpu(&device_properties2) &&
                            is_compute_queue_supported(&app_ctx.instance, physical_device)
                        {
                            log!(
                                get_supported_physical_gpus,
                                AndroidLogPriority::Info,
                                "[AppName: {}]: true",
                                app_ctx.get_application_name()
                            );
                            true
                        } else {
                            log!(
                                get_supported_physical_gpus,
                                AndroidLogPriority::Info,
                                "[AppName: {}]: false",
                                app_ctx.get_application_name()
                            );
                            false
                        }
                    } else {
                        log!(
                            get_supported_physical_gpus,
                            AndroidLogPriority::Info,
                            "[AppName: {}]: false",
                            app_ctx.get_application_name()
                        );
                        false
                    }
                })
                .collect();

            Ok(supported_gpus)
        }
    }
}

/// Returns all GPU extension properties
/// 
/// **NOTE**: If the provided application name isnt valid UTF-8, the default string value
/// will be used as its name in the logs
pub fn get_gpu_extension_properties(
    app_ctx: &ApplicationContext,
    selected_gpu: &PhysicalDevice
) -> VkResult<Vec<ExtensionProperties>> {
    let gpu_extensions = unsafe {
        app_ctx.instance.enumerate_device_extension_properties(*selected_gpu)
    };

    match gpu_extensions {
        Err(e) => {
            log!(
                get_gpu_extension_properties,
                AndroidLogPriority::Error,
                "[AppName: {}]: {:?}",
                app_ctx.get_application_name(),
                e
            );
            Err(e)
        },
        Ok(gpu_extensions) => Ok(gpu_extensions)
    }
}

/// Returns the names of extensions supported by the GPU.
/// 
/// Ignores the extensions whose names arent null terminated or
/// dont contain valid UTF-8
pub fn get_gpu_extension_names(
    gpu_extensions: &[ExtensionProperties]
) -> Vec<&str> {
    gpu_extensions
        .iter()
        .filter_map(|ext| ext.extension_name_as_c_str().ok()?.to_str().ok())
        .collect()
}