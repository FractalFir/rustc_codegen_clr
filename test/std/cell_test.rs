#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params
)]
#![allow(
    internal_features,
    incomplete_features,
    unused_variables,
    dead_code,
    unused_imports,
    unused_mut,
    private_interfaces,
    non_upper_case_globals
)]
#[allow(dead_code)]
use core::cell::Cell;
include!("../common.rs");
fn cell_test() {
    let cell = Cell::new(black_box(64));
    test_eq!(cell.get(), 64);
    cell.set(black_box(33));
    test_eq!(cell.get(), 33);
}
fn main() {
    cell_test();
    refcell_ref_coercion();
}

fn refcell_ref_coercion() {
    use core::cell::Ref;
    use core::cell::RefCell;
    use core::cell::RefMut;

    let cell: RefCell<[i32; 3]> = RefCell::new([1, 2, 3]);
    {
        let mut cellref: RefMut<'_, [i32; 3]> = cell.borrow_mut();
        cellref[0] = 4;
        let mut coerced: RefMut<'_, [i32]> = cellref;
        coerced[2] = 5;
        drop(coerced);
    }
    {
        let comp: &mut [i32] = &mut [4, 2, 5];
        let cellref: Ref<'_, [i32; 3]> = cell.borrow();
        assert_eq!(&*cellref, comp);
        let coerced: Ref<'_, [i32]> = cellref;
        assert_eq!(&*coerced, comp);
    }
}
