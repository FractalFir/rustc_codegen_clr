use crate::{assembly::MethodCompileCtx, utilis::compiletime_sizeof};
use cilly::{
    call,
    cil_node::CILNode,
    conv_i32, conv_u32, rem_un, shl, shr, shr_un, Type,
    {cilnode::MethodKind, ClassRef, Int, MethodRef},
};

use rustc_codegen_clr_type::GetTypeExt;

use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};
pub fn shr_unchecked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_RightShift"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::I32)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_RightShift"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I32)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
                ]
            )
        }
        TyKind::Uint(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr_un!(
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
                )
            }
            _ => shr_un!(ops_a, ops_b),
        },
        TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr!(
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
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
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    let bit_cap = u32::try_from(compiletime_sizeof(value_type, ctx.tcx()) * 8)
        .expect("Intiger size over 2^32 bits.");
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_RightShift"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::I32)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        crate::casts::int_to_int(type_b, Type::Int(Int::U32), ops_b, ctx),
                        CILNode::V2(ctx.alloc_node(128_u32))
                    ))
                ]
            );
            cilnode
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_RightShift"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I32)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        crate::casts::int_to_int(type_b, Type::Int(Int::U32), ops_b, ctx),
                        CILNode::V2(ctx.alloc_node(128_u32))
                    ))
                ]
            );
            cilnode
        }
        TyKind::Uint(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr_un!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(
                            type_b,
                            Type::Int(Int::I32),
                            ops_b,
                            ctx
                        )),
                        CILNode::V2(ctx.alloc_node(bit_cap))
                    )
                )
            }
            _ => {
                shr_un!(
                    ops_a,
                    rem_un!(conv_u32!(ops_b), CILNode::V2(ctx.alloc_node(bit_cap)))
                )
            }
        },
        TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shr!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(
                            type_b,
                            Type::Int(Int::I32),
                            ops_b,
                            ctx
                        )),
                        CILNode::V2(ctx.alloc_node(bit_cap))
                    )
                )
            }
            _ => {
                shr!(
                    ops_a,
                    rem_un!(conv_u32!(ops_b), CILNode::V2(ctx.alloc_node(bit_cap)))
                )
            }
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}

pub fn shl_checked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    let bit_cap = u32::try_from(compiletime_sizeof(value_type, ctx.tcx()) * 8)
        .expect("Intiger has over 2^32 bits.");
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("shl_u128"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::I32)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        conv_u32!(crate::casts::int_to_int(
                            type_b,
                            Type::Int(Int::U32),
                            ops_b,
                            ctx
                        )),
                        CILNode::V2(ctx.alloc_node(bit_cap))
                    ))
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("shl_i128"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I32)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    conv_i32!(rem_un!(
                        conv_u32!(crate::casts::int_to_int(
                            type_b,
                            Type::Int(Int::U32),
                            ops_b,
                            ctx
                        )),
                        CILNode::V2(ctx.alloc_node(bit_cap))
                    ))
                ]
            )
        }
        TyKind::Uint(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shl!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(
                            type_b,
                            Type::Int(Int::I32),
                            ops_b,
                            ctx
                        )),
                        CILNode::V2(ctx.alloc_node(bit_cap))
                    )
                )
            }
            _ => {
                shl!(
                    ops_a,
                    rem_un!(conv_u32!(ops_b), CILNode::V2(ctx.alloc_node(bit_cap)))
                )
            }
        },
        TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shl!(
                    ops_a,
                    rem_un!(
                        conv_u32!(crate::casts::int_to_int(
                            type_b,
                            Type::Int(Int::I32),
                            ops_b,
                            ctx
                        )),
                        CILNode::V2(ctx.alloc_node(bit_cap))
                    )
                )
            }

            _ => {
                shl!(
                    ops_a,
                    rem_un!(conv_u32!(ops_b), CILNode::V2(ctx.alloc_node(bit_cap)))
                )
            }
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}

pub fn shl_unchecked<'tcx>(
    value_type: Ty<'tcx>,
    shift_type: Ty<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    ops_a: CILNode,
    ops_b: CILNode,
) -> CILNode {
    let type_b = ctx.type_from_cache(shift_type);
    match value_type.kind() {
        TyKind::Uint(UintTy::U128) => {
            let mref = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_LeftShift"),
                ctx.sig(
                    [Type::Int(Int::U128), Type::Int(Int::I32)],
                    Type::Int(Int::U128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
                ]
            )
        }
        TyKind::Int(IntTy::I128) => {
            let mref = MethodRef::new(
                ClassRef::int_128(ctx),
                ctx.alloc_string("op_LeftShift"),
                ctx.sig(
                    [Type::Int(Int::I128), Type::Int(Int::I32)],
                    Type::Int(Int::I128),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                ctx.alloc_methodref(mref),
                [
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
                ]
            )
        }
        TyKind::Uint(_) | TyKind::Int(_) => match shift_type.kind() {
            TyKind::Uint(UintTy::U128 | UintTy::U64) | TyKind::Int(IntTy::I128 | IntTy::I64) => {
                shl!(
                    ops_a,
                    crate::casts::int_to_int(type_b, Type::Int(Int::I32), ops_b, ctx)
                )
            }
            _ => shl!(ops_a, ops_b),
        },
        _ => panic!("Can't bitshift type  {value_type:?}"),
    }
}
