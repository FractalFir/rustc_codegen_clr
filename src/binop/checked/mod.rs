use crate::{assembly::MethodCompileCtx, casts};
use cilly::{
    and, call,
    cil_node::CILNode,
    cil_root::CILRoot,
    conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u64, conv_usize, eq, gt, gt_un,
    ldc_i32, ldc_i64, ldc_u32, ldc_u64, lt, mul, or,
    v2::{cilnode::MethodKind, Assembly, ClassRef, FieldDesc, Int, MethodRef},
    Type,
};
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};

pub fn result_tuple(tpe: Type, out_of_range: CILNode, val: CILNode, asm: &mut Assembly) -> CILNode {
    let tuple = crate::r#type::simple_tuple(&[tpe, Type::Bool], asm);
    let item2 = asm.alloc_string("Item2");
    let item1 = asm.alloc_string("Item1");
    CILNode::TemporaryLocal(Box::new((
        tuple.into(),
        [
            CILRoot::SetField {
                addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                value: Box::new(out_of_range),
                desc: asm.alloc_field(FieldDesc::new(tuple, item2, Type::Bool)),
            },
            CILRoot::SetField {
                addr: Box::new(CILNode::LoadAddresOfTMPLocal),
                value: Box::new(val),
                desc: asm.alloc_field(FieldDesc::new(tuple, item1, tpe)),
            },
        ]
        .into(),
        CILNode::LoadTMPLocal,
    )))
    //CILNode::T
}
pub fn zero(ty: Ty, asm: &mut Assembly) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(0),
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(0),
        TyKind::Uint(UintTy::U32) => ldc_u32!(0),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(0),
        TyKind::Int(IntTy::I32) => ldc_i32!(0),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(0),
        TyKind::Uint(UintTy::U64) => ldc_u64!(0),
        TyKind::Int(IntTy::I64) => ldc_i64!(0),
        TyKind::Uint(UintTy::Usize) => conv_usize!(ldc_u32!(0)),
        TyKind::Int(IntTy::Isize) => conv_isize!(ldc_u32!(0)),
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([Type::Int(Int::U32)], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [ldc_u32!(0)])
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(asm),
                asm.alloc_string("op_Implicit"),
                asm.sig([Type::Int(Int::I32)], Type::Int(Int::I128)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [ldc_i32!(0)])
        }
        _ => todo!("Can't get zero of {ty:?}"),
    }
}
fn min(ty: Ty, asm: &mut Assembly) -> CILNode {
    match ty.kind() {
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(u8::MIN),
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(u16::MIN),
        TyKind::Uint(UintTy::U32) => ldc_u32!(u32::MIN),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(i8::MIN),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(i16::MIN),
        TyKind::Int(IntTy::I32) => ldc_i32!(i32::MIN),
        TyKind::Uint(UintTy::U64) => ldc_u64!(u64::MIN),
        TyKind::Int(IntTy::I64) => ldc_i64!(i64::MIN),
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
        TyKind::Uint(UintTy::U8) => CILNode::LdcU8(u8::MAX),
        TyKind::Uint(UintTy::U16) => CILNode::LdcU16(u16::MAX),
        TyKind::Uint(UintTy::U32) => ldc_u32!(u32::MAX),
        TyKind::Int(IntTy::I8) => CILNode::LdcI8(i8::MAX),
        TyKind::Int(IntTy::I16) => CILNode::LdcI16(i16::MAX),
        TyKind::Int(IntTy::I32) => ldc_i32!(i32::MAX),
        TyKind::Uint(UintTy::U64) => ldc_u64!(u64::MAX),
        TyKind::Int(IntTy::I64) => ldc_i64!(i64::MAX),
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
    let mul = super::mul_unchecked(ty, ty, ctx, ops_a.clone(), ops_b.clone());
    let ovf = match ty.kind() {
        // Work without promotions
        TyKind::Uint(UintTy::U8 | UintTy::U16) => gt_un!(mul.clone(), max(ty, ctx)),
        TyKind::Int(IntTy::I8 | IntTy::I16) => {
            or!(
                gt!(mul.clone(), max(ty, ctx)),
                lt!(mul.clone(), min(ty, ctx))
            )
        }
        // Works with 32 -> 64 size promotions
        TyKind::Uint(UintTy::U32) => {
            let mul = mul!(conv_u64!(ops_a.clone()), conv_u64!(ops_b.clone()));
            gt_un!(mul.clone(), conv_u64!(max(ty, ctx)))
        }
        TyKind::Int(IntTy::I32) => {
            let mul = mul!(conv_i64!(ops_a.clone()), conv_i64!(ops_b.clone()));
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
            let op_mul = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_Multiply"),
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
            let op_gt = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_GreaterThan"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            let gt = call!(
                ctx.alloc_methodref(op_gt),
                [
                    mul.clone(),
                    casts::int_to_int(Type::Int(Int::I64), Type::Int(Int::I128), max(ty, ctx), ctx)
                ]
            );
            let op_lt = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_LessThan"),
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
            let op_mul = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_Multiply"),
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
            let op_gt = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_GreaterThan"),
                ctx.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                MethodKind::Static,
                vec![].into(),
            );
            let gt = call!(
                ctx.alloc_methodref(op_gt),
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
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_LessThan"),
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
                CILNode::LdFalse
            )
        }

        _ => {
            eprintln!("WARINING: can't checked mul type {ty:?}");
            CILNode::LdFalse
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
                    lt!(sum.clone(), conv_i16!(ldc_i32!(i8::MIN.into()))),
                    gt!(sum.clone(), conv_i16!(ldc_i32!(i8::MAX.into())))
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
                    lt!(sum.clone(), (ldc_i32!(i16::MIN.into()))),
                    gt!(sum.clone(), (ldc_i32!(i16::MAX.into())))
                ),
                conv_i16!(sum),
                ctx,
            );
        }
        TyKind::Int(IntTy::I32) => {
            let sum = conv_i64!(ops_a.clone()) + conv_i64!(ops_b.clone());
            let out_of_range = or!(
                lt!(sum.clone(), conv_i64!(ldc_i32!(i32::MIN))),
                gt!(sum.clone(), conv_i64!(ldc_i32!(i32::MAX)))
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
