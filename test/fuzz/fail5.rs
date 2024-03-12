pub enum Adt44 {
    Variant1 { fld0: [i64; ARR_SIZE], fld1: u8 },
    Variant3 { fld6: Adt27 },
}
pub enum Adt27 {
    Variant0 { fld0: (u8, char) },
    Variant2 { fld0: u8 },
}
// OK if less than 2(array size less than sizeof(int128))
const ARR_SIZE: usize = 3;
fn fn9(mut _1: Adt44, mut _5: [i64; ARR_SIZE]) {
    core::hint::black_box(())
}
pub fn main() {
  
    return fn9(
        Adt44::Variant1 {
            fld0: <[i64; ARR_SIZE]>::default(),
            fld1: 8,
        },
        <[i64; ARR_SIZE]>::default(),
    );
}
