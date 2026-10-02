use std::ffi::{CStr, c_void};

use ash::{Device, prelude::VkResult, vk::{DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice}};

#[allow(unused)]
use crate::logging::android::AndroidLogPriority;

use crate::{init::ApplicationContext, logging::log, utils::get_raw_cstr_ptrs};

/// Returns a logical GPU using the physical GPU provided.
/// 
/// **NOTE**: If the provided application name isnt valid UTF-8, the default string value
/// will be used as its name in the logs
/// 
/// # Safety
/// The `p_next` parameters must either be `nullptrs` or
/// must be valid!
pub unsafe fn create_logical_gpu(
    app_ctx: &ApplicationContext,
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

    let enabled_extension_names = get_raw_cstr_ptrs(device_create_info_enabled_extension_names);
    let logical_gpu_create_info = DeviceCreateInfo {
        p_next: device_create_info_p_next,
        ..Default::default()
    }.queue_create_infos(
        &queue_create_infos
    ).enabled_extension_names(
        &enabled_extension_names
    );

    let logical_gpu = unsafe {
        app_ctx.instance.create_device(
            *selected_gpu,
            &logical_gpu_create_info,
            None
        )
    };

    match logical_gpu {
        Err(e) => {
            log!(
                create_logical_gpu,
                AndroidLogPriority::Error,
                "[AppName: {}]: {:?}",
                app_ctx.get_application_name(),
                e
            );
            Err(e)
        },
        Ok(logical_gpu) => Ok(logical_gpu)
    }
}