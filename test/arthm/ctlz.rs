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
#[no_mangle]
fn u128max_ovf_check() {
    test!(std::hint::black_box(2_i128).checked_pow(2).is_some());
    test!(
        !0x0ffffffffffffffff_u128
            .overflowing_mul(0x0ffffffffffffffff_u128)
            .1
    );
}
#[no_mangle]
#[inline(never)]
fn option_const() {
    const OPTION: Option<usize> = Some(32);
    const REF: Option<&usize> = OPTION.as_ref();
    assert_eq!(REF, Some(&32));
}
#[no_mangle]
#[inline(never)]
fn const_opt() {
    const OPTION: Option<usize> = Some(32);
    const REF: Option<&usize> = OPTION.as_ref();
    const COPIED: Option<usize> = OPTION.as_ref().copied();
    test_eq!(COPIED, OPTION);
    test_eq!(*Some(&32).unwrap(), 32);
    test_eq!(REF, Some(&32));
}
#[no_mangle]
fn test_isqrt() {
    let res = sqrts::u128(<u128>::MAX);
    unsafe { printf(c"res:%lx%lx\n".as_ptr(), res as u64, (res >> 64) as u64) };
    test_eq!(res, 0x0ffffffffffffffff_u128);
}
macro_rules! unsigned_fn {
    (u128, $HalfBitsT:ident, $stages:ident) => {
        #[doc = concat!("[`", stringify!(u128), "`](prim@", stringify!(u128), ")")]
        #[must_use = "this returns the result of the operation, \
                      without modifying the original"]
        #[inline]
        pub fn u128(mut n: u128) -> u128 {
            if n <= <$HalfBitsT>::MAX as u128 {
                $HalfBitsT((n as $HalfBitsT).into()) as u128
            } else {
                const EVEN_MAKING_BITMASK: u32 = !1;
                let normalization_shift = n.leading_zeros() & EVEN_MAKING_BITMASK;
                n <<= normalization_shift;
                let s = $stages(n);
                unsafe { printf(c"s:%lx%lx\n".as_ptr(), s as u64, (s >> 64) as u64) };
                let denormalization_shift = normalization_shift >> 1;

                let res = s >> denormalization_shift;
                unsafe { printf(c"res:%lx%lx\n".as_ptr(), res as u64, (res >> 64) as u64) };
                res
            }
        }
    };
    ($UnsignedT:ident, $HalfBitsT:ident, $stages:ident) => {
        #[doc = concat!("[`", stringify!($UnsignedT), "`](prim@", stringify!($UnsignedT), ")")]
        #[must_use = "this returns the result of the operation, \
                      without modifying the original"]
        #[inline]
        pub fn $UnsignedT(mut n: $UnsignedT) -> $UnsignedT {
            if n <= <$HalfBitsT>::MAX as $UnsignedT {
                $HalfBitsT((n as $HalfBitsT).into()) as $UnsignedT
            } else {
                const EVEN_MAKING_BITMASK: u32 = !1;
                let normalization_shift = n.leading_zeros() & EVEN_MAKING_BITMASK;
                n <<= normalization_shift;
                let s = $stages(n);
                let denormalization_shift = normalization_shift >> 1;
                s >> denormalization_shift
            }
        }
    };
}
macro_rules! first_stage {
    (128, $n:ident) => {{
        test!($n != 0); //, "`$n` is  zero in `first_stage!`."
        const N_SHIFT: u32 = 128 - 8;
        let n = $n >> N_SHIFT;
        unsafe { printf(c"n:%lx\n".as_ptr(), n as u64) };
        let (s, r): (u8, u8) = U8_ISQRT_WITH_REMAINDER[n as usize];
        unsafe { printf(c"s:%x r:%x\n".as_ptr(), s as u64, r as u64) };
        unsafe { test!(s != 0) };
        (s, r)
    }};
    ($original_bits:literal, $n:ident) => {{
        test!($n != 0); //, "`$n` is  zero in `first_stage!`."
        const N_SHIFT: u32 = $original_bits - 8;
        let n = $n >> N_SHIFT;
        let (s, r) = U8_ISQRT_WITH_REMAINDER[n as usize];
        unsafe { test!(s != 0) };
        (s, r)
    }};
}
macro_rules! middle_stage {
    ($original_bits:literal, $ty:ty, $n:ident, $s:ident, $r:ident) => {{
        debug_assert!($s != 0, "`$s` is  zero in `middle_stage!`.");
        const N_SHIFT: u32 = $original_bits - <$ty>::BITS;
        let n = ($n >> N_SHIFT) as $ty;
        const HALF_BITS: u32 = <$ty>::BITS >> 1;
        const QUARTER_BITS: u32 = <$ty>::BITS >> 2;
        const LOWER_HALF_1_BITS: $ty = (1 << HALF_BITS) - 1;
        const LOWEST_QUARTER_1_BITS: $ty = (1 << QUARTER_BITS) - 1;
        let lo = n & LOWER_HALF_1_BITS;
        let numerator = (($r as $ty) << QUARTER_BITS) | (lo >> QUARTER_BITS);
        let denominator = ($s as $ty) << 1;
        let q = numerator / denominator;
        let u = numerator % denominator;
        let mut s = ($s << QUARTER_BITS) as $ty + q;
        let (mut r, overflow) =
            ((u << QUARTER_BITS) | (lo & LOWEST_QUARTER_1_BITS)).overflowing_sub(q * q);
        if overflow {
            r = r.wrapping_add(2 * s - 1);
            s -= 1;
        }
        unsafe { test!(s != 0) };
        (s, r)
    }};
}
macro_rules! last_stage {
    ($ty:ty, $n:ident, $s:ident, $r:ident) => {{
        debug_assert!($s != 0, "`$s` is  zero in `last_stage!`.");
        const HALF_BITS: u32 = <$ty>::BITS >> 1;
        const QUARTER_BITS: u32 = <$ty>::BITS >> 2;
        const LOWER_HALF_1_BITS: $ty = (1 << HALF_BITS) - 1;
        let lo = $n & LOWER_HALF_1_BITS;
        let numerator = (($r as $ty) << QUARTER_BITS) | (lo >> QUARTER_BITS);
        let denominator = ($s as $ty) << 1;
        let q = numerator / denominator;
        let mut s = ($s << QUARTER_BITS) as $ty + q;
        let (s_squared, overflow) = s.overflowing_mul(s);
        if overflow || s_squared > $n {
            s -= 1;
        }
        s
    }};
}
#[no_mangle]
fn test() {
    test_isqrt();
    let mut n: u128 = 0;
    for sqrt_n in 0..1_024.min((1_u128 << (<u128>::MAX.count_ones() / 2)) - 1) as u128 {
        isqrt_consistency_check(n);
        test_eq!(n.isqrt(), sqrt_n);
        n += sqrt_n;
        isqrt_consistency_check(n);
        test_eq!(n.isqrt(), sqrt_n);
        n += sqrt_n;
        isqrt_consistency_check(n);
        test_eq!(n.isqrt(), sqrt_n);
        n += 1;
    }
    let maximum_sqrt = <u128>::MAX.isqrt();
    let mut n = maximum_sqrt * maximum_sqrt;
    for sqrt_n in (maximum_sqrt - 1_024.min((1_u128 << (<u128>::MAX.count_ones() / 2)) - 1) as u128
        ..maximum_sqrt)
        .rev()
    {
        //unsafe { printf(c"%lx%lx\n".as_ptr(), sqrt_n as u64, sqrt_n >> 64 as u64) };
        isqrt_consistency_check(n);
        test_eq!(n.isqrt(), sqrt_n + 1);
        n -= 1;
        isqrt_consistency_check(n);
        test_eq!(n.isqrt(), sqrt_n);
        n -= sqrt_n;
        isqrt_consistency_check(n);
        test_eq!(n.isqrt(), sqrt_n);
        n -= sqrt_n;
    }
}
fn isqrt_consistency_check(n: u128) {
    if n > 0 {
        test_eq!(
            n.isqrt(),
            core::num::NonZero::<u128>::new(n)
                .expect("Was not able to create a new `NonZero` value from a nonzero number.")
                .isqrt()
                .get()
        );
    }
}
#[no_mangle]
fn max_ones() {
    test_eq!(u128::MAX.count_ones(), u128::BITS);
}
fn main() {
    test_rposition();
    const_opt();
    option_const();
    u128max_ovf_check();
    max_ones();
    test();
    const A: u128 = 0b0101_1111_u128;
    test_eq!(A.trailing_ones(), 5);
    unsafe { printf(c"%u\n".as_ptr(), (!A).leading_ones() as u32) };
    test_eq!((!A).leading_ones(), u128::BITS - 7);
    test_eq!(A.reverse_bits().leading_ones(), 5);
    test_eq!((!0_u128).leading_ones(), u128::BITS);
    test_eq!((!0_u128).trailing_ones(), u128::BITS);
    test_eq!(((!0_u128) << 1).trailing_ones(), 0);
    test_eq!(((!0_u128) >> 1).leading_ones(), 0);
    test_eq!(((!0_u128) << 1).leading_ones(), u128::BITS - 1);
    test_eq!(((!0_u128) >> 1).trailing_ones(), u128::BITS - 1);
    test_eq!(0_u128.leading_ones(), 0);
    test_eq!(0_u128.trailing_ones(), 0);
    const X: u128 = 0b0010_1100_u128;
    test_eq!(X.leading_ones(), 0);
    test_eq!(X.trailing_ones(), 0);
}
mod sqrts {
    use super::printf;
    #[inline]
    fn u16_stages(n: u16) -> u16 {
        let (s, r) = first_stage!(16, n);
        last_stage!(u16, n, s, r)
    }
    #[inline]
    fn u32_stages(n: u32) -> u32 {
        let (s, r) = first_stage!(32, n);
        let (s, r) = middle_stage!(32, u16, n, s, r);
        last_stage!(u32, n, s, r)
    }
    #[inline]
    fn u64_stages(n: u64) -> u64 {
        let (s, r) = first_stage!(64, n);
        let (s, r) = middle_stage!(64, u16, n, s, r);
        let (s, r) = middle_stage!(64, u32, n, s, r);
        last_stage!(u64, n, s, r)
    }
    #[inline]
    fn u128_stages(n: u128) -> u128 {
        let (s, r) = first_stage!(128, n);
        unsafe {
            printf(
                c"first_stage n:%lx%lx r:%lx s:%lx\n".as_ptr(),
                n as u64,
                (n >> 64) as u64,
                s as u64,
                r as u64,
            );
        }
        let (s, r) = middle_stage!(128, u16, n, s, r);
        unsafe {
            printf(
                c"middle_stage n:%lx%lx r:%lx s:%lx\n".as_ptr(),
                n as u64,
                (n >> 64) as u64,
                s as u64,
                r as u64,
            );
        }
        let (s, r) = middle_stage!(128, u32, n, s, r);
        unsafe {
            printf(
                c"middle_stage n:%lx%lx r:%lx s:%lx\n".as_ptr(),
                n as u64,
                (n >> 64) as u64,
                s as u64,
                r as u64,
            );
        }
        let (s, r) = middle_stage!(128, u64, n, s, r);
        let res = {
            debug_assert!(s != 0, "`s` is  zero in `last_stage!`.");
            const HALF_BITS: u32 = <u128>::BITS >> 1;
            const QUARTER_BITS: u32 = <u128>::BITS >> 2;
            const LOWER_HALF_1_BITS: u128 = (1 << HALF_BITS) - 1;
            let lo = n & LOWER_HALF_1_BITS;
            let numerator = ((r as u128) << QUARTER_BITS) | (lo >> QUARTER_BITS);
            let denominator = (s as u128) << 1;
            let q = numerator / denominator;
            let mut s = (s << QUARTER_BITS) as u128 + q;
            unsafe {
                printf(c"s:%lx%lx\n".as_ptr(), s as u64, (s >> 64) as u64);
            }
            let (s_squared, overflow) = s.overflowing_mul(s);
            unsafe {
                printf(
                    c"s_squared:%lx%lx overflow:%d\n".as_ptr(),
                    s as u64,
                    (s >> 64) as u64,
                    overflow as u32,
                );
            }
            if overflow || s_squared > n {
                s -= 1;
            }
            s
        };
        unsafe {
            printf(
                c"last_stage n:%lx%lx r:%lx s:%lx\n".as_ptr(),
                res as u64,
                (res >> 64) as u64,
                s as u64,
                r as u64,
            );
        }
        res
    }
    unsigned_fn!(u16, u8, u16_stages);
    unsigned_fn!(u32, u16, u32_stages);
    unsigned_fn!(u64, u32, u64_stages);
    unsigned_fn!(u128, u64, u128_stages);
    const U8_ISQRT_WITH_REMAINDER: [(u8, u8); 256] = {
        let mut result = [(0, 0); 256];
        let mut n: usize = 0;
        let mut isqrt_n: usize = 0;
        while n < result.len() {
            result[n] = (isqrt_n as u8, (n - isqrt_n.pow(2)) as u8);
            n += 1;
            if n == (isqrt_n + 1).pow(2) {
                isqrt_n += 1;
            }
        }
        result
    };
    #[inline]
    pub(super) const fn u8(n: u8) -> u8 {
        U8_ISQRT_WITH_REMAINDER[n as usize].0
    }
}

fn test_rposition() {
    fn f(xy: &(isize, char)) -> bool {
        let (_x, y) = *xy;
        y == 'b'
    }
    fn g(xy: &(isize, char)) -> bool {
        let (_x, y) = *xy;
        y == 'd'
    }
    let v = [(0, 'a'), (1, 'b'), (2, 'c'), (3, 'b')];

    assert_eq!(v.iter().rposition(f), Some(3));
    assert!(v.iter().rposition(g).is_none());
}
