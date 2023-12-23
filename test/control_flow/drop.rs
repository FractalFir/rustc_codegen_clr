#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code,unused_imports)]
#![no_std]
include!("../common.rs");
struct DecrementOnDrop<'a>(&'a mut u32);
impl Drop for DecrementOnDrop<'_>{
    fn drop(&mut self){
        //Put::putnl(*self.0);
        *self.0 -= 1;
    }
}
fn main(){
    let mut i = 1;
    //Put::putnl(i);
    //test_eq!(black_box(i),1);
    let d = DecrementOnDrop(&mut i);
    black_box(d);
    //Put::putnl(i);
    //test_eq!(black_box(i),0);
}
