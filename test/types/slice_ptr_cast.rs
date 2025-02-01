#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn test_ref_deref() {
    let mut eight_u8s: u64 = black_box(0);
    let ptr = core::ptr::addr_of_mut!(eight_u8s);
    let size = black_box(8);
    let mut slice = (ptr, size);
    let slice_ref = unsafe {
        <*const [u8]>::as_ref(*(core::ptr::addr_of_mut!(slice) as *mut _ as *mut *mut [u8]))
            .unwrap()
    };
    let slice_ref = black_box(slice_ref);
    //slice_ref[0] = 1;
    //let slice_ptr = slice_ref as *const [u8];
    // cast slice pointer back to *mut u8
    // let uint8_ptr = {slice_ptr as *mut u8};
    //black_box(uint8_ptr);
    return;
}
fn main() {
    test_ref_deref();
    return;
}
