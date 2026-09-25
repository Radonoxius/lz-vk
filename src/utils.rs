use std::{ffi::{CStr, c_char}, mem::transmute};

/// Converts a slice of `&CStr` to a slice of `*const c_char`
/// 
/// # Safety
/// `cstr_slice` and its contents must be valid!
pub(crate) unsafe fn get_raw_cstr_slice<'a>(cstr_slice: &[&'a CStr]) -> &'a [*const c_char] {
    unsafe {
        transmute::<&[&CStr], &[*const c_char]>(cstr_slice)
    }
}