#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params,
    isqrt
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports
)]
#![no_std]
use core::num::NonZero;
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
        test_eq!(bitreverse(b), core::intrinsics::bitreverse(b));
    }
    for b in 0..u16::MAX {
        test_eq!(bitreverse_u16(b), core::intrinsics::bitreverse(b));
    }
    pub fn bitreverse_u64(mut n: u64) -> u64 {
        n = (n >> 1) & 0x5555555555555555u64 | (n & 0x5555555555555555u64) << 1;
        n = (n >> 2) & 0x3333333333333333u64 | (n & 0x3333333333333333u64) << 2;
        n = (n >> 4) & 0x0F0F0F0F0F0F0F0Fu64 | (n & 0x0F0F0F0F0F0F0F0Fu64) << 4;
        n = (n >> 8) & 0x00FF00FF00FF00FFu64 | (n & 0x00FF00FF00FF00FFu64) << 8;
        n = (n >> 16) & 0x0000FFFF0000FFFFu64 | (n & 0x0000FFFF0000FFFFu64) << 16;
        (n >> 32) & 0x00000000FFFFFFFFu64 | (n & 0x00000000FFFFFFFFu64) << 32
    }
    #[cfg(not(debug_assertions))]
    fn bitreverse_u128(mut n: u128) -> u128 {
        n = (n >> 1) & 0x55555555555555555555555555555555u128
            | (n & 0x55555555555555555555555555555555u128) << 1;
        n = (n >> 2) & 0x33333333333333333333333333333333u128
            | (n & 0x33333333333333333333333333333333u128) << 2;
        n = (n >> 4) & 0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0Fu128
            | (n & 0x0F0F0F0F0F0F0F0F0F0F0F0F0F0F0F0Fu128) << 4;
        n = (n >> 8) & 0x00FF00FF00FF00FF00FF00FF00FF00FFu128
            | (n & 0x00FF00FF00FF00FF00FF00FF00FF00FFu128) << 8;
        n = (n >> 16) & 0x0000FFFF0000FFFF0000FFFF0000FFFFu128
            | (n & 0x0000FFFF0000FFFF0000FFFF0000FFFFu128) << 16;
        n = (n >> 32) & 0x00000000FFFFFFFF00000000FFFFFFFFu128
            | (n & 0x00000000FFFFFFFF00000000FFFFFFFFu128) << 32;
        n = (n >> 64) | (n << 64);
        n
    }
    fn bitreverse_u32(mut n: u32) -> u32 {
        n = ((n >> 1) & 0x55555555) | ((n & 0x55555555) << 1);
        n = ((n >> 2) & 0x33333333) | ((n & 0x33333333) << 2);
        n = ((n >> 4) & 0x0F0F0F0F) | ((n & 0x0F0F0F0F) << 4);
        n = ((n >> 8) & 0x00FF00FF) | ((n & 0x00FF00FF) << 8);
        n = (n >> 16) | (n << 16);
        n
    }
    #[cfg(not(debug_assertions))]
    for b in 0..u16::MAX {
        let b = b as u128 + b as u128 * (u64::MAX as u128);
        test_eq!(bitreverse_u128(b), core::intrinsics::bitreverse(b));
    }

    for b in 0..u16::MAX {
        let b = b as u64 + b as u64 * (u32::MAX as u64);
        test_eq!(bitreverse_u64(b), core::intrinsics::bitreverse(b));
    }
    for b in 0..u16::MAX {
        let b = b as u32 + b as u32 * (u16::MAX as u32);
        test_eq!(bitreverse_u32(b), core::intrinsics::bitreverse(b));
    }
    #[cfg(not(debug_assertions))]
    for b in 0..u16::MAX {
        let b = b as i128 + b as i128 * (u64::MAX as i128);
        test_eq!(
            bitreverse_u128(b as u128) as i128,
            core::intrinsics::bitreverse(b)
        );
    }
    test_nonzerou128(2);
    test_eq!(isqrt(black_box(0 as u128)), 0 as u128);

    test!(matches!(NonZero::new(4), Some(_)));

    /*test_eq!((0 as u128).isqrt(), 0 as u128);*/
    /*test_eq!((1 as u128).isqrt(), 1 as u128);*/
    Put::putnl(isqrt(black_box(2 as u128)));
    test_eq!(isqrt(black_box(2 as u128)), 1 as u128);

    /*test_eq!((99 as u128).isqrt(), 9 as u128);
    test_eq!((100 as u128).isqrt(), 10 as u128);
    test_eq!(u128::MAX.isqrt(), (1 << (u128::BITS / 2)) - 1);
    ;
    test_eq!((1 as u128).isqrt(), 1 as u128);
    test_eq!((2 as u128).isqrt(), 1 as u128);
    test_eq!((99 as u128).isqrt(), 9 as u128);
    test_eq!((100 as u128).isqrt(), 10 as u128);
    test_eq!(u128::MAX.isqrt(), (1 << (u128::BITS / 2)) - 1);*/
}
pub fn test_nonzerou128(v: u128) {
    match NonZero::new(v) {
        Some(x) => unsafe {
            printf(c"NonZero returned Some for %u\n".as_ptr(), v as u32);
        },
        None => {
            unsafe { printf(c"NonZero returned None for %u\n".as_ptr(), v as u32) };
        }
    }
}
pub fn isqrt(v: u128) -> u128 {
    match NonZero::new(v) {
        Some(x) => isqrt_nonzero(x).get(),
        None => {
            unsafe { printf(c"NonZero returned None for %u\n".as_ptr(), v as u32) };
            0
        }
    }
}

pub fn isqrt_nonzero(v: NonZero<u128>) -> NonZero<u128> {
    // The algorithm is based on the one presented in
    // <https://en.wikipedia.org/wiki/Methods_of_computing_square_roots#Binary_numeral_system_(base_2)>
    // which cites as source the following C code:
    // <https://web.archive.org/web/20120306040058/http://medialab.freaknet.org/martin/src/sqrt/sqrt.c>.

    let mut op = v.get();
    let mut res = 0;
    let mut one = 1 << (v.ilog2() & !1);
    Put::putnl(3.14);
    Put::putnl(one);
    Put::putnl(2.5);
    while one != 0 {
        if op >= res + one {
            op -= res + one;
            res = (res >> 1) + one;
        } else {
            res >>= 1;
        }
        one >>= 2;
    }

    // SAFETY: The result fits in an integer with half as many bits.
    // Inform the optimizer about it.
    unsafe { core::hint::assert_unchecked(res < 1 << (NonZero::<u128>::BITS / 2)) };

    // SAFETY: The square root of an integer >= 1 is always >= 1.
    unsafe { NonZero::new_unchecked(res) }
}
