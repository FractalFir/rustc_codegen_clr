#![allow(unused_variables,unused_must_use)]
use std::hint::black_box;
pub fn simplest_closure(){
    black_box(||{black_box(0)});
}
pub fn return_simple_closure()->impl Fn()->(){
    black_box(||{})
}
pub fn call_simple_closure(){
    let simple = black_box(||{black_box(0)});
    black_box(simple());
}
pub fn call_closure_with_1_arg(){
    let simple = black_box(|x|{black_box(x)});
    black_box(simple(4));
}
pub fn call_closure_with_4_args(){
    let simple = black_box(|x,y,z|{(black_box(x) + y) as f32 * z});
    black_box(simple(4,7,8.43));
}
pub fn closure_which_captures(){
    let a = black_box(8);
    let closure = black_box(||{black_box(a)});
}