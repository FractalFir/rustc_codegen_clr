use crate::{assembly::MethodCompileCtx, casts};
use rustc_codegen_clr_type::GetTypeExt;
use cilly::{
    and, call,
    cil_node::CILNode,
    conv_i16, conv_i32, conv_i64, conv_i8, conv_u32, conv_u64, conv_u8, eq, gt, gt_un, lt, or,
    v2::{cilnode::MethodKind, Assembly, ClassRef, Int, MethodRef},
    Type,
};
use rustc_codegen_clr_type::utilis::simple_tuple;
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};

pub fn result_tuple(tpe: Type, out_of_range: CILNode, val: CILNode, asm: &mut Assembly) -> CILNode {
    let tuple = simple_tuple(&[tpe, Type::Bool], asm);
    CILNode::ovf_check_tuple(asm, tuple, out_of_range, val, tpe)
}
pub fn zero(ty: Ty, asm: &mut Assembly) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::V2(asm.alloc_node(0_u8)),
        TyKind::Uint(UintTy::U16) => CILNode::V2(asm.alloc_node(0_u16)),
        TyKind::Uint(UintTy::U32) => CILNode::V2(asm.alloc_node(0_u32)),
        TyKind::Uint(UintTy::U64) => CILNode::V2(asm.alloc_node(0_u64)),
        TyKind::Uint(UintTy::Usize) => CILNode::V2(asm.alloc_node(0_usize)),
        TyKind::Int(IntTy::I8) => CILNode::V2(asm.alloc_node(0_i8)),
        TyKind::Int(IntTy::I16) => CILNode::V2(asm.alloc_node(0_i16)),
        TyKind::Int(IntTy::I32) => CILNode::V2(asm.alloc_node(0_i32)),
        TyKind::Int(IntTy::I64) => CILNode::V2(asm.alloc_node(0_i64)),
        TyKind::Int(IntTy::Isize) => CILNode::V2(asm.alloc_node(0_isize)),
        TyKind::Uint(UintTy::U128) => CILNode::V2(asm.alloc_node(0_u128)),
        TyKind::Int(IntTy::I128) => CILNode::V2(asm.alloc_node(0_i128)),
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn min(ty: Ty, asm: &mut Assembly) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::V2(asm.alloc_node(u8::MIN)),
        TyKind::Uint(UintTy::U16) => CILNode::V2(asm.alloc_node(u16::MIN)),
        TyKind::Uint(UintTy::U32) => CILNode::V2(asm.alloc_node(u32::MIN)),
        TyKind::Uint(UintTy::U64) => CILNode::V2(asm.alloc_node(u64::MIN)),
        TyKind::Int(IntTy::I8) => CILNode::V2(asm.alloc_node(i8::MIN)),
        TyKind::Int(IntTy::I16) => CILNode::V2(asm.alloc_node(i16::MIN)),
        TyKind::Int(IntTy::I32) => CILNode::V2(asm.alloc_node(i32::MIN)),
        TyKind::Int(IntTy::I64) => CILNode::V2(asm.alloc_node(i64::MIN)),
        TyKind::Uint(UintTy::Usize) => {
            let mref = MethodRef::new(
                ClassRef::usize_type(asm),
                asm.alloc_string("get_MinValue"),
                asm.sig([], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), []);
            cilnode
        }
        TyKind::Int(IntTy::Isize) => {
            let mref = MethodRef::new(
                ClassRef::isize_type(asm),
                asm.alloc_string("get_MinValue"),
                asm.sig([], Type::Int(Int::ISize)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), []);
            cilnode
        }
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("get_MinValue"),
                asm.sig([], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("get_MinValue"),
                asm.sig([], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        _ => todo!("Can't get min of {ty:?}"),
    }
}
fn max(ty: Ty, asm: &mut Assembly) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::V2(asm.alloc_node(u8::MAX)),
        TyKind::Uint(UintTy::U16) => CILNode::V2(asm.alloc_node(u16::MAX)),
        TyKind::Uint(UintTy::U32) => CILNode::V2(asm.alloc_node(u32::MAX)),
        TyKind::Uint(UintTy::U64) => CILNode::V2(asm.alloc_node(u64::MAX)),
        TyKind::Int(IntTy::I8) => CILNode::V2(asm.alloc_node(i8::MAX)),
        TyKind::Int(IntTy::I16) => CILNode::V2(asm.alloc_node(i16::MAX)),
        TyKind::Int(IntTy::I32) => CILNode::V2(asm.alloc_node(i32::MAX)),
        TyKind::Int(IntTy::I64) => CILNode::V2(asm.alloc_node(i64::MAX)),
        TyKind::Uint(UintTy::Usize) => {
            let mref = MethodRef::new(
                ClassRef::usize_type(asm),
                asm.alloc_string("get_MaxValue"),
                asm.sig([], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        TyKind::Int(IntTy::Isize) => {
            let mref = MethodRef::new(
                ClassRef::isize_type(asm),
                asm.alloc_string("get_MaxValue"),
                asm.sig([], Type::Int(Int::ISize)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("get_MaxValue"),
                asm.sig([], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("get_MaxValue"),
                asm.sig([], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [])
        }
        _ => todo!("Can't get max of {ty:?}"),
    }
}

pub fn mul<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    //(b > 0 && a < INT_MIN + b) || (b < 0 && a > INT_MAX + b);
    let tpe = ctx.type_from_cache(ty);
    let mul = super::mul_unchecked(ty, ctx, ops_a.clone(), ops_b.clone());
    let ovf = match ty.kind() {
        // Work without promotions
        TyKind::Uint(UintTy::U8) => {
            let mul = CILNode::Mul(
                conv_u8!(ops_a.clone()).into(),
                conv_u8!(ops_b.clone()).into(),
            );
            gt_un!(mul.clone(), conv_u8!(max(ty, ctx)))
        }
        TyKind::Uint(UintTy::U16) => {
            let mul = CILNode::Mul(
                conv_u32!(ops_a.clone()).into(),
                conv_u32!(ops_b.clone()).into(),
            );
            gt_un!(mul.clone(), conv_u32!(max(ty, ctx)))
        }
        TyKind::Int(IntTy::I8) => {
            let mul = CILNode::Mul(
                conv_i16!(ops_a.clone()).into(),
                conv_i16!(ops_b.clone()).into(),
            );
            or!(
                gt!(mul.clone(), conv_i16!(max(ty, ctx))),
                lt!(mul.clone(), conv_i16!(min(ty, ctx)))
            )
        }
        TyKind::Int(IntTy::I16) => {
            let mul = CILNode::Mul(
                conv_i32!(ops_a.clone()).into(),
                conv_i32!(ops_b.clone()).into(),
            );
            or!(
                gt!(mul.clone(), conv_i32!(max(ty, ctx))),
                lt!(mul.clone(), conv_i32!(min(ty, ctx)))
            )
        }
        // Works with 32 -> 64 size promotions
        TyKind::Uint(UintTy::U32) => {
            let mul = CILNode::Mul(
                conv_u64!(ops_a.clone()).into(),
                conv_u64!(ops_b.clone()).into(),
            );
            gt_un!(mul.clone(), conv_u64!(max(ty, ctx)))
        }
        TyKind::Int(IntTy::I32) => {
            let mul = CILNode::Mul(
                conv_i64!(ops_a.clone()).into(),
                conv_i64!(ops_b.clone()).into(),
            );
            or!(
                gt!(mul.clone(), conv_i64!(max(ty, ctx))),
                lt!(mul.clone(), conv_i64!(min(ty, ctx)))
            )
        }
        // Use 128 bit ints, not supported in mono.
        TyKind::Uint(UintTy::U64) => {
            let op_mul = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_Multiply"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::U128)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let mul = call!(
                ctx.alloc_methodref(op_mul),
                [
                    casts::int_to_int(
                        Type::Int(Int::U64),
                        Type::Int(Int::U128),
                        ops_a.clone(),
                        ctx
                    ),
                    casts::int_to_int(
                        Type::Int(Int::U64),
                        Type::Int(Int::U128),
                        ops_b.clone(),
                        ctx
                    )
                ]
            );
            let op_gt = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_GreaterThan"),
                ctx.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(op_gt),
                [
                    mul.clone(),
                    casts::int_to_int(Type::Int(Int::U64), Type::Int(Int::U128), max(ty, ctx), ctx)
                ]
            )
        }
        TyKind::Int(IntTy::I64) => {
            let main_module = *ctx.main_module();
            let op_mul = MethodRef::new(
                main_module,
                ctx.alloc_string("mul_i128"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let mul = call!(
                ctx.alloc_methodref(op_mul),
                [
                    casts::int_to_int(
                        Type::Int(Int::I64),
                        Type::Int(Int::I128),
                        ops_a.clone(),
                        ctx
                    ),
                    casts::int_to_int(
                        Type::Int(Int::I64),
                        Type::Int(Int::I128),
                        ops_b.clone(),
                        ctx
                    )
                ]
            );
            let op_greater_than = MethodRef::new(
                main_module,
                ctx.alloc_string("gt_i128"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            let gt = call!(
                ctx.alloc_methodref(op_greater_than),
                [
                    mul.clone(),
                    casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), max(ty, ctx), ctx)
                ]
            );
            let op_lt = MethodRef::new(
                main_module,
                ctx.alloc_string("lt_i128"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            let lt = call!(
                ctx.alloc_methodref(op_lt),
                [
                    mul.clone(),
                    casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), min(ty, ctx), ctx)
                ]
            );
            or!(gt, lt)
        }

        TyKind::Uint(UintTy::Usize) => {
            let main_module = *ctx.main_module();
            let op_mul = MethodRef::new(
                main_module,
                ctx.alloc_string("mul_u128"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::U128)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let mul = call!(
                ctx.alloc_methodref(op_mul),
                [
                    casts::int_to_int(
                        Type::Int(Int::USize),
                        Type::Int(Int::U128),
                        ops_a.clone(),
                        ctx
                    ),
                    casts::int_to_int(
                        Type::Int(Int::USize),
                        Type::Int(Int::U128),
                        ops_b.clone(),
                        ctx
                    )
                ]
            );
            let op_gt = MethodRef::new(
                main_module,
                ctx.alloc_string("gt_u128"),
                ctx.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(op_gt),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::USize),
                        Type::Int(Int::U128),
                        max(ty, ctx),
                        ctx
                    )
                ]
            )
        }
        TyKind::Int(IntTy::Isize) => {
            let main_module = *ctx.main_module();
            let op_mul = MethodRef::new(
                main_module,
                ctx.alloc_string("mul_i128"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I128)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let mul = call!(
                ctx.alloc_methodref(op_mul),
                [
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        ops_a.clone(),
                        ctx
                    ),
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        ops_b.clone(),
                        ctx
                    )
                ]
            );
            let op_greater_than = MethodRef::new(
                main_module,
                ctx.alloc_string("gt_i128"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            let gt = call!(
                ctx.alloc_methodref(op_greater_than),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        max(ty, ctx),
                        ctx
                    )
                ]
            );
            let op_lt = MethodRef::new(
                main_module,
                ctx.alloc_string("lt_i128"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            let lt = call!(
                ctx.alloc_methodref(op_lt),
                [
                    mul.clone(),
                    casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::I128),
                        min(ty, ctx),
                        ctx
                    )
                ]
            );
            or!(gt, lt)
        }
        TyKind::Int(IntTy::I128) => {
            let op_mul = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("i128_mul_ovf_check"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            eq!(
                call!(ctx.alloc_methodref(op_mul), [ops_a.clone(), ops_b.clone()]),
                CILNode::V2(ctx.alloc_node(false))
            )
        }

        _ => {
            eprintln!("WARINING: can't checked mul type {ty:?}");
            CILNode::V2(ctx.alloc_node(false))
        }
    };
    result_tuple(tpe, ovf, mul, ctx)
}
pub fn sub_signed<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    let min = min(ty, ctx);
    let max = max(ty, ctx);
    result_tuple(
        tpe,
        or!(
            and!(
                super::cmp::gt_unchecked(ty, ops_b.clone(), zero(ty, ctx), ctx),
                super::cmp::lt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(ty, ty, ctx, min, ops_b.clone()),
                    ctx
                )
            ),
            and!(
                super::cmp::lt_unchecked(ty, ops_b.clone(), zero(ty, ctx), ctx),
                super::cmp::gt_unchecked(
                    ty,
                    ops_a.clone(),
                    super::add_unchecked(ty, ty, ctx, max, ops_b.clone()),
                    ctx
                )
            )
        ),
        super::sub_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
        ctx,
    )
}
pub fn sub_unsigned<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    result_tuple(
        tpe,
        super::cmp::lt_unchecked(ty, ops_a.clone(), ops_b.clone(), ctx),
        super::sub_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
        ctx,
    )
}
pub fn add_unsigned<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    let res = super::add_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());

    result_tuple(
        tpe,
        super::cmp::lt_unchecked(
            ty,
            res.clone(),
            super::bit_or_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone()),
            ctx,
        ),
        res,
        ctx,
    )
}
pub fn add_signed<'tcx>(
    ops_a: &CILNode,
    ops_b: &CILNode,
    ty: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILNode {
    let tpe = ctx.type_from_cache(ty);
    match ty.kind() {
        TyKind::Int(IntTy::I8) => {
            let sum = conv_i16!(ops_a.clone()) + conv_i16!(ops_b.clone());
            return result_tuple(
                tpe,
                or!(
                    lt!(sum.clone(), CILNode::V2(ctx.alloc_node(i16::from(i8::MIN)))),
                    gt!(sum.clone(), CILNode::V2(ctx.alloc_node(i16::from(i8::MAX))))
                ),
                conv_i8!(sum),
                ctx,
            );
        }
        TyKind::Int(IntTy::I16) => {
            let sum = conv_i32!(ops_a.clone()) + conv_i32!(ops_b.clone());
            return result_tuple(
                tpe,
                or!(
                    lt!(
                        sum.clone(),
                        (CILNode::V2(ctx.alloc_node(i32::from(i16::MIN))))
                    ),
                    gt!(
                        sum.clone(),
                        (CILNode::V2(ctx.alloc_node(i32::from(i16::MAX))))
                    )
                ),
                conv_i16!(sum),
                ctx,
            );
        }
        TyKind::Int(IntTy::I32) => {
            let sum = conv_i64!(ops_a.clone()) + conv_i64!(ops_b.clone());
            let out_of_range = or!(
                lt!(
                    sum.clone(),
                    conv_i64!(CILNode::V2(ctx.alloc_node(i32::MIN)))
                ),
                gt!(
                    sum.clone(),
                    conv_i64!(CILNode::V2(ctx.alloc_node(i32::MAX)))
                )
            );
            return result_tuple(tpe, out_of_range, conv_i32!(sum), ctx);
        }
        _ => (),
    }
    let res = super::add_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());
    result_tuple(
        tpe,
        or!(
            and!(
                super::lt_unchecked(ty, ops_a.clone(), zero(ty, ctx), ctx),
                and!(
                    super::lt_unchecked(ty, ops_b.clone(), zero(ty, ctx), ctx),
                    super::gt_unchecked(ty, res.clone(), zero(ty, ctx), ctx)
                )
            ),
            and!(
                super::gt_unchecked(ty, ops_a.clone(), zero(ty, ctx), ctx),
                and!(
                    super::gt_unchecked(ty, ops_b.clone(), zero(ty, ctx), ctx),
                    super::lt_unchecked(ty, res.clone(), zero(ty, ctx), ctx)
                )
            )
        ),
        res,
        ctx,
    )
}
