use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, InstanceCreateFlags, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME, PhysicalDevice, PhysicalDeviceProperties2, PhysicalDeviceType, QueueFamilyProperties2, QueueFlags, api_version_major, api_version_minor}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{LAYER_KHRONOS_VALIDATION_NAME, LZVK_BASELINE_MAJOR, LZVK_BASELINE_MINOR, init_utils::get_application_name, logging::log};

/// Initializes a Vulkan instance based on the given parameters
/// 
/// SAFETY: `entry` must be valid & `app_info.p_application_name` must be null terminated & valid UTF-8!
pub unsafe fn init(
    entry: &Entry,
    app_info: &ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<Instance> {
    let mut instance_info = InstanceCreateInfo::default();
    let applicaion_name = unsafe { get_application_name(app_info) };

    let enabled_layers = [LAYER_KHRONOS_VALIDATION_NAME.as_ptr()];
    let enabled_instance_extensions = [KHR_PORTABILITY_ENUMERATION_NAME.as_ptr()];
    
    if enable_debug_validation {
        instance_info = instance_info.enabled_layer_names(&enabled_layers);
    }
    if enbale_portability_enumeration {
        instance_info = instance_info
            .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
            .enabled_extension_names(&enabled_instance_extensions);
    }
    
    let instance = unsafe {
        entry.create_instance(&instance_info, None)
    };

    match instance {
        Err(e) => log!(
            init,
            AndroidLogPriority::Error,
            "[AppName: {}]: {:?}",
            applicaion_name,
            e
        ),
        _ => log!(
            init,
            AndroidLogPriority::Info,
            "[AppName: {}]: enable_debug_validation: {}, enbale_portability_enumeration: {}",
            applicaion_name,
            enable_debug_validation,
            enbale_portability_enumeration
        )
    }

    instance
}

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
pub fn get_supported_gpus(
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