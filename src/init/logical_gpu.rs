use std::ffi::{CStr, c_void};

use ash::{Device, Instance, prelude::VkResult, vk::{DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{logging::log, utils::get_raw_cstr_slice};


/// Returns a logical GPU using the physical GPU provided.
/// 
/// # Safety
/// The `p_next` parameters must either be `nullptrs` or 
/// must be valid!
pub unsafe fn create_logical_gpu(
    instance: &Instance,
    selected_gpu: &PhysicalDevice,

    queue_create_info_p_nexts: &[*const c_void],
    queue_family_index: &[u32],
    queue_count: &[u32],
    queue_priorities: &[&[f32]],

    device_create_info_p_next: *const c_void,
    device_create_info_enabled_extension_names: &[&CStr]
) -> VkResult<Device> {
    let mut queue_create_infos = Vec::with_capacity(queue_family_index.len());

    for i in 0..queue_create_infos.capacity() {
        queue_create_infos.push(
            DeviceQueueCreateInfo {
                p_next: queue_create_info_p_nexts[i],
                queue_family_index: queue_family_index[i],
                queue_count: queue_count[i],
                ..Default::default()
            }.queue_priorities(queue_priorities[i])
        );
    }

    let logical_gpu_create_info = DeviceCreateInfo {
        p_next: device_create_info_p_next,
        ..Default::default()
    }.queue_create_infos(
        &queue_create_infos
    ).enabled_extension_names(
        unsafe { get_raw_cstr_slice(device_create_info_enabled_extension_names) }
    );

    let logical_gpu = unsafe {
        instance.create_device(
            *selected_gpu,
            &logical_gpu_create_info,
            None
        )
    };

    match logical_gpu {
        Err(e) => {
            log!(
                get_logical_gpu,
                AndroidLogPriority::Error,
                "{:?}",
                e
            );
            Err(e)
        },
        Ok(logical_gpu) => Ok(logical_gpu)
    }
}