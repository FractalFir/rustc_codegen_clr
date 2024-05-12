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
    if let Some((first,rem)) = oslice.split_first(){
        unsafe{printf(rem.as_ptr() as *const i8)};
        test_eq!(rem.len(),oslice.len() - 1);
        unsafe{printf("%c\n\0".as_ptr() as *const i8,*first  as i32)};
    }
    test_eq!(oslice.split_first(),Some((&b'H',b"ello, World\n\0")));
    dump_var(0,0,true,1,1,2,2,3,false);
    
}
#[inline(never)]
    fn dump_var(
        f: usize,
        var0: usize, val0: impl PrintFDebug,
        var1: usize, val1: impl PrintFDebug,
        var2: usize, val2: impl PrintFDebug,
        var3: usize, val3: impl PrintFDebug,
    ) {
        unsafe{
            printf("fn%u:_%u = \0".as_ptr() as *const i8,f,var0);
            val0.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const i8,var1);
            val1.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const i8,var2);
            val2.printf_debug();
            printf("\n_%u = \0".as_ptr() as *const i8,var3);
            val3.printf_debug();
            printf("\n\0".as_ptr() as *const i8);
        }
    }
    trait PrintFDebug{
        unsafe fn printf_debug(&self);
    }
    impl PrintFDebug for u8{
        unsafe fn printf_debug(&self){
            printf("%u\0".as_ptr() as *const i8,*self as u8 as i32);
        }
    } 
    impl PrintFDebug for bool{
        unsafe fn printf_debug(&self){
            if *self{
                printf("true\0".as_ptr() as *const i8);
            }
            else{
                printf("false\0".as_ptr() as *const i8);
            }
        }
    } 
    impl PrintFDebug for (){
        unsafe fn printf_debug(&self){
            printf("()\0".as_ptr() as *const i8);
        }
    } 