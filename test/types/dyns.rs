#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
#[derive(Clone,Copy)]
#[repr(C)]
struct Vtable{
    drop_in_place: fn(*mut ()),
    size: usize,
    alignment: usize,
    be_fun: fn(*mut ()),
}
pub trait FunTrait{
    fn be_fun(&self);
}
struct Bob(pub u32,pub u32,pub u32,);
impl FunTrait for Bob{
    fn be_fun(&self){
        unsafe{printf("Bob is a funny guy\n\0".as_ptr() as *const i8)};
    }
}
pub fn test(fun:Option<&dyn FunTrait>){
    match black_box(fun){
        Some(fun)=>{
            let ptrs :(*mut (), *mut Vtable) = unsafe{core::mem::transmute::<_,(*mut (), *mut Vtable)>(fun)};
            test_ne!(ptrs.0,core::ptr::null_mut());
            test_ne!(ptrs.1,core::ptr::null_mut());
            let vtable = unsafe{*ptrs.1};
            unsafe{printf("Obj size:%p \n\0".as_ptr() as *const i8,vtable.size)};
            unsafe{printf("Obj alignment:%p \n\0".as_ptr() as *const i8,vtable.alignment)};
            test_eq!(vtable.size,core::mem::size_of::<Bob>());
            (vtable.be_fun)(ptrs.0);
            test_eq!(vtable.alignment,4);
            //fun.be_fun()
        }
        _=>(),
    }
    
}
fn main(){
    test(None);
    let bob = Bob(64,64,32);
    bob.be_fun();
    test(Some(&bob as &dyn FunTrait));
}


