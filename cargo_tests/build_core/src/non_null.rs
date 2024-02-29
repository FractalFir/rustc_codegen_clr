use crate::{malloc, printf};
pub fn non_null() {
    unsafe {
        let size = 4;
        if size == 0 {
            printf("sizeof int32 is %d\n\0".as_ptr() as *const i8, size);
            core::intrinsics::abort();
        }
        let raw_ptr = malloc(size) as *mut i32;
        if raw_ptr == core::ptr::null_mut() {
            printf("raw_ptr is %p\n\0".as_ptr() as *const i8, raw_ptr);
            core::intrinsics::abort();
        }
        printf("ptr is %p\n\0".as_ptr() as *const i8, raw_ptr);
        let mut ptr = core::ptr::NonNull::new(raw_ptr).unwrap();
        *ptr.as_mut() = -64;
    }
}
