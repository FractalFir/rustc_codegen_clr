#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
#![no_std]

include!("../common.rs");
fn main() {
    const A: u32 = 0b0101100;
    const B: u32 = 0b0100001;
    const C: u32 = 0b1111001;

    const _0: u32 = 0;
    const _1: u32 = !0;

    unsafe {
        printf(
            c" %x rol = %x\n".as_ptr(),
            A as u32,
            A.rotate_left(6) as u32,
        )
    };
    test_eq!(A.rotate_left(6), 2816);
    test_eq!(A.rotate_left(6).rotate_right(2), 704);

    test_eq!(A.rotate_left(6).rotate_right(2).rotate_right(4), A);
    test_eq!(B.rotate_left(3).rotate_left(2).rotate_right(5), B);
    test_eq!(C.rotate_left(6).rotate_right(2).rotate_right(4), C);

    // Rotating these should make no difference
    //
    // We test using 124 bits because to ensure that overlong bit shifts do
    // not cause undefined behaviour. See #10183.
    test_eq!(_0.rotate_left(124), _0);
    test_eq!(_1.rotate_left(124), _1);
    test_eq!(_0.rotate_right(124), _0);
    test_eq!(_1.rotate_right(124), _1);

    // Rotating by 0 should have no effect
    test_eq!(A.rotate_left(0), A);
    test_eq!(B.rotate_left(0), B);
    test_eq!(C.rotate_left(0), C);
    // Rotating by a multiple of word size should also have no effect
    test_eq!(A.rotate_left(128), A);
    test_eq!(B.rotate_left(128), B);
    test_eq!(C.rotate_left(128), C);

    // Test if bitreverse is correct.
    fn bitreverse(b: u8) -> u8 {
        ((b as u64 * 0x0202020202 & 0x010884422010) % 1023) as u8
    }
    fn bitreverse_u16(input: u16) -> u16 {
        bitreverse(input as u8) as u16 * 256 + bitreverse((input / 256) as u8) as u16
    }
    for b in 0..u8::MAX {
        assert_eq!(bitreverse(b), core::intrinsics::bitreverse(b));
    }
    for b in 0..u16::MAX {
        test_eq!(bitreverse_u16(b), core::intrinsics::bitreverse(b));
    }
}
