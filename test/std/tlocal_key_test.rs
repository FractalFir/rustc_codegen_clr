#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    let_chains,
    never_type
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code,unused_imports,unused_mut,private_interfaces,non_upper_case_globals,unused_unsafe)]
#![no_std]
#[allow(dead_code)]
mod tlocal_key;
use core::cell::Cell;
use tlocal_key::*;

include!("../common.rs");
use core::mem::MaybeUninit;

fn to_static<T: Copy>(tmp: Option<&mut Option<Cell<T>>>) -> Option<&'static Cell<T>> {
    Put::putnl(0xC0FE_BEFF_u32);
    let mut static_val = unsafe {
        let mut alloc = &mut *(__rust_alloc(
            core::mem::size_of::<Option<Cell<T>>>(),
            core::mem::align_of::<Option<Cell<T>>>(),
        ) as *mut MaybeUninit<Option<Cell<T>>>);
        alloc.write(None);
        alloc.assume_init_mut()
    };
    
    if let Some(tmp) = tmp {
        if let Some(tmp) = tmp.as_ref() {
            *static_val = Some(Cell::new(tmp.get()));
        }
    }
    
    //Put::putnl((static_val.as_ref().unwrap()) as *const _ as usize);
    Put::putnl(0xC0FE_BABE_u32);
    static_val.as_ref()
}
fn main() {
    static KEY: LocalKey<Cell<u8>> = unsafe {
        LocalKey::new({
            let ts: unsafe fn(tmp: Option<&mut Option<Cell<u8>>>) -> Option<&'static Cell<u8>> =
                to_static::<u8>;
            ts
        })
    };
    KEY.set(64);
}
