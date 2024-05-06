#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start,ascii_char)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
fn main(){
    let ptr = unsafe{malloc(64) as *mut _};
    black_box(ptr);
    let slice:&mut [u8] = unsafe{core::slice::from_raw_parts_mut(ptr,64)};
    let len = slice.len();
    let first = slice[black_box(0)];
    Put::putnl(len as u64);
    Put::putnl(first);
    slice[black_box(0)] = 'H' as u8;
    slice[black_box(1)] = 'e' as u8;
    slice[black_box(2)] = 'l' as u8;
    slice[black_box(3)] = 'l' as u8;
    slice[black_box(4)] = 'o' as u8;
    slice[black_box(5)] = '.' as u8;
    slice[black_box(6)] = '\n' as u8;
    slice[black_box(7)] = 0;
    unsafe{puts(ptr)};
    black_box(&slice);
    let oslice = b"Hello, World\n\0";
    test_eq!(oslice.len(),14);
    test_eq!(oslice.last(),Some(&b'\0'));
     //TODOD: Split this bug into separate test case
      /* 
    let slice = slice.as_ascii();
    let slice = if let Some(slice) = slice{
        slice
    }else{
       
       

        unsafe{puts(oslice.as_ptr() as *const _ )};
        for c in oslice.chars(){
            test_eq!(c.is_ascii(),true);
        }
       
        if black_box(false){
            loop{}
        }
        else{
            core::intrinsics::abort();
        }
    };
    test_eq!(slice.len(),14);
    test_eq!(slice[0],core::ascii::Char::from_u8(72).unwrap());
    */
}
