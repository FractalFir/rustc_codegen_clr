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
use std::ops::Sub;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::num::FpCategory;
use std::fmt::LowerExp;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Div;
use std::mem::MaybeUninit;
#[no_mangle]
#[inline(never)]
fn ub(){
    let res = FullDecoded::Finite(Decoded {
        mant: 1 << 1,
        minus: 1,
        plus: 1,
        exp: 1 - 1,
        inclusive: true,
    });
    let discr = std::mem::discriminant(&res);
    unsafe{printf(c"res:%lu\n".as_ptr(), std::mem::transmute::<_,u64>(discr))};
    let tagish = match res{
        FullDecoded::Finite(_)=>1,
        FullDecoded::Nan=>2,
        FullDecoded::Infinite=>3,
        FullDecoded::Zero=>4,
    };
    test_eq!(tagish,1);
}
fn main(){
    ub();
    let num = 3.14159;
    let precision = 8_usize;
  let mut buf: [MaybeUninit<u8>; 1024] = [MaybeUninit::uninit(); 1024]; // enough for f32 and f64
    let mut parts: [MaybeUninit<Part<'_>>; 4] = [MaybeUninit::uninit(); 4];
    let formatted = to_exact_fixed_str(
        format_exact,
        num,
        Sign::MinusPlus,
        precision.into(),
        &mut buf,
        &mut parts,
    );
    
    test_eq!(formatted.sign, "+");
    test_eq!(formatted.parts.len(), 3);
    test_eq!(formatted.parts, [Part::Copy(&[63]),Part::Copy(&[46]),Part::Copy(&[63, 63, 63, 63, 63, 63, 63, 64])]);
}
pub fn to_exact_fixed_str<'a, T, F>(
    mut format_exact: F,
    v: T,
    sign: Sign,
    frac_digits: usize,
    buf: &'a mut [MaybeUninit<u8>],
    parts: &'a mut [MaybeUninit<Part<'a>>],
) -> Formatted<'a>
where
    T: DecodableFloat,
    F: FnMut(&Decoded, &'a mut [MaybeUninit<u8>], i16) -> (&'a [u8], i16),
{
    assert!(parts.len() >= 4);

    let (negative, full_decoded) = decode(v);
    unsafe{printf(c"%p Sign: %d\n".as_ptr(), &full_decoded as *const _, negative as u8 as u32)};
    // UB: decoded value with corrupt tag.
    full_decoded.print();
    let sign = determine_sign(sign, &full_decoded, negative);
    unsafe{printf(c"Decoded, preparing to match.\n".as_ptr())};
    match full_decoded {
        FullDecoded::Nan => {
            parts[0] = MaybeUninit::new(Part::Copy(b"NaN"));
            // SAFETY: we just initialized the elements `..1`.
            Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
        }
        FullDecoded::Infinite => {
            parts[0] = MaybeUninit::new(Part::Copy(b"inf"));
            // SAFETY: we just initialized the elements `..1`.
            Formatted { sign, parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) } }
        }
        FullDecoded::Zero => {
            if frac_digits > 0 {
                // [0.][0000]
                parts[0] = MaybeUninit::new(Part::Copy(b"0."));
                parts[1] = MaybeUninit::new(Part::Zero(frac_digits));
                Formatted {
                    sign,
                    // SAFETY: we just initialized the elements `..2`.
                    parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..2]) },
                }
            } else {
                parts[0] = MaybeUninit::new(Part::Copy(b"0"));
                Formatted {
                    sign,
                    // SAFETY: we just initialized the elements `..1`.
                    parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) },
                }
            }
        }
        FullDecoded::Finite(ref decoded) => {
            unsafe{printf(c"finite.\n".as_ptr())};
            let maxlen = estimate_max_buf_len(decoded.exp);
            assert!(buf.len() >= maxlen);

            // it *is* possible that `frac_digits` is ridiculously large.
            // `format_exact` will end rendering digits much earlier in this case,
            // because we are strictly limited by `maxlen`.
            let limit = if frac_digits < 0x8000 { -(frac_digits as i16) } else { i16::MIN };
            let (buf, exp) = format_exact(decoded, &mut buf[..maxlen], limit);
            if exp <= limit {
                // the restriction couldn't been met, so this should render like zero no matter
                // `exp` was. this does not include the case that the restriction has been met
                // only after the final rounding-up; it's a regular case with `exp = limit + 1`.
                debug_assert_eq!(buf.len(), 0);
                if frac_digits > 0 {
                    // [0.][0000]
                    parts[0] = MaybeUninit::new(Part::Copy(b"0."));
                    parts[1] = MaybeUninit::new(Part::Zero(frac_digits));
                    Formatted {
                        sign,
                        // SAFETY: we just initialized the elements `..2`.
                        parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..2]) },
                    }
                } else {
                    parts[0] = MaybeUninit::new(Part::Copy(b"0"));
                    Formatted {
                        sign,
                        // SAFETY: we just initialized the elements `..1`.
                        parts: unsafe { MaybeUninit::slice_assume_init_ref(&parts[..1]) },
                    }
                }
            } else {
                Formatted { sign, parts: digits_to_dec_str(buf, exp, frac_digits, parts) }
            }
        }
    }
}
#[derive(Clone,Copy)]
pub struct Decoded {
    pub mant: u64,
    pub minus: u64,
    pub plus: u64,
    pub exp: i16,
    pub inclusive: bool,
}
impl Decoded {
    fn print(&self){

    }
}

pub trait DecodableFloat: RawFloat + Copy {
    // Required method
    fn min_pos_norm_value() -> Self;
}
impl DecodableFloat for f64 {
    fn min_pos_norm_value() -> Self {
        f64::MIN_POSITIVE
    }
}
pub trait RawFloat: Sized + Div<Output = Self> + Neg<Output = Self> + Mul<Output = Self> + Add<Output = Self> + LowerExp + PartialEq + PartialOrd + Default + Clone + Copy + Debug {
    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;
    const NEG_NAN: Self;
    const MANTISSA_EXPLICIT_BITS: usize;
    const MIN_EXPONENT_ROUND_TO_EVEN: i32;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32;
    const MIN_EXPONENT_FAST_PATH: i64;
    const MAX_EXPONENT_FAST_PATH: i64;
    const MAX_EXPONENT_DISGUISED_FAST_PATH: i64;
    const MINIMUM_EXPONENT: i32;
    const INFINITE_POWER: i32;
    const SIGN_INDEX: usize;
    const SMALLEST_POWER_OF_TEN: i32;
    const LARGEST_POWER_OF_TEN: i32;
    const MAX_MANTISSA_FAST_PATH: u64 =  2_u64 << Self::MANTISSA_EXPLICIT_BITS;

    // Required methods
    fn from_u64(v: u64) -> Self;
    fn from_u64_bits(v: u64) -> Self;
    fn pow10_fast_path(exponent: usize) -> Self;
    fn classify(self) -> FpCategory;
    fn integer_decode(self) -> (u64, i16, i8);
}
impl RawFloat for f64 {
    const INFINITY: Self = f64::INFINITY;
    const NEG_INFINITY: Self = f64::NEG_INFINITY;
    const NAN: Self = f64::NAN;
    const NEG_NAN: Self = -f64::NAN;

    const MANTISSA_EXPLICIT_BITS: usize = 52;
    const MIN_EXPONENT_ROUND_TO_EVEN: i32 = -4;
    const MAX_EXPONENT_ROUND_TO_EVEN: i32 = 23;
    const MIN_EXPONENT_FAST_PATH: i64 = -22; // assuming FLT_EVAL_METHOD = 0
    const MAX_EXPONENT_FAST_PATH: i64 = 22;
    const MAX_EXPONENT_DISGUISED_FAST_PATH: i64 = 37;
    const MINIMUM_EXPONENT: i32 = -1023;
    const INFINITE_POWER: i32 = 0x7FF;
    const SIGN_INDEX: usize = 63;
    const SMALLEST_POWER_OF_TEN: i32 = -342;
    const LARGEST_POWER_OF_TEN: i32 = 308;

    #[inline]
    fn from_u64(v: u64) -> Self {
        debug_assert!(v <= Self::MAX_MANTISSA_FAST_PATH);
        v as _
    }

    #[inline]
    fn from_u64_bits(v: u64) -> Self {
        f64::from_bits(v)
    }

    fn pow10_fast_path(exponent: usize) -> Self {
        const TABLE: [f64; 32] = [
            1e0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8, 1e9, 1e10, 1e11, 1e12, 1e13, 1e14, 1e15,
            1e16, 1e17, 1e18, 1e19, 1e20, 1e21, 1e22, 0., 0., 0., 0., 0., 0., 0., 0., 0.,
        ];
        TABLE[exponent & 31]
    }

    /// Returns the mantissa, exponent and sign as integers.
    fn integer_decode(self) -> (u64, i16, i8) {
        let bits = self.to_bits();
        let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
        let mut exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
        let mantissa = if exponent == 0 {
            (bits & 0xfffffffffffff) << 1
        } else {
            (bits & 0xfffffffffffff) | 0x10000000000000
        };
        // Exponent bias + mantissa shift
        exponent -= 1023 + 52;
        (mantissa, exponent, sign)
    }

    fn classify(self) -> FpCategory {
        self.classify()
    }
}
#[derive(Clone,Copy)]
pub enum FullDecoded {
    Nan,
    Infinite,
    Zero,
    Finite(Decoded),
}
impl FullDecoded{
    #[no_mangle]
    fn print(&self){
        let discr = std::mem::discriminant(self);
        unsafe{printf(c"%x %lu\n".as_ptr(), self as *const _,std::mem::transmute::<_,u64>(discr))};
        /*let tagish = match self{
            FullDecoded::Finite(_)=>1,
            FullDecoded::Nan=>2,
            FullDecoded::Infinite=>3,
            FullDecoded::Zero=>4,
        };
    
        test_eq!(tagish,1);*/
        /*match self{
            FullDecoded::Nan=>unsafe{printf(c"FullDecoded::Nan\n".as_ptr());},
            FullDecoded::Infinite=>unsafe{printf(c"FullDecoded::Infinite\n".as_ptr());},
            FullDecoded::Zero=>unsafe{printf(c"FullDecoded::Zero\n".as_ptr());},
            FullDecoded::Finite(decoded)=>{
                unsafe{printf(c"FullDecoded::Finite{ decoded:".as_ptr())};
                decoded.print();
                unsafe{printf(c"}\n".as_ptr())};
            },
        }*/
    }
}
#[derive(Clone,Copy)]
pub struct Formatted<'a> {
    pub sign: &'static str,
    pub parts: &'a [Part<'a>],
}
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub enum Part<'a> {
    Zero(usize),
    Num(u16),
    Copy(&'a [u8]),
}
pub enum Sign {
    Minus,
    MinusPlus,
}
/// Returns a sign (true when negative) and `FullDecoded` value
/// from given floating point number.
pub fn decode<T: DecodableFloat>(v: T) -> (/*negative?*/ bool, FullDecoded) {
    let (mant, exp, sign) = v.integer_decode();
    let even = (mant & 1) == 0;
    let decoded = match v.classify() {
        FpCategory::Nan => FullDecoded::Nan,
        FpCategory::Infinite => FullDecoded::Infinite,
        FpCategory::Zero => FullDecoded::Zero,
        FpCategory::Subnormal => {
            unsafe{printf(c" FpCategory::Subnormal\n".as_ptr())};
            // neighbors: (mant - 2, exp) -- (mant, exp) -- (mant + 2, exp)
            // Float::integer_decode always preserves the exponent,
            // so the mantissa is scaled for subnormals.
            FullDecoded::Finite(Decoded { mant, minus: 1, plus: 1, exp, inclusive: even })
        }
        FpCategory::Normal => {
         
            let minnorm = <T as DecodableFloat>::min_pos_norm_value().integer_decode();
            if mant == minnorm.0 {
                // neighbors: (maxmant, exp - 1) -- (minnormmant, exp) -- (minnormmant + 1, exp)
                // where maxmant = minnormmant * 2 - 1
                unsafe{printf(c" FpCategory::Normal, minnorm == mant\n".as_ptr())};
                FullDecoded::Finite(Decoded {
                    mant: mant << 2,
                    minus: 1,
                    plus: 2,
                    exp: exp - 2,
                    inclusive: even,
                })
            } else {
                // neighbors: (mant - 1, exp) -- (mant, exp) -- (mant + 1, exp)
                unsafe{printf(c" FpCategory::Normal, minnorm != mant\n".as_ptr())};
                let res = FullDecoded::Finite(Decoded {
                    mant: mant << 1,
                    minus: 1,
                    plus: 1,
                    exp: exp - 1,
                    inclusive: even,
                });
                let tagish = match res{
                    FullDecoded::Finite(_)=>1,
                    FullDecoded::Nan=>2,
                    FullDecoded::Infinite=>3,
                    FullDecoded::Zero=>4,
                };
                test_eq!(tagish,1);
                res.print();
                res
            }
        }
    };
    (sign < 0, decoded)
}
fn determine_sign(sign: Sign, decoded: &FullDecoded, negative: bool) -> &'static str {
    match (*decoded, sign) {
        (FullDecoded::Nan, _) => "",
        (_, Sign::Minus) => {
            if negative {
                "-"
            } else {
                ""
            }
        }
        (_, Sign::MinusPlus) => {
        
            if negative {
                "-"
            } else {
                "+"
            }
        }
    }
}
fn digits_to_dec_str<'a>(
    buf: &'a [u8],
    exp: i16,
    frac_digits: usize,
    parts: &'a mut [MaybeUninit<Part<'a>>],
) -> &'a [Part<'a>] {
    assert!(!buf.is_empty());
    assert!(buf[0] > b'0');
    assert!(parts.len() >= 4);

    // if there is the restriction on the last digit position, `buf` is assumed to be
    // left-padded with the virtual zeroes. the number of virtual zeroes, `nzeroes`,
    // equals to `max(0, exp + frac_digits - buf.len())`, so that the position of
    // the last digit `exp - buf.len() - nzeroes` is no more than `-frac_digits`:
    //
    //                       |<-virtual->|
    //       |<---- buf ---->|  zeroes   |     exp
    //    0. 1 2 3 4 5 6 7 8 9 _ _ _ _ _ _ x 10
    //    |                  |           |
    // 10^exp    10^(exp-buf.len())   10^(exp-buf.len()-nzeroes)
    //
    // `nzeroes` is individually calculated for each case in order to avoid overflow.

    if exp <= 0 {
        // the decimal point is before rendered digits: [0.][000...000][1234][____]
        let minus_exp = -(exp as i32) as usize;
        parts[0] = MaybeUninit::new(Part::Copy(b"0."));
        parts[1] = MaybeUninit::new(Part::Zero(minus_exp));
        parts[2] = MaybeUninit::new(Part::Copy(buf));
        if frac_digits > buf.len() && frac_digits - buf.len() > minus_exp {
            parts[3] = MaybeUninit::new(Part::Zero((frac_digits - buf.len()) - minus_exp));
            // SAFETY: we just initialized the elements `..4`.
            unsafe { MaybeUninit::slice_assume_init_ref(&parts[..4]) }
        } else {
            // SAFETY: we just initialized the elements `..3`.
            unsafe { MaybeUninit::slice_assume_init_ref(&parts[..3]) }
        }
    } else {
        let exp = exp as usize;
        if exp < buf.len() {
            // the decimal point is inside rendered digits: [12][.][34][____]
            parts[0] = MaybeUninit::new(Part::Copy(&buf[..exp]));
            parts[1] = MaybeUninit::new(Part::Copy(b"."));
            parts[2] = MaybeUninit::new(Part::Copy(&buf[exp..]));
            if frac_digits > buf.len() - exp {
                parts[3] = MaybeUninit::new(Part::Zero(frac_digits - (buf.len() - exp)));
                // SAFETY: we just initialized the elements `..4`.
                unsafe { MaybeUninit::slice_assume_init_ref(&parts[..4]) }
            } else {
                // SAFETY: we just initialized the elements `..3`.
                unsafe { MaybeUninit::slice_assume_init_ref(&parts[..3]) }
            }
        } else {
            // the decimal point is after rendered digits: [1234][____0000] or [1234][__][.][__].
            parts[0] = MaybeUninit::new(Part::Copy(buf));
            parts[1] = MaybeUninit::new(Part::Zero(exp - buf.len()));
            if frac_digits > 0 {
                parts[2] = MaybeUninit::new(Part::Copy(b"."));
                parts[3] = MaybeUninit::new(Part::Zero(frac_digits));
                // SAFETY: we just initialized the elements `..4`.
                unsafe { MaybeUninit::slice_assume_init_ref(&parts[..4]) }
            } else {
                // SAFETY: we just initialized the elements `..2`.
                unsafe { MaybeUninit::slice_assume_init_ref(&parts[..2]) }
            }
        }
    }
}
fn estimate_max_buf_len(exp: i16) -> usize {
    21 + ((if exp < 0 { -12 } else { 5 } * exp as i32) as usize >> 4)
}

/// The exact and fixed mode implementation for Dragon.
pub fn format_exact<'a>(
    d: &Decoded,
    buf: &'a mut [MaybeUninit<u8>],
    limit: i16,
) -> (/*digits*/ &'a [u8], /*exp*/ i16) {
    assert!(d.mant > 0);
    assert!(d.minus > 0);
    assert!(d.plus > 0);
    assert!(d.mant.checked_add(d.plus).is_some());
    assert!(d.mant.checked_sub(d.minus).is_some());

    // estimate `k_0` from original inputs satisfying `10^(k_0-1) < v <= 10^(k_0+1)`.
    let mut k = estimate_scaling_factor(d.mant, d.exp);

    // `v = mant / scale`.
    let mut mant:u128 = (d.mant as u128); // Big
    let mut scale:u128 = (1); // Big
    if d.exp < 0 {
        //scale.mul_pow2(-d.exp as usize);
    } else {
        //mant.mul_pow2(d.exp as usize);
    }

    // divide `mant` by `10^k`. now `scale / 10 < mant <= scale * 10`.
    if k >= 0 {
        //mul_pow10(&mut scale, k as usize);
    } else {
        //mul_pow10(&mut mant, -k as usize);
    }

    // fixup when `mant + plus >= scale`, where `plus / scale = 10^-buf.len() / 2`.
    // in order to keep the fixed-size bignum, we actually use `mant + floor(plus) >= scale`.
    // we are not actually modifying `scale`, since we can skip the initial multiplication instead.
    // again with the shortest algorithm, `d[0]` can be zero but will be eventually rounded up.
    if scale / 2 * 10_u128.pow(buf.len() as u32) + mant >= scale {
        // equivalent to scaling `scale` by 10
        k += 1;
    } else {
        mant *= 10;
    }

    // if we are working with the last-digit limitation, we need to shorten the buffer
    // before the actual rendering in order to avoid double rounding.
    // note that we have to enlarge the buffer again when rounding up happens!
    let mut len = if k < limit {
        // oops, we cannot even produce *one* digit.
        // this is possible when, say, we've got something like 9.5 and it's being rounded to 10.
        // we return an empty buffer, with an exception of the later rounding-up case
        // which occurs when `k == limit` and has to produce exactly one digit.
        0
    } else if ((k as i32 - limit as i32) as usize) < buf.len() {
        (k - limit) as usize
    } else {
        buf.len()
    };

    if len > 0 {
        // cache `(2, 4, 8) * scale` for digit generation.
        // (this can be expensive, so do not calculate them when the buffer is empty.)
        let mut scale2 = scale.clone();
        scale2 = scale2 * 2;
        let mut scale4 = scale.clone();
        scale4 = scale4 * 4;
        let mut scale8 = scale.clone();
        scale8 = scale8 * 8;

        for i in 0..len {
            if mant == 0 {
                // following digits are all zeroes, we stop here
                // do *not* try to perform rounding! rather, fill remaining digits.
                for c in &mut buf[i..len] {
                    *c = MaybeUninit::new(b'0');
                }
                // SAFETY: we initialized that memory above.
                return (unsafe { MaybeUninit::slice_assume_init_ref(&buf[..len]) }, k);
            }

            let mut d = 0;
            if mant >= scale8 {
                mant.sub(&scale8);
                d += 8;
            }
            if mant >= scale4 {
                mant.sub(&scale4);
                d += 4;
            }
            if mant >= scale2 {
                mant.sub(&scale2);
                d += 2;
            }
            if mant >= scale {
                mant.sub(&scale);
                d += 1;
            }
            debug_assert!(mant < scale);
            debug_assert!(d < 10);
            buf[i] = MaybeUninit::new(b'0' + d);
            mant *= 10;
        }
    }

    // rounding up if we stop in the middle of digits
    // if the following digits are exactly 5000..., check the prior digit and try to
    // round to even (i.e., avoid rounding up when the prior digit is even).
    let order = mant.cmp(&(scale * 5));
    if order == Ordering::Greater
        || (order == Ordering::Equal
            // SAFETY: `buf[len-1]` is initialized.
            && len > 0 && unsafe { buf[len - 1].assume_init() } & 1 == 1)
    {
        // if rounding up changes the length, the exponent should also change.
        // but we've been requested a fixed number of digits, so do not alter the buffer...
        // SAFETY: we initialized that memory above.
        if let Some(c) = round_up(unsafe { MaybeUninit::slice_assume_init_mut(&mut buf[..len]) }) {
            // ...unless we've been requested the fixed precision instead.
            // we also need to check that, if the original buffer was empty,
            // the additional digit can only be added when `k == limit` (edge case).
            k += 1;
            if k > limit && len < buf.len() {
                buf[len] = MaybeUninit::new(c);
                len += 1;
            }
        }
    }

    // SAFETY: we initialized that memory above.
    (unsafe { MaybeUninit::slice_assume_init_ref(&buf[..len]) }, k)
}
pub fn estimate_scaling_factor(mant: u64, exp: i16) -> i16 {
    // 2^(nbits-1) < mant <= 2^nbits if mant > 0
    let nbits = 64 - (mant - 1).leading_zeros() as i64;
    // 1292913986 = floor(2^32 * log_10 2)
    // therefore this always underestimates (or is exact), but not much.
    (((nbits + exp as i64) * 1292913986) >> 32) as i16
}
pub fn round_up(d: &mut [u8]) -> Option<u8> {
    match d.iter().rposition(|&c| c != b'9') {
        Some(i) => {
            // d[i+1..n] is all nines
            d[i] += 1;
            for j in i + 1..d.len() {
                d[j] = b'0';
            }
            None
        }
        None if d.len() > 0 => {
            // 999..999 rounds to 1000..000 with an increased exponent
            d[0] = b'1';
            for j in 1..d.len() {
                d[j] = b'0';
            }
            Some(b'0')
        }
        None => {
            // an empty buffer rounds up (a bit strange but reasonable)
            Some(b'1')
        }
    }
}

