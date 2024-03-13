#![feature(core_intrinsics)]
#![allow(internal_features, dead_code)]
pub fn main() {
    let adt = Adt29 {
        fld2: 0x42,
        fld6: Adt18::Variant1 { fld0: 0x66 },
    };
    let adt = core::hint::black_box(adt);
    if adt.fld2 != 0x42 {
        core::intrinsics::abort();
    }
}

pub struct Adt29 {
    fld2: u8,
    fld6: Adt18,
}

pub enum Adt18 {
    Variant0 { fld0: [u64; 3], fld4: u128 },
    Variant1 { fld0: u8 },
}
