use crate::*;
use std::ffi::CStr;
pub fn test_cstr() {
    unsafe {
        // This test used to fail becasue the call to libc strlen always returned 0. This happened because preservesig was not set, and the return value was replaced by a HRESULT.
        let slice = CStr::from_ptr("Testing cstr!\n\0".as_ptr() as *const i8);
        printf(slice.as_ptr() as *const i8);
    }
}
