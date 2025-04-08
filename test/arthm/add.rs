#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#[allow(dead_code)]
include!("../common.rs");
#[inline(never)]
#[no_mangle]
fn mag(x: f32, y: f32) -> f32 {
    x * x + y * y
}
fn main() {
    let cons = black_box(18_446_744_073_709_551_615u64);
    let cons = black_box(u128::MAX);
    let cons = black_box(34163_u16);
    let flot = black_box(353136643_i32) as f32;
    let int = black_box(flot) as u16;
    black_box(mag(black_box(6.6), black_box(5.5)));
    test_eq!(65535_u16, int);
    // Test addition of different types
    test_eq!((black_box(2_i8) + 2) as i32, 4 as i32);

    test_eq!(black_box(2_u8) + 2, 4);
    test_eq!(black_box(2_i16) + 2, 4);
    test_eq!(black_box(2_u16) + 2, 4);
    test_eq!(black_box(2_i32) + 2, 4);
    test_eq!(black_box(2_u32) + 2, 4);

    //test_eq!(black_box(2_u128)+ 2, 4);
    #[cfg(not(debug_assertions))]
    {
        test_eq!(black_box(2_u64) + 2, 4);
        // Signed 64 bit checked add DOES NOT WORK in mono.
        test_eq!(black_box(2_i64) + 2, 4);
    }
    //let slice:&mut [u8] = black_box(unsafe{core::slice::from_raw_parts_mut(black_box(core::ptr::null_mut()),64)});

    test_eq!(black_box(2.0_f32) + 2.0, 4.0);
    test_eq!(black_box(2.0_f64) + 2.0, 4.0);
    // Test addition of negative values
    test_eq!(black_box(2_i8) + black_box(-2), 0);
    test_eq!(black_box(2_i16) + black_box(-2), 0);
    test_eq!(black_box(2_i32) + black_box(-2), 0);
    //test_eq!(black_box(2_i64) + black_box(-2), 0);
    test_eq!(black_box(2_f32) + black_box(-2.0), 0.0);
    test_eq!(black_box(2_f64) + black_box(-2.0), 0.0);
    // Test overflows
    test_eq!(black_box(255_u8).wrapping_add(1_u8), 0_u8);
    test_eq!(black_box(127_i8).wrapping_add(1_i8), -128_i8);
    test_eq!(black_box(65_535u16).wrapping_add(1_u16), 0_u16);
    test_eq!(black_box(32_767i16).wrapping_add(1_i16), -32_768i16);
    test_eq!(black_box(4_294_967_295u32).wrapping_add(1_u32), 0_u32);
    test_eq!(
        black_box(2_147_483_647i32).wrapping_add(1_i32),
        -2_147_483_648i32
    );
    test_eq!(
        black_box(18_446_744_073_709_551_615u64).wrapping_add(1_u64),
        0_u64
    );
    test_eq!(
        black_box(9_223_372_036_854_775_807i64).wrapping_add(1_i64),
        -9_223_372_036_854_775_808i64
    );

    let val = 7818556801315723626_usize as u64;
    let val2 = black_box(val) as isize;
    test_eq!(val2, 7818556801315723626_isize);
    // Test saturating add
    test_eq!(
        core::intrinsics::saturating_add(black_box(2_usize), black_box(2)),
        4
    );
    // Test checked add
    test_eq!(black_box(2_usize).checked_add(black_box(2)), Some(4));
    test_eq!(black_box(2_usize).checked_add(black_box(usize::MAX)), None);
    test_eq!(black_box(2_usize).checked_sub(black_box(2)), Some(0));
    test_eq!(black_box(2_usize).checked_sub(black_box(3)), None);
    // Test saturating sub
    test_eq!(
        core::intrinsics::saturating_sub(black_box(2_usize), black_box(2)),
        0
    );
    unsafe { printf(c"%u\n".as_ptr(), black_box(black_box(!0_u32) as f32) as u32) };
    unsafe { printf(c"%f\n".as_ptr(), black_box(black_box(!0_u32) as f32) as f64) };
    test_eq!(black_box(u128::MAX).wrapping_add(1_u128), 0_u128);
    test_eq!(black_box(i128::MAX).wrapping_add(1_i128), i128::MIN);
}
