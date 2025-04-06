#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params,
    pointer_is_aligned_to
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
fn main() {
    use std::fmt::Debug;

    fn test<T: Copy + Debug + PartialEq>(x: T) {
        let v: &[T] = &[x, x, x];
        let v_ptrs: [*const T; 3] = match v {
            [v1, v2, v3] => [v1 as *const _, v2 as *const _, v3 as *const _],
            _ => unreachable!(),
        };
        let len = v.len();

        // nth(i)
        for i in 0..len {
            test_eq!(&v[i] as *const _, v_ptrs[i]); // check the v_ptrs array, just to be sure
            let nth = v.iter().nth(i).unwrap();
            test_eq!(nth as *const _, v_ptrs[i]);
        }
        test_eq!(v.iter().nth(len), None);

        // stepping through with nth(0)
        {
            let mut it = v.iter();
            for i in 0..len {
                let next = it.nth(0).unwrap();
                test_eq!(next as *const _, v_ptrs[i]);
            }
            test_eq!(it.nth(0), None);
        }

        // next()
        {
            let mut it = v.iter();
            for i in 0..len {
                let remaining = len - i;
                test_eq!(it.size_hint(), (remaining, Some(remaining)));

                let next = it.next().unwrap();
                test_eq!(next as *const _, v_ptrs[i]);
            }
            test_eq!(it.size_hint(), (0, Some(0)));
            test_eq!(it.next(), None);
        }

        // next_back()
        {
            let mut it = v.iter();
            for i in 0..len {
                let remaining = len - i;
                test_eq!(it.size_hint(), (remaining, Some(remaining)));

                let prev = it.next_back().unwrap();
                test_eq!(prev as *const _, v_ptrs[remaining - 1]);
            }
            test_eq!(it.size_hint(), (0, Some(0)));
            test_eq!(it.next_back(), None);
        }
    }

    fn test_mut<T: Copy + Debug + PartialEq>(x: T) {
        let v: &mut [T] = &mut [x, x, x];
        let v_ptrs: [*mut T; 3] = match v {
            &mut [ref v1, ref v2, ref v3] => {
                [v1 as *const _ as *mut _, v2 as *const _ as *mut _, v3 as *const _ as *mut _]
            }
            _ => unreachable!(),
        };
        let len = v.len();

        // nth(i)
        for i in 0..len {
            test_eq!(&mut v[i] as *mut _, v_ptrs[i]); // check the v_ptrs array, just to be sure
            let nth = v.iter_mut().nth(i).unwrap();
            test_eq!(nth as *mut _, v_ptrs[i]);
        }
        test_eq!(v.iter().nth(len), None);

        // stepping through with nth(0)
        {
            let mut it = v.iter();
            for i in 0..len {
                let next = it.nth(0).unwrap();
                test_eq!(next as *const _, v_ptrs[i] as *const _);
            }
            test_eq!(it.nth(0), None);
        }

        // next()
        {
            let mut it = v.iter_mut();
            for i in 0..len {
                let remaining = len - i;
                test_eq!(it.size_hint(), (remaining, Some(remaining)));

                let next = it.next().unwrap();
                test_eq!(next as *mut _, v_ptrs[i]);
            }
            test_eq!(it.size_hint(), (0, Some(0)));
            test_eq!(it.next(), None);
        }

        // next_back()
        {
            let mut it = v.iter_mut();
            for i in 0..len {
                let remaining = len - i;
                test_eq!(it.size_hint(), (remaining, Some(remaining)));

                let prev = it.next_back().unwrap();
                test_eq!(prev as *mut _, v_ptrs[remaining - 1]);
            }
            test_eq!(it.size_hint(), (0, Some(0)));
            test_eq!(it.next_back(), None);
        }
    }
    zst_issue();
    // Make sure iterators and slice patterns yield consistent addresses for various types,
    // including ZSTs.
    test(());
     /*test(0u32);
    
   test([0u32; 0]); // ZST with alignment > 0
    test_mut(0u32);
    test_mut(());
    test_mut([0u32; 0]); // ZST with alignment > 0*/
}
#[inline(never)]
#[no_mangle]
fn zst_issue(){
    let v: &[()] = &[()];
    test_eq!(v.iter().nth(1), None);
}
