#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
#[allow(dead_code)]
struct Test<T> {
    data: T,
}
include!("../common.rs");
fn equivalent_offset(val: u8, off: i16) -> i16 {
    for i in 0..8 {
        if val << i == val << off {
            return i;
        }
    }
    panic!()
}
fn main() {
    /* 
    #[cfg(not(debug_assertions))]
    {
        let val = 105_u8;
        let off = -1973_i16;
        for off in -32..32_i16 {
            let est_off = off as u32 % 8;
            test_eq!(est_off, equivalent_offset(val, off) as u32)
        }
    }*/
    #[cfg(not(debug_assertions))]
    {
        let val = 104_u8;
        let off = -9_i16;
        let shift_res = black_box(val) >> black_box(off);
        black_box(shift_res);
        test_eq!(shift_res, 0);
    }
    #[cfg(not(debug_assertions))]
    {
        let val = 25907_u16;
        let off = 0x4c4c0ad961f67741cee4d2d40cd22b7_i128;
        let shift_res = black_box(val) << black_box(off);
        black_box(shift_res);
        test_eq!(shift_res, 39296);
    }
    #[cfg(not(debug_assertions))]
    {
        let val = 104_u8;
        let off = -1973_i16;
        let shift_res = black_box(val) >> black_box(off);
        black_box(shift_res);
        test_eq!(shift_res, 13);
    }

    black_box(());
}
