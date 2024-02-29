use crate::*;
use core::num::NonZeroUsize;
use core::sync::atomic::{AtomicUsize, Ordering};
use core::{array, assert_eq};

pub fn array_from_ref() {
    let value: i32 = 8.into();
    let arr: &[i32; 1] = array::from_ref(&value);
    assert_eq!(&[value.clone()], arr);

    const VALUE: &&str = &"Hello World!";
    const ARR: &[&str; 1] = array::from_ref(VALUE);
    assert_eq!(&[*VALUE], ARR);
    assert!(core::ptr::eq(VALUE, &ARR[0]));
}
