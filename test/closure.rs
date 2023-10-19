use std::hint::black_box;
pub fn simplest_closure(){
    black_box(||{black_box(0)});
}