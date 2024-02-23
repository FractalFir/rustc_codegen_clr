#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    invalid_value
)]
#![no_std]
include!("../common.rs");


fn main(){
    let _5 = black_box(1111759727_i32) as isize;
    test_eq!(_5,1111759727_isize);
    let _19 = black_box(_5)*black_box(_5);
    test_eq!(_19,1236009690579114529_isize);
    let _5 = black_box(_19);
    test_eq!(_5,1236009690579114529_isize);
    let _11 = black_box(138366210602866892655579814762857650160_u128) as f64;
    test_eq!(_11,1.383662106028669e38);
    let _23 = black_box(_11) as isize;
    test_eq!(_23, black_box(9223372036854775807_isize));
    let _5 = black_box(_23) - black_box(_19);
    test_eq!(_5,7987362346275661278_isize);
 }