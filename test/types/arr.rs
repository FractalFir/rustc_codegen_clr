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
struct IndirectArray {
    arr: [i32; 5],
}
fn main() {
    // Const array

    let arr = [0, 1, 2, 3, 4];
    black_box(arr);
    // Const indexing
    test_eq!(arr[3], 3);
    test_eq!(arr[4], 4);
    // Non-const indexing
    test_eq!(arr[black_box(3)], 3);
    test_eq!(arr[black_box(4)], 4);

    // Aggreagte array

    let arr = [0, 1, 2, 3, black_box(4)];
    black_box(arr);
    // Const indexing
    test_eq!(arr[3], 3);
    test_eq!(arr[4], 4);
    // Non-const indexing
    test_eq!(arr[black_box(3)], 3);
    test_eq!(arr[black_box(4)], 4);

    // Mutable const array

    let mut arr = [0, 1, 2, 3, 4];
    black_box(arr);
    // Const indexing
    test_eq!(arr[3], 3);
    test_eq!(arr[4], 4);
    // Const indexed mutating
    arr[4] = 99;
    test_eq!(arr[4], 99);
    // Non-const indexing
    test_eq!(arr[black_box(3)], 3);
    test_eq!(arr[black_box(4)], 99);
    // Non-const indexed mutating
    arr[black_box(4)] = 4;
    test_eq!(arr[4], 4);

    // Mutable aggreagte array

    let mut arr = [0, 1, 2, 3, black_box(4)];
    black_box(arr);
    // Const indexing
    test_eq!(arr[3], 3);
    test_eq!(arr[4], 4);
    // Const indexed mutating
    arr[4] = 99;
    test_eq!(arr[4], 99);
    // Non-const indexing
    test_eq!(arr[black_box(3)], 3);
    test_eq!(arr[black_box(4)], 99);
    // Non-const indexed mutating
    arr[black_box(4)] = 4;
    test_eq!(arr[4], 4);

    // Indirect mutable array

    let mut indarr = IndirectArray {
        arr: [0, 1, 2, 3, black_box(4)],
    };
    black_box(indarr.arr);
    // Const indexing
    test_eq!(indarr.arr[3], 3);
    test_eq!(indarr.arr[4], 4);
    // Const indexed mutating
    indarr.arr[4] = 99;
    test_eq!(indarr.arr[4], 99);
    // Non-const indexing
    test_eq!(indarr.arr[black_box(3)], 3);
    test_eq!(indarr.arr[black_box(4)], 99);
    // Non-const indexed mutating
    indarr.arr[black_box(4)] = 4;
    test_eq!(indarr.arr[4], 4);
    let a = [123456_u128];
    black_box(a);
}
