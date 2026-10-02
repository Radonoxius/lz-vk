use std::ffi::{CStr, c_char};

/// Converts a slice of `&CStr` to a vector of `*const c_char`
pub(crate) fn get_raw_cstr_ptrs(cstr_slice: &[&CStr]) -> Vec<*const c_char> {
    cstr_slice
        .iter()
        .map(|cstr_ref| cstr_ref.as_ptr())
        .collect()
}