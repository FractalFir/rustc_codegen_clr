#![feature(float_next_up_down)]
#[test]
fn should_pass() {}
#[test]
#[should_panic]
fn should_panic() {
    panic!();
}
#[allow(unused_macros)]
macro_rules! assert_f64_biteq {
    ($left : expr, $right : expr) => {
        let l: &f64 = &$left;
        let r: &f64 = &$right;
        let lb = l.to_bits();
        let rb = r.to_bits();
        assert_eq!(
            lb, rb,
            "float {l} ({lb:#018x}) is not bitequal to {r} ({rb:#018x})"
        );
    };
}
fn main() {
    let nan0 = f64::NAN;
    let nan1 = f64::from_bits(f64::NAN.to_bits() ^ NAN_MASK1);
    let nan2 = f64::from_bits(f64::NAN.to_bits() ^ NAN_MASK2);
    assert_f64_biteq!(next_up(nan0), nan0);
    assert_f64_biteq!(next_up(nan1), nan1);
    assert_f64_biteq!(next_up(nan2), nan2);
}

#[test]
fn test_next_up() {
    let tiny = f64::from_bits(TINY_BITS);
    let tiny_up = f64::from_bits(TINY_UP_BITS);
    let max_down = f64::from_bits(MAX_DOWN_BITS);
    let largest_subnormal = f64::from_bits(LARGEST_SUBNORMAL_BITS);
    let smallest_normal = f64::from_bits(SMALLEST_NORMAL_BITS);
    assert_f64_biteq!(f64::NEG_INFINITY.next_up(), f64::MIN);
    assert_f64_biteq!(f64::MIN.next_up(), -max_down);
    assert_f64_biteq!((-1.0 - f64::EPSILON).next_up(), -1.0);
    assert_f64_biteq!((-smallest_normal).next_up(), -largest_subnormal);
    assert_f64_biteq!((-tiny_up).next_up(), -tiny);
    assert_f64_biteq!((-tiny).next_up(), -0.0f64);
    assert_f64_biteq!((-0.0f64).next_up(), tiny);
    assert_f64_biteq!(0.0f64.next_up(), tiny);
    assert_f64_biteq!(tiny.next_up(), tiny_up);
    assert_f64_biteq!(largest_subnormal.next_up(), smallest_normal);
    assert_f64_biteq!(1.0f64.next_up(), 1.0 + f64::EPSILON);
    assert_f64_biteq!(f64::MAX.next_up(), f64::INFINITY);
    assert_f64_biteq!(f64::INFINITY.next_up(), f64::INFINITY);

    let nan0 = f64::NAN;
    let nan1 = f64::from_bits(f64::NAN.to_bits() ^ NAN_MASK1);
    let nan2 = f64::from_bits(f64::NAN.to_bits() ^ NAN_MASK2);
    assert_f64_biteq!(next_up(nan0), nan0);
    assert_f64_biteq!(next_up(nan1), nan1);
    assert_f64_biteq!(next_up(nan2), nan2);
}
pub fn next_up(val: f64) -> f64 {
    // Some targets violate Rust's assumption of IEEE semantics, e.g. by flushing
    // denormals to zero. This is in general unsound and unsupported, but here
    // we do our best to still produce the correct result on such targets.
    let bits = val.to_bits();
    eprintln!("bits:{bits:?}");
    if val.is_nan() || bits == f64::INFINITY.to_bits() {
        return val;
    }

    let abs = bits & !SIGN_MASK;
    let next_bits = if abs == 0 {
        TINY_BITS
    } else if bits == abs {
        bits + 1
    } else {
        bits - 1
    };
    eprintln!("next_bits:{next_bits:?}");
    f64::from_bits(next_bits)
}
/// Sign bit
const SIGN_MASK: u64 = 0x8000_0000_0000_0000;

/// Exponent mask
const EXP_MASK: u64 = 0x7ff0_0000_0000_0000;

/// Mantissa mask
const MAN_MASK: u64 = 0x000f_ffff_ffff_ffff;

/// Minimum representable positive value (min subnormal)

/// Minimum representable negative value (min negative subnormal)
const NEG_TINY_BITS: u64 = TINY_BITS | SIGN_MASK;
/// Smallest number
#[allow(dead_code)] // unused on x86
const TINY_BITS: u64 = 0x1;

/// Next smallest number
#[allow(dead_code)] // unused on x86
const TINY_UP_BITS: u64 = 0x2;

/// Exponent = 0b11...10, Sifnificand 0b1111..10. Min val > 0
#[allow(dead_code)] // unused on x86
const MAX_DOWN_BITS: u64 = 0x7fef_ffff_ffff_fffe;

/// Zeroed exponent, full significant
#[allow(dead_code)] // unused on x86
const LARGEST_SUBNORMAL_BITS: u64 = 0x000f_ffff_ffff_ffff;

/// Exponent = 0b1, zeroed significand
#[allow(dead_code)] // unused on x86
const SMALLEST_NORMAL_BITS: u64 = 0x0010_0000_0000_0000;

/// First pattern over the mantissa
#[allow(dead_code)] // unused on x86
const NAN_MASK1: u64 = 0x000a_aaaa_aaaa_aaaa;

/// Second pattern over the mantissa
#[allow(dead_code)] // unused on x86
const NAN_MASK2: u64 = 0x0005_5555_5555_5555;
