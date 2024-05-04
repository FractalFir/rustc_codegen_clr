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
    test_fn: fn(*mut ()),
}
pub trait TestTrait{
    fn test_fn(&self);
    fn test_fn2(&self)->u32;
    fn test_ident(&self,ident:u32)->u32;
}
struct TestObject(pub u32,pub u32,pub u32,);
impl TestTrait for TestObject{
    fn test_fn(&self){
        unsafe{printf("TestObject is a funny thing. It has the following fields: %d %d %d \n\0".as_ptr() as *const i8,self.0,self.1,self.2)};
    }
    fn test_fn2(&self)->u32{
       0xDEAD_BEEF
    }
    fn test_ident(&self,ident:u32)->u32{
        ident
    }
}
pub fn test(fun:Option<&dyn TestTrait>){
    match black_box(fun){
        Some(fun)=>{
            let ptrs :(*mut (), *mut Vtable) = unsafe{core::mem::transmute::<_,(*mut (), *mut Vtable)>(fun)};
            test_ne!(ptrs.0,core::ptr::null_mut());
            test_ne!(ptrs.1,core::ptr::null_mut());
            let vtable = unsafe{*ptrs.1};
            unsafe{printf("Obj size:%p \n\0".as_ptr() as *const i8,vtable.size)};
            unsafe{printf("Obj alignment:%p \n\0".as_ptr() as *const i8,vtable.alignment)};
            test_eq!(vtable.size,core::mem::size_of::<TestObject>());
            (vtable.test_fn)(ptrs.0);
            test_eq!(vtable.alignment,4);
           
            fun.test_fn();
            test_eq!(fun.test_fn2(),0xDEAD_BEEF);
            test_eq!(fun.test_ident(0xDEAD_BEEF),0xDEAD_BEEF);
        }
        _=>(),
    } 
}
fn main(){
    test(None);
    let bob = TestObject(64,64,32);
    bob.test_fn();
    test(Some(&bob as &dyn TestTrait));
}


