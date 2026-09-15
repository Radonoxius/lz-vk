use ash::{Entry, Instance, prelude::VkResult, vk::{ApplicationInfo, InstanceCreateFlags, InstanceCreateInfo, KHR_PORTABILITY_ENUMERATION_NAME, PhysicalDevice}};

use crate::{LAYER_KHRONOS_VALIDATION_NAME, logging::log, utils::get_application_name};

/// SAFETY: `app_info.p_application_name` must be null terminated!
pub unsafe fn init(
    entry: &Entry,
    app_info: &ApplicationInfo,
    enable_debug_validation: bool,
    enbale_portability_enumeration: bool
) -> VkResult<Instance> {
    let instance_info;
    let applicaion_name = unsafe { get_application_name(app_info) };

    let enabled_layers = [LAYER_KHRONOS_VALIDATION_NAME.as_ptr()];
    let enabled_instance_extensions = [KHR_PORTABILITY_ENUMERATION_NAME.as_ptr()];
    
    if enable_debug_validation && enbale_portability_enumeration {
        instance_info = InstanceCreateInfo::default()
            .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
            .application_info(app_info)
            .enabled_layer_names(&enabled_layers)
            .enabled_extension_names(&enabled_instance_extensions);
    } else if enable_debug_validation && !enbale_portability_enumeration {
        instance_info = InstanceCreateInfo::default()
            .application_info(app_info)
            .enabled_layer_names(&enabled_layers)
    } else if !enable_debug_validation && enbale_portability_enumeration {
        instance_info = InstanceCreateInfo::default()
            .flags(InstanceCreateFlags::ENUMERATE_PORTABILITY_KHR)
            .application_info(app_info)
            .enabled_extension_names(&enabled_instance_extensions);
    } else {
        instance_info = InstanceCreateInfo::default()
            .application_info(app_info);
    }
    
    let instance = unsafe {
        entry.create_instance(&instance_info, None)
    };

    if let Err(e) = instance {
        log!(
            init,
            "[AppName: {}]: {:?}",
            applicaion_name,
            e
        );
    } else {
        log!(
            init,
            "[AppName: {}]: enable_debug_validation: {}, enbale_portability_enumeration: {}",
            applicaion_name,
            enable_debug_validation,
            enbale_portability_enumeration
        );
    }

    instance
}

pub fn get_physical_gpus(
    instance: &Instance
) -> VkResult<Vec<PhysicalDevice>> {
    let physical_gpus = unsafe { instance.enumerate_physical_devices() };

    if let Err(e) = physical_gpus {
        log!(
            get_physical_gpus,
            "{:?}",
            e
        );
    }

    physical_gpus
}