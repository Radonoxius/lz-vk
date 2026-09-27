use std::{ffi::CStr, ptr::null};

use ash::{Entry, Instance, vk::ApplicationInfo};

pub mod instance;

pub mod physical_gpu;
pub mod logical_gpu;

pub struct ApplicationContext<'a> {
    pub(crate) entry: Entry,
    pub(crate) app_info: ApplicationInfo<'a>,
    pub(crate) instance: Instance
}

impl<'a> ApplicationContext<'a> {
    /// Creates a new `ApplicationContext`
    pub fn new(
        entry: Entry,
        app_info: ApplicationInfo<'a>,
        instance: Instance
    ) -> Self {
        Self { entry, app_info, instance }
    }

    /// Returns a shared reference to the associated `Entry`
    pub fn entry(&self) -> &Entry {
        &self.entry
    }

    /// Returns a shared reference to the associated `Instance`
    pub fn instance(&self) -> &Instance {
        &self.instance
    }

    /// Used to set the name of the application
    pub fn set_application_name(&mut self, name: &CStr) {
        self.app_info.p_application_name = name.as_ptr();
    }

    /// Returns the name of the Application.
    /// 
    /// **NOTE**: This returns the default string if name isnt valid UTF-8.
    pub fn get_application_name(&self) -> &str {
        if self.app_info.p_application_name == null() {
            Default::default()
        } else {
            let app_name = unsafe {
                CStr::from_ptr(self.app_info.p_application_name)
            }.to_str();
            match app_name {
                Err(_) => Default::default(),
                Ok(app_name) => app_name
            }
        }
    }
}