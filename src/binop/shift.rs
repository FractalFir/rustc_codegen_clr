use crate::assembly::MethodCompileCtx;
use crate::utilis::compiletime_sizeof;

use cilly::{
    call, call_site::CallSite, cil_node::CILNode, conv_i32, conv_u32, fn_sig::FnSig, ldc_u32,
    rem_un, shl, shr, shr_un, DotnetTypeRef, Type,
};

use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};
pub fn shr_unchecked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_RightShift".into(),
                    FnSig::new(&[Type::U128, Type::I32], Type::U128),
                    true,
                ),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_RightShift".into(),
                    FnSig::new(&[Type::I128, Type::I32], Type::I128),
                    true,
                ),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                ]
            )
        }
        TyKind::Uint(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr_un!(
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                )
            }
            _ => shr_un!(ops_a, ops_b),
        },
        TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr!(
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                )
            }

            _ => shr!(ops_a, ops_b),
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}

pub fn shr_checked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    let bit_cap = u32::try_from(compiletime_sizeof(value_type, ctx.tcx()) * 8)
        .expect("Intiger size over 2^32 bits.");
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_RightShift".into(),
                    FnSig::new(&[Type::U128, Type::I32], Type::U128),
                    true,
                ),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        crate::casts::int_to_int(type_b.clone(), &Type::U32, ops_b),
                        ldc_u32!(128)
                    ))
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_RightShift".into(),
                    FnSig::new(&[Type::I128, Type::I32], Type::I128),
                    true,
                ),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        crate::casts::int_to_int(type_b.clone(), &Type::U32, ops_b),
                        ldc_u32!(128)
                    ))
                ]
            )
        }
        TyKind::Uint(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr_un!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)),
                        ldc_u32!(bit_cap)
                    )
                )
            }
            _ => {
                shr_un!(ops_a, rem_un!(conv_u32!(ops_b), ldc_u32!(bit_cap)))
            }
        },
        TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)),
                        ldc_u32!(bit_cap)
                    )
                )
            }
            _ => {
                shr!(ops_a, rem_un!(conv_u32!(ops_b), ldc_u32!(bit_cap)))
            }
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}

pub fn shl_checked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    let bit_cap = u32::try_from(compiletime_sizeof(value_type, ctx.tcx()) * 8)
        .expect("Intiger has over 2^32 bits.");
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_LeftShift".into(),
                    FnSig::new(&[Type::U128, Type::I32], Type::U128),
                    true,
                ),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        conv_u32!(crate::casts::int_to_int(type_b.clone(), &Type::U32, ops_b)),
                        ldc_u32!(bit_cap)
                    ))
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_LeftShift".into(),
                    FnSig::new(&[Type::I128, Type::I32], Type::I128),
                    true,
                ),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        conv_u32!(crate::casts::int_to_int(type_b.clone(), &Type::U32, ops_b)),
                        ldc_u32!(bit_cap)
                    ))
                ]
            )
        }
        TyKind::Uint(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shl!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)),
                        ldc_u32!(bit_cap)
                    )
                )
            }
            _ => {
                shl!(ops_a, rem_un!(conv_u32!(ops_b), ldc_u32!(bit_cap)))
            }
        },
        TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shl!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)),
                        ldc_u32!(bit_cap)
                    )
                )
            }

            _ => {
                shl!(ops_a, rem_un!(conv_u32!(ops_b), ldc_u32!(bit_cap)))
            }
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}

pub fn shl_unchecked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::uint_128().into(),
                    "op_LeftShift".into(),
                    FnSig::new(&[Type::U128, Type::I32], Type::U128),
                    true,
                ),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            call!(
                CallSite::boxed(
                    DotnetTypeRef::int_128().into(),
                    "op_LeftShift".into(),
                    FnSig::new(&[Type::I128, Type::I32], Type::I128),
                    true,
                ),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                ]
            )
        }
        TyKind::Uint(_) | TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shl!(
                    ops_a,
                    crate::casts::int_to_int(type_b.clone(), &Type::I32, ops_b)
                )
            }
            _ => shl!(ops_a, ops_b),
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}
