#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    unsized_const_params,
    maybe_uninit_slice
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#[allow(dead_code)]
include!("../common.rs");
extern "C"{
    fn ldexp(x: f64, exp: i32) -> f64;
}
pub fn ldexp_f64(x: f64, mut exp: i32) -> f64 {
    unsafe{ldexp(x,exp)}
}
#[no_mangle]
pub fn estimate_scaling_factor(mant: u64, exp: i16) -> i16 {
    // 2^(nbits-1) < mant <= 2^nbits if mant > 0
    let nbits = 64 - (mant - 1).leading_zeros() as i64;
    // 1292913986 = floor(2^32 * log_10 2)
    // therefore this always underestimates (or is exact), but not much.
    (((nbits + exp as i64) * 1292913986) >> 32) as i16
}
fn main(){
    macro_rules! assert_almost_eq {
        ($actual:expr, $expected:expr) => {{
            let actual = $actual;
            let expected = $expected;
            unsafe{printf(
                c"%s - %s = %d - %d = %d".as_ptr(),
                concat!(stringify!($expected),"\0").as_ptr() as *const _,
                concat!(stringify!($actual),"\0").as_ptr() as *const _,
                expected as i32,
                actual as i32,
                expected - actual as i32
            )};
            test!(
                expected as i32 == actual as i32 || expected as i32 == (actual + 1)as i32
            );
        }};
    }

    assert_almost_eq!(estimate_scaling_factor(1, 0), 0);
    assert_almost_eq!(estimate_scaling_factor(2, 0), 1);
    assert_almost_eq!(estimate_scaling_factor(10, 0), 1);
    assert_almost_eq!(estimate_scaling_factor(11, 0), 2);
    assert_almost_eq!(estimate_scaling_factor(100, 0), 2);
    assert_almost_eq!(estimate_scaling_factor(101, 0), 3);
    assert_almost_eq!(estimate_scaling_factor(10000000000000000000, 0), 19);
    assert_almost_eq!(estimate_scaling_factor(10000000000000000001, 0), 20);

    // 1/2^20 = 0.00000095367...
    assert_almost_eq!(estimate_scaling_factor(1 * 1048576 / 1000000, -20), -6);
    assert_almost_eq!(estimate_scaling_factor(1 * 1048576 / 1000000 + 1, -20), -5);
    assert_almost_eq!(estimate_scaling_factor(10 * 1048576 / 1000000, -20), -5);
    assert_almost_eq!(estimate_scaling_factor(10 * 1048576 / 1000000 + 1, -20), -4);
    assert_almost_eq!(estimate_scaling_factor(100 * 1048576 / 1000000, -20), -4);
    assert_almost_eq!(estimate_scaling_factor(100 * 1048576 / 1000000 + 1, -20), -3);
    assert_almost_eq!(estimate_scaling_factor(1048575, -20), 0);
    assert_almost_eq!(estimate_scaling_factor(1048576, -20), 0);
    assert_almost_eq!(estimate_scaling_factor(1048577, -20), 1);
    assert_almost_eq!(estimate_scaling_factor(10485759999999999999, -20), 13);
    assert_almost_eq!(estimate_scaling_factor(10485760000000000000, -20), 13);
    assert_almost_eq!(estimate_scaling_factor(10485760000000000001, -20), 14);

    // extreme values:
    // 2^-1074 = 4.94065... * 10^-324
    // (2^53-1) * 2^971 = 1.79763... * 10^308
    assert_almost_eq!(estimate_scaling_factor(1, -1074), -323);
    assert_almost_eq!(estimate_scaling_factor(0x1fffffffffffff, 971), 309);

    // Miri is too slow
    let step = if cfg!(miri) { 37 } else { 1 };

    for i in (-1074..972).step_by(step) {
        let expected = ldexp_f64(1.0, i).log10().ceil();
        assert_almost_eq!(estimate_scaling_factor(1, i as i16) as i32, expected as i16 as i32);
    }
}