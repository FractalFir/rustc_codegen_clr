#![feature(
    lang_items,
    adt_const_params,
    associated_type_defaults,
    core_intrinsics,
    let_chains,
    never_type,
    unsized_const_params
)]
#![allow(internal_features, incomplete_features, unused_variables, dead_code)]
include!("../common.rs");
/// SAFETY: The caller MUST guarantee that `v_base` is valid for 4 reads and

/// `dst` is valid for 4 writes. The result will be stored in `dst[0..4]`.

pub unsafe fn sort4_stable<T, F: FnMut(&T, &T) -> bool>(

    v_base: *const T,

    dst: *mut T,

    is_less: &mut F,

) {

    // By limiting select to picking pointers, we are guaranteed good cmov code-gen

    // regardless of type T's size. Further this only does 5 instead of 6

    // comparisons compared to a stable transposition 4 element sorting-network,

    // and always copies each element exactly once.


    // SAFETY: all pointers have offset at most 3 from v_base and dst, and are

    // thus in-bounds by the precondition.

    unsafe {

        // Stably create two pairs a <= b and c <= d.

        let c1 = is_less(&*v_base.add(1), &*v_base);

        let c2 = is_less(&*v_base.add(3), &*v_base.add(2));

        let a = v_base.add(c1 as usize);

        let b = v_base.add(!c1 as usize);

        let c = v_base.add(2 + c2 as usize);

        let d = v_base.add(2 + (!c2 as usize));


        // Compare (a, c) and (b, d) to identify max/min. We're left with two

        // unknown elements, but because we are a stable sort we must know which

        // one is leftmost and which one is rightmost.

        // c3, c4 | min max unknown_left unknown_right

        //  0,  0 |  a   d    b         c

        //  0,  1 |  a   b    c         d

        //  1,  0 |  c   d    a         b

        //  1,  1 |  c   b    a         d

        let c3 = is_less(&*c, &*a);

        let c4 = is_less(&*d, &*b);

        let min = select(c3, c, a);

        let max = select(c4, b, d);

        let unknown_left = select(c3, a, select(c4, c, b));

        let unknown_right = select(c4, d, select(c3, b, c));


        // Sort the last two unknown elements.

        let c5 = is_less(&*unknown_right, &*unknown_left);

        let lo = select(c5, unknown_right, unknown_left);

        let hi = select(c5, unknown_left, unknown_right);


        core::ptr::copy_nonoverlapping(min, dst, 1);

        core::ptr::copy_nonoverlapping(lo, dst.add(1), 1);

        core::ptr::copy_nonoverlapping(hi, dst.add(2), 1);

        core::ptr::copy_nonoverlapping(max, dst.add(3), 1);

    }


    #[inline(always)]

    fn select<T>(cond: bool, if_true: *const T, if_false: *const T) -> *const T {

        if cond { if_true } else { if_false }

    }

}
fn main() {
    let src = [4.0_f64,3.0,2.0,1.0];
    let mut dst = [0.0_f64; 4];
    unsafe{sort4_stable(&src[0] as *const f64, &mut dst[0] as *mut f64, &mut |a,b| a < b)};
    unsafe{printf(c"%f\n".as_ptr(),dst[0])};
    test_eq!(dst[0],1.0);
    test_eq!(dst[1],2.0);
    test_eq!(dst[2],3.0);
    test_eq!(dst[3],4.0);
}