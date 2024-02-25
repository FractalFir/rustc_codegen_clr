#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code,improper_ctypes_definitions)]
#![no_std]

include!("../common.rs");
struct Vec<T>{
    ptr:*mut T,
    cap:usize,
    len:usize,
}
impl<T> Drop for Vec<T>{
    fn drop(&mut self){
        /* 
        for idx in 0..self.len{
            unsafe{core::ptr::drop_in_place(self.ptr.wrapping_add(idx))};
        }*/
        unsafe{free(self.ptr as *mut  core::ffi::c_void)};
    }
}
impl<T> Vec<T>{
    pub fn new()->Self{
        Self{ptr:core::ptr::null_mut(),cap:0,len:0}
    }
    pub fn with_capacity(cap:usize)->Self{
        let byte_size = cap * core::mem::size_of::<T>();
        let ptr = unsafe{malloc(byte_size)}.cast();
        Self{ptr,cap,len:0}
    }
    pub fn ptr(&self)->*const T{
        self.ptr.cast()
    }
    pub fn push(&mut self,val:T){
        let new_len = self.len + 1;
        if new_len > self.cap{
            let mut new_cap = self.cap << 1;
            if new_cap < 4{
                new_cap = 4;
            }
            let byte_size = new_cap * core::mem::size_of::<T>();
            self.ptr = unsafe{realloc(self.ptr.cast(),byte_size)}.cast();
            self.cap = new_cap;
        }
        unsafe{self.ptr.offset(self.len as isize).write(val)};
        self.len = new_len;
    }
}
fn main(){
    let mut vec:Vec<u8> = Vec::with_capacity(0x1);
    vec.push('H' as u8);
    vec.push('e' as u8);
    vec.push('l' as u8);
    vec.push('l' as u8);
    vec.push('o' as u8);
    vec.push(',' as u8);
    vec.push(' ' as u8);
    vec.push('.' as u8);
    vec.push('N' as u8);
    vec.push('E' as u8);
    vec.push('T' as u8);
    vec.push('!' as u8);
    vec.push('\n' as u8);
    let ptr:*mut Option<i32> = unsafe{malloc(5).cast()};
    black_box(ptr);
    vec.push(0);
    black_box(&mut vec);
    unsafe{puts(vec.ptr())};
    drop(vec);
}

