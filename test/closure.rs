#![allow(unused_variables,unused_must_use)]
use std::hint::black_box;
pub fn simplest_closure(){
    black_box(||{black_box(0)});
}
pub fn return_simple_closure()->impl Fn()->(){
    black_box(||{})
}
pub fn call_simple_closure(){
    let simple = black_box(||{});
    simple();
}