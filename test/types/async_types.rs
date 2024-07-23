#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    start,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
#![no_std]
include!("../common.rs");

fn main() {
    /*
    let animal = black_box(Animal::Cow(8));
    match animal{
        Animal::Cow(val)=>{

            Put::putnl(val);
            test_eq!(val,8);
        }
        Animal::Dog(val)=>{
            Put::putnl(val);
            test_eq!(0,1);
        }
    }
    //test_indirect(&animal);*/
    let x = black_box(async_fn(8, black_box(9.9)));
}
async fn async_fn(a: i32, b: f32) -> f32 {
    a as f32 + black_box(b)
}
