
#![feature(lang_items,adt_const_params,associated_type_defaults,core_intrinsics,start)]
#![allow(internal_features,incomplete_features,unused_variables,dead_code)]
#![no_std]
include!("../common.rs");
use core::cmp::Ordering;
fn alt_cmp(a:&usize,b:&usize)->Ordering{
    if *a < *b{
        Ordering::Less
    }
    else if *a == *b{
        Ordering::Equal
    }
    else{
        Ordering::Greater
    }
}
#[inline]
#[must_use]
fn max_by(v1:usize,v2:usize,compare:impl FnOnce(&usize,&usize)->Ordering)->usize{
    match compare(&v1, &v2) {
        Ordering::Less | Ordering::Equal => v2,
        Ordering::Greater => v1,
    }
}
fn test(a:usize)->usize{
    a
}
fn test_test(test:impl Fn(usize)->usize){
    test_eq!(66,test(black_box(66_usize)));
}
fn main(){
    test_test(test);
    let ord = <usize as Ord>::cmp(&black_box(67_usize),&black_box(171_usize));
    black_box(ord);
    /* 
    test_eq!(black_box(max_by(black_box(67_usize),black_box(171_usize),alt_cmp)),171_usize);
    test_eq!(black_box(max_by(black_box(171_usize),black_box(67_usize),alt_cmp)),171_usize);
    test_eq!(black_box(max_by(black_box(71_usize),black_box(71_usize),alt_cmp)),71_usize);
    */
    /*
    test_eq!(black_box(max_by(black_box(67_usize),black_box(171_usize),<usize as Ord>::cmp)),171_usize);
    test_eq!(black_box(max_by(black_box(171_usize),black_box(67_usize),<usize as Ord>::cmp)),171_usize);
    test_eq!(black_box(max_by(black_box(71_usize),black_box(71_usize),<usize as Ord>::cmp)),71_usize);    
    */

    /*
    test_eq!(black_box(core::cmp::max_by(black_box(67_usize),black_box(171_usize),<usize as Ord>::cmp)),171_usize);
    test_eq!(black_box(core::cmp::max_by(black_box(171_usize),black_box(67_usize),<usize as Ord>::cmp)),171_usize);
    test_eq!(black_box(core::cmp::max_by(black_box(71_usize),black_box(71_usize),<usize as Ord>::cmp)),71_usize);
    */
    /* 
    test_eq!(black_box(core::cmp::max(black_box(67_usize),black_box(171_usize))),171_usize);
    test_eq!(black_box(core::cmp::max(black_box(171_usize),black_box(67_usize))),171_usize);
    test_eq!(black_box(core::cmp::max(black_box(71_usize),black_box(71_usize))),71_usize);
    */
}

