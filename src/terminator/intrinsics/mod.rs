use crate::{
    operand::handle_operand,
    place::{place_adress, place_set},
    r#type::{pointer_to_is_fat, tycache::TyCache},
    utilis::field_descrptor,
};
use cilly::{
    call, call_site::CallSite, call_virt, cil_node::CILNode, cil_root::CILRoot, conv_f32, conv_f64,
    conv_isize, conv_u32, conv_u64, conv_usize, eq, field_desc::FieldDescriptor, fn_sig::FnSig,
    ld_field, ldc_i32, ldc_u32, ldc_u64, lt_un, or, size_of, sub, DotnetTypeRef, Type,
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, ParamEnv, TyCtxt, TyKind},
};
use rustc_span::source_map::Spanned;
mod bswap;
mod interop;
pub fn interlocked_add(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::U64 | Type::I64 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "Add".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 | Type::I32 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "Add".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize | Type::ISize => call!(
            CallSite::builtin(
                "interlocked_add_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        _ => todo!(),
    }
}
fn compare_bytes(a: CILNode, b: CILNode, len: CILNode) -> CILNode {
    call!(
        CallSite::builtin(
            "memcmp".into(),
            FnSig::new(
                &[
                    Type::Ptr(Type::U8.into()),
                    Type::Ptr(Type::U8.into()),
                    Type::USize
                ],
                Type::I32
            ),
            true
        ),
        [a, b, len]
    )
}
pub fn handle_intrinsic<'tyctx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
    signature: FnSig,
    span: rustc_span::Span,
) -> CILRoot {
    match fn_name {
        "caller_location" => {
            let caller_loc = tyctx.span_as_caller_location(span);
            let caller_loc_ty = tyctx.caller_location_ty();
            crate::place::place_set(
                destination,
                tyctx,
                crate::constant::load_const_value(
                    caller_loc,
                    caller_loc_ty,
                    tyctx,
                    body,
                    method_instance,
                    type_cache,
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "breakpoint" => {
            debug_assert_eq!(
                args.len(),
                0,
                "The intrinsic `breakpoint` MUST take in exactly 1 argument!"
            );
            CILRoot::Break
        }
        "unlikely" | "likely" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `unlikely` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                body,
                method_instance,
                type_cache,
            )
        }
        "arith_offset" => {
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);

            place_set(
                destination,
                tyctx,
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache)
                    + handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
                        * conv_isize!(size_of!(tpe)),
                body,
                method_instance,
                type_cache,
            )
        }
        "is_val_statically_known" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `is_val_statically_known` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                ldc_i32!(0),
                body,
                method_instance,
                type_cache,
            )
        }
        "needs_drop" => {
            debug_assert_eq!(
                args.len(),
                0,
                "The intrinsic `needs_drop` MUST take in exactly 0 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let needs_drop = tpe.needs_drop(tyctx, ParamEnv::reveal_all());
            let needs_drop = i32::from(needs_drop);
            place_set(
                destination,
                tyctx,
                ldc_i32!(needs_drop),
                body,
                method_instance,
                type_cache,
            )
        }
        "black_box" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `black_box` MUST take in exactly 1 argument!"
            );
            if *signature.output() == Type::Void {
                return CILRoot::Nop;
            }
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(
                destination,
                tyctx,
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                body,
                method_instance,
                type_cache,
            )
        }
        "fmaf64" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `fmaf64` MUST take in exactly 1 argument!"
            );
            let a = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let b = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let c = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);

            place_set(
                destination,
                tyctx,
                a * b + c,
                body,
                method_instance,
                type_cache,
            )
        }
        "ctpop" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctpop` MUST take in exactly 1 argument!"
            );
            let tpe = type_cache.type_from_cache(
                crate::utilis::monomorphize(
                    &method_instance,
                    call_instance.args[0]
                        .as_type()
                        .expect("needs_drop works only on types!"),
                    tyctx,
                ),
                tyctx,
                method_instance,
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            place_set(
                destination,
                tyctx,
                match tpe {
                    Type::U64 => conv_u64!(call!(
                        CallSite::boxed(
                            bit_operations.clone(),
                            "PopCount".into(),
                            FnSig::new(&[Type::U64], Type::I32),
                            true,
                        ),
                        [operand]
                    )),
                    Type::U32 => conv_u32!(call!(
                        CallSite::boxed(
                            bit_operations.clone(),
                            "PopCount".into(),
                            FnSig::new(&[Type::U32], Type::I32),
                            true,
                        ),
                        [operand]
                    )),
                    Type::USize => conv_usize!(call!(
                        CallSite::boxed(
                            bit_operations.clone(),
                            "PopCount".into(),
                            FnSig::new(&[Type::USize], Type::I32),
                            true,
                        ),
                        [operand]
                    )),
                    _ => todo!("Unsported pop count type {tpe:?}"),
                },
                body,
                method_instance,
                type_cache,
            )
        }
        "ctlz" | "ctlz_nonzero" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctlz` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);

            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            // TODO: this assumes a 64 bit system!
            let sub = match tpe {
                Type::ISize | Type::USize | Type::Ptr(_) => {
                    ldc_i32!(64) - (conv_usize!(size_of!(tpe.clone())) * ldc_u32!(8))
                }
                Type::I64 | Type::U64 => ldc_i32!(0),
                Type::I32 | Type::U32 => ldc_i32!(32),
                Type::I16 | Type::U16 => ldc_i32!(48),
                Type::I8 | Type::U8 => ldc_i32!(56),
                Type::I128 => {
                    return place_set(
                        destination,
                        tyctx,
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::int_128(),
                                "LeadingZeroCount".into(),
                                FnSig::new([Type::I128], Type::I128),
                                true
                            ),
                            [handle_operand(
                                &args[0].node,
                                tyctx,
                                body,
                                method_instance,
                                type_cache
                            )]
                        ),
                        body,
                        method_instance,
                        type_cache,
                    )
                }
                Type::U128 => {
                    return place_set(
                        destination,
                        tyctx,
                        call!(
                            CallSite::new_extern(
                                DotnetTypeRef::uint_128(),
                                "LeadingZeroCount".into(),
                                FnSig::new([Type::U128], Type::U128),
                                true
                            ),
                            [handle_operand(
                                &args[0].node,
                                tyctx,
                                body,
                                method_instance,
                                type_cache
                            )]
                        ),
                        body,
                        method_instance,
                        type_cache,
                    )
                }
                _ => todo!("Can't `ctlz`  type {tpe:?} yet!"),
            };
            place_set(
                destination,
                tyctx,
                crate::casts::int_to_int(
                    Type::I32,
                    &tpe,
                    sub!(
                        call!(
                            CallSite::boxed(
                                bit_operations.clone(),
                                "LeadingZeroCount".into(),
                                FnSig::new(&[Type::U64], Type::I32),
                                true,
                            ),
                            [conv_u64!(handle_operand(
                                &args[0].node,
                                tyctx,
                                body,
                                method_instance,
                                type_cache
                            ))]
                        ),
                        sub
                    ),
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "compare_bytes" => place_set(
            destination,
            tyctx,
            compare_bytes(
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                handle_operand(&args[1].node, tyctx, body, method_instance, type_cache),
                handle_operand(&args[2].node, tyctx, body, method_instance, type_cache),
            ),
            body,
            method_instance,
            type_cache,
        ),
        "raw_eq" => {
            // Raw eq returns 0 if values are not equal, and 1 if they are, unlike memcmp, which does the oposite.
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            let size = match tpe {
                Type::Bool
                | Type::U8
                | Type::I8
                | Type::U16
                | Type::I16
                | Type::U32
                | Type::I32
                | Type::U64
                | Type::I64
                | Type::USize
                | Type::ISize
                | Type::Ptr(_) => {
                    return place_set(
                        destination,
                        tyctx,
                        eq!(
                            handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                            handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
                        ),
                        body,
                        method_instance,
                        type_cache,
                    );
                }
                _ => size_of!(tpe),
            };
            place_set(
                destination,
                tyctx,
                eq!(
                    compare_bytes(
                        handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                        handle_operand(&args[1].node, tyctx, body, method_instance, type_cache),
                        conv_usize!(size),
                    ),
                    ldc_i32!(0)
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "bswap" => bswap::bswap(args, destination, tyctx, body, method_instance, type_cache),
        "cttz" | "cttz_nonzero" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ctlz` MUST take in exactly 1 argument!"
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            let bit_operations = Some(bit_operations);
            let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);

            place_set(
                destination,
                tyctx,
                crate::casts::int_to_int(
                    Type::I32,
                    &tpe.clone(),
                    call!(
                        CallSite::boxed(
                            bit_operations.clone(),
                            "TrailingZeroCount".into(),
                            FnSig::new(&[tpe], Type::I32),
                            true,
                        ),
                        [operand]
                    ),
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "rotate_left" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `rotate_left` MUST take in exactly 2 arguments!"
            );
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
            );
            let val_tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let val_tpe = type_cache.type_from_cache(val_tpe, tyctx, method_instance);
            let val = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let rot = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            match val_tpe {
                Type::U8 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(8) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::U16 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(16) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::U32 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(32) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::U64 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::ShrUn(Box::new(val), Box::new(ldc_u32!(64) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::USize => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::ShrUn(
                            Box::new(val),
                            Box::new(size_of!(Type::USize) * ldc_i32!(8) - rot)
                        )
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::I8 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::Shr(Box::new(val), Box::new(ldc_u32!(8) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::I16 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::Shr(Box::new(val), Box::new(ldc_u32!(16) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::I32 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::Shr(Box::new(val), Box::new(ldc_u32!(32) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::I64 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::Shr(Box::new(val), Box::new(ldc_u32!(64) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                Type::ISize => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::Shl(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::Shr(
                            Box::new(val),
                            Box::new(size_of!(Type::ISize) * ldc_i32!(8) - rot)
                        )
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                _ => todo!("Can't ror {val_tpe:?}"),
            }
        }
        "write_bytes" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `write_bytes` MUST take in exactly 3 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let val = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache)
                * conv_usize!(size_of!(tpe));
            CILRoot::InitBlk { dst, val, count }
        }
        "copy" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `copy` MUST take in exactly 3 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            let src = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let dst = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache)
                * conv_usize!(size_of!(tpe));

            CILRoot::CpBlk {
                src,
                dst,
                len: count,
            }
        }
        "exact_div" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `exact_div` MUST take in exactly 2 argument!"
            );

            place_set(
                destination,
                tyctx,
                crate::binop::binop(
                    rustc_middle::mir::BinOp::Div,
                    &args[0].node,
                    &args[1].node,
                    tyctx,
                    body,
                    method_instance,
                    type_cache,
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "type_id" => {
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            let sig = FnSig::new(
                &[DotnetTypeRef::type_handle_type().into()],
                DotnetTypeRef::type_type(),
            );
            let gethash_sig = FnSig::new(&[DotnetTypeRef::type_type().into()], Type::I32);
            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Implicit".into(),
                        FnSig::new(&[Type::U32], Type::U128),
                        true,
                    ),
                    [call_virt!(
                        CallSite::boxed(
                            DotnetTypeRef::object_type().into(),
                            "GetHashCode".into(),
                            gethash_sig,
                            false,
                        ),
                        [call!(
                            CallSite::boxed(
                                DotnetTypeRef::type_type().into(),
                                "GetTypeFromHandle".into(),
                                sig,
                                true,
                            ),
                            [CILNode::LDTypeToken(tpe.into())]
                        )]
                    )]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "volatile_load" => {
            volitale_load(args, destination, tyctx, body, method_instance, type_cache)
        }
        "atomic_load_unordered" => {
            // This is already implemented by default in .NET when volatile is used. TODO: ensure this is 100% right.
            //TODO:fix volitale prefix!
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_unordered` MUST take in exactly 1 argument!"
            );
            let arg =
                crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
            let arg_ty = arg.builtin_deref(true).unwrap();
            let arg = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let ops =
                crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, arg);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "atomic_load_acquire" | "atomic_load_seqcst" => {
            //I am not sure this is implemented propely
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_acquire` MUST take in exactly 1 argument!"
            );
            let ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let arg =
                crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
            let arg_ty = arg.builtin_deref(true).unwrap();

            let ops =
                crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, ops);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "atomic_store_relaxed"
        | "atomic_store_seqcst"
        | "atomic_store_release"
        | "atomic_store_unordered" => {
            // This is *propably* wrong :)
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_acquire` MUST take in exactly 1 argument!"
            );
            let addr = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let val = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let arg_ty =
                crate::utilis::monomorphize(&method_instance, args[1].node.ty(body, tyctx), tyctx);

            crate::place::ptr_set_op(
                arg_ty.into(),
                tyctx,
                &method_instance,
                type_cache,
                addr,
                val,
            )
        }
        "atomic_cxchgweak_acquire_acquire"
        | "atomic_cxchg_acquire_relaxed"
        | "atomic_cxchgweak_acquire_relaxed"
        | "atomic_cxchgweak_relaxed_relaxed" => {
            let interlocked = DotnetTypeRef::interlocked();
            // *T
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            // T
            let old = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            // T
            let src = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `atomic_cxchgweak_acquire_acquire` MUST take in exactly 3 argument!"
            );
            let src_type =
                crate::utilis::monomorphize(&method_instance, args[2].node.ty(body, tyctx), tyctx);
            let src_type = type_cache.type_from_cache(src_type, tyctx, method_instance);

            let call_site = CallSite::new(
                Some(interlocked),
                "CompareExchange".into(),
                FnSig::new(
                    &[
                        Type::ManagedReference(src_type.clone().into()),
                        src_type.clone(),
                        src_type.clone(),
                    ],
                    src_type.clone(),
                ),
                true,
            );

            let value = src;
            let comaprand = old.clone();
            let exchange_res = call!(call_site, [dst, value, comaprand]);
            // Set a field of the destination
            let dst_ty = destination.ty(body, tyctx);
            let fld_desc = field_descrptor(dst_ty.ty, 0, tyctx, method_instance, type_cache);
            assert_eq!(*fld_desc.tpe(), src_type);
            // Set the value of the result.
            let set_val = CILRoot::SetField {
                addr: place_adress(destination, tyctx, body, method_instance, type_cache),
                value: exchange_res,
                desc: fld_desc.clone(),
            };
            // Get the result back
            let val = CILNode::SubTrees(Box::new((
                [set_val].into(),
                ld_field!(
                    place_adress(destination, tyctx, body, method_instance, type_cache),
                    fld_desc
                )
                .into(),
            )));
            // Compare the result to comparand(aka `old`)
            let cmp = eq!(val, old);
            let fld_desc = field_descrptor(dst_ty.ty, 1, tyctx, method_instance, type_cache);
            assert_eq!(*fld_desc.tpe(), Type::Bool);

            CILRoot::SetField {
                addr: place_adress(destination, tyctx, body, method_instance, type_cache),
                value: cmp,
                desc: fld_desc.clone(),
            }
        }
        "atomic_xsub_release" => {
            // *T
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            // T
            let sub_ammount =
                handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            // we sub by adding a negative number
            let add_ammount = CILNode::Neg(Box::new(sub_ammount.clone()));
            let src_type =
                crate::utilis::monomorphize(&method_instance, args[1].node.ty(body, tyctx), tyctx);
            let src_type = type_cache.type_from_cache(src_type, tyctx, method_instance);

            place_set(
                destination,
                tyctx,
                interlocked_add(dst, add_ammount, src_type) + sub_ammount,
                body,
                method_instance,
                type_cache,
            )
        }
        "atomic_fence_acquire" => {
            let thread = DotnetTypeRef::thread();
            CILRoot::Call {
                site: CallSite::new(
                    Some(thread),
                    "MemoryBarrier".into(),
                    FnSig::new(&[], Type::Void),
                    true,
                ),
                args: [].into(),
            }
        }
        "atomic_xadd_release" | "atomic_xadd_relaxed" => {
            // *T
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            // T
            let add_ammount =
                handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            // we sub by adding a negative number

            let src_type =
                crate::utilis::monomorphize(&method_instance, args[1].node.ty(body, tyctx), tyctx);
            let src_type = type_cache.type_from_cache(src_type, tyctx, method_instance);

            place_set(
                destination,
                tyctx,
                interlocked_add(dst, add_ammount, src_type),
                body,
                method_instance,
                type_cache,
            )
        }
        "atomic_xchg_release" => {
            let interlocked = DotnetTypeRef::interlocked();
            // *T
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            // T
            let new = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);

            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `atomic_xchg_release` MUST take in exactly 3 argument!"
            );
            let src_type =
                crate::utilis::monomorphize(&method_instance, args[1].node.ty(body, tyctx), tyctx);
            let src_type = type_cache.type_from_cache(src_type, tyctx, method_instance);

            let call_site = CallSite::new(
                Some(interlocked),
                "Exchange".into(),
                FnSig::new(
                    &[
                        Type::ManagedReference(src_type.clone().into()),
                        src_type.clone(),
                    ],
                    src_type.clone(),
                ),
                true,
            );
            // T
            place_set(
                destination,
                tyctx,
                call!(call_site, [dst, new]),
                body,
                method_instance,
                type_cache,
            )
        }
        // TODO:Those are not stricly neccessary, but SHOULD be implemented at some point.
        "assert_inhabited" | "assert_zero_valid" => CILRoot::Nop,

        "ptr_offset_from_unsigned" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `min_align_of_val` MUST take in exactly 1 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            place_set(
                destination,
                tyctx,
                CILNode::DivUn(
                    sub!(
                        handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                        handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
                    )
                    .into(),
                    conv_usize!(size_of!(tpe)).into(),
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "saturating_add" => {
            let a = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let b = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let a_type = type_cache.type_from_cache(
                crate::utilis::monomorphize(
                    &method_instance,
                    call_instance.args[0]
                        .as_type()
                        .expect("needs_drop works only on types!"),
                    tyctx,
                ),
                tyctx,
                method_instance,
            );
            let calc = match a_type {
                Type::USize | Type::U64 | Type::U32 | Type::U16 | Type::U8 => {
                    let sum = a.clone() + b.clone();
                    let or = a | b;
                    let flag = lt_un!(sum.clone(), or);
                    let max = crate::r#type::max_value(&a_type);
                    CILNode::select(a_type, max, sum, flag)
                }
                _ => todo!("Can't use the intrinsic `saturating_add` on {a_type:?}"),
            };
            place_set(destination, tyctx, calc, body, method_instance, type_cache)
        }
        "saturating_sub" => {
            let a = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let b = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let a_ty = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("saturating_sub works only on types!"),
                tyctx,
            );
            let a_type = type_cache.type_from_cache(a_ty, tyctx, method_instance);
            let calc = match a_type {
                Type::U128 | Type::U64 | Type::U32 | Type::U16 | Type::U8 | Type::USize => {
                    let undeflow = crate::binop::cmp::lt_unchecked(a_ty, a.clone(), b.clone());
                    let diff = crate::binop::sub_unchecked(
                        a_ty,
                        a_ty,
                        tyctx,
                        &method_instance,
                        type_cache,
                        a,
                        b,
                    );
                    let max = crate::binop::checked::zero(a_ty);
                    CILNode::select(a_type, max, diff, undeflow)
                }
                _ => todo!("Can't use the intrinsic `saturating_sub` on {a_type:?}"),
            };
            place_set(destination, tyctx, calc, body, method_instance, type_cache)
        }
        "min_align_of_val" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `min_align_of_val` MUST take in exactly 1 argument!"
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            place_set(
                destination,
                tyctx,
                conv_usize!(ldc_u64!(crate::utilis::align_of(tpe, tyctx) as u64)),
                body,
                method_instance,
                type_cache,
            )
        }
        // .NET guarantess all loads are tear-free
        "atomic_load_relaxed" => {
            //I am not sure this is implemented propely
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_relaxed` MUST take in exactly 1 argument!"
            );
            let ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let arg =
                crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
            let arg_ty = arg.builtin_deref(true).unwrap();

            let ops =
                crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, ops);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "sqrtf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf32` MUST take in exactly 1 argument!"
            );

            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Sqrt".into(),
                        FnSig::new(&[Type::F32], Type::F32),
                        true,
                    ),
                    [handle_operand(
                        &args[0].node,
                        tyctx,
                        body,
                        method_instance,
                        type_cache
                    )]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "floorf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `floorf32` MUST take in exactly 1 argument!"
            );

            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Floor".into(),
                        FnSig::new(&[Type::F32], Type::F32),
                        true,
                    ),
                    [handle_operand(
                        &args[0].node,
                        tyctx,
                        body,
                        method_instance,
                        type_cache
                    )]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "ceilf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `ceilf32` MUST take in exactly 1 argument!"
            );

            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Ceiling".into(),
                        FnSig::new(&[Type::F32], Type::F32),
                        true,
                    ),
                    [handle_operand(
                        &args[0].node,
                        tyctx,
                        body,
                        method_instance,
                        type_cache
                    )]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "maxnumf32" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `maxnumf32` MUST take in exactly 2 arguments!"
            );

            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Max".into(),
                        FnSig::new(&[Type::F32, Type::F32], Type::F32),
                        true,
                    ),
                    [
                        handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                        handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
                    ]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "minnumf32" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `minnumf32` MUST take in exactly 2 arguments!"
            );
            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Min".into(),
                        FnSig::new(&[Type::F32, Type::F32], Type::F32),
                        true,
                    ),
                    [
                        handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                        handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
                    ]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "powif32" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `powif32` MUST take in exactly 2 arguments!"
            );

            place_set(
                destination,
                tyctx,
                conv_f32!(call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::math()),
                        "Pow".into(),
                        FnSig::new(&[Type::F64, Type::F64], Type::F64),
                        true,
                    ),
                    [
                        conv_f64!(handle_operand(
                            &args[0].node,
                            tyctx,
                            body,
                            method_instance,
                            type_cache
                        )),
                        conv_f64!(handle_operand(
                            &args[1].node,
                            tyctx,
                            body,
                            method_instance,
                            type_cache
                        ))
                    ]
                )),
                body,
                method_instance,
                type_cache,
            )
        }
        "size_of_val" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `size_of_val` MUST take in exactly 1 argument!"
            );

            let pointed_ty = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            if crate::utilis::is_zst(pointed_ty, tyctx) {
                return place_set(
                    destination,
                    tyctx,
                    conv_usize!(ldc_u32!(0)),
                    body,
                    method_instance,
                    type_cache,
                );
            }
            if pointer_to_is_fat(pointed_ty, tyctx, method_instance) {
                let ptr_ty = crate::utilis::monomorphize(
                    &method_instance,
                    args[0].node.ty(body, tyctx),
                    tyctx,
                );
                match pointed_ty.kind() {
                    TyKind::Slice(inner) => {
                        let slice_tpe: DotnetTypeRef = type_cache
                            .type_from_cache(ptr_ty, tyctx, method_instance)
                            .as_dotnet()
                            .unwrap();
                        let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                        let inner_type = type_cache.type_from_cache(inner, tyctx, method_instance);
                        let descriptor =
                            FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
                        let addr = crate::operand::operand_address(
                            &args[0].node,
                            tyctx,
                            body,
                            method_instance,
                            type_cache,
                        );
                        return place_set(
                            destination,
                            tyctx,
                            ld_field!(addr, descriptor) * conv_usize!(size_of!(inner_type)),
                            body,
                            method_instance,
                            type_cache,
                        );
                    }
                    TyKind::Dynamic(_, _, _) => {
                        let slice_tpe: DotnetTypeRef = type_cache
                            .type_from_cache(ptr_ty, tyctx, method_instance)
                            .as_dotnet()
                            .unwrap();

                        let descriptor =
                            FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
                        let addr = crate::operand::operand_address(
                            &args[0].node,
                            tyctx,
                            body,
                            method_instance,
                            type_cache,
                        );
                        return place_set(
                            destination,
                            tyctx,
                            CILNode::LDIndISize {
                                ptr: Box::new(
                                    ld_field!(addr, descriptor) + (size_of!(Type::ISize)),
                                ),
                            },
                            body,
                            method_instance,
                            type_cache,
                        );
                    }
                    _ => todo!("Can't yet get `size_of_val` on non-slice dst. dst:{ptr_ty}"),
                }
            }
            let tpe = crate::utilis::monomorphize(&method_instance, pointed_ty, tyctx);
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            place_set(
                destination,
                tyctx,
                conv_usize!(size_of!(tpe)),
                body,
                method_instance,
                type_cache,
            )
        }
        "typed_swap" => {
            let pointed_ty = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = crate::utilis::monomorphize(&method_instance, pointed_ty, tyctx);
            let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
            CILRoot::Call {
                site: CallSite::builtin(
                    "swap_at_generic".into(),
                    FnSig::new(
                        [
                            Type::Ptr(Box::new(Type::Void)),
                            Type::Ptr(Box::new(Type::Void)),
                            Type::USize,
                        ],
                        Type::Void,
                    ),
                    true,
                ),
                args: [
                    handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                    handle_operand(&args[1].node, tyctx, body, method_instance, type_cache),
                    conv_usize!(size_of!(tpe)),
                ]
                .into(),
            }
        }
        "sqrtf64" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
            );

            let ops = call!(
                CallSite::boxed(
                    Some(DotnetTypeRef::math()),
                    "Sqrt".into(),
                    FnSig::new(&[Type::F64], Type::F64),
                    true,
                ),
                [handle_operand(
                    &args[0].node,
                    tyctx,
                    body,
                    method_instance,
                    type_cache
                )]
            );
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "rotate_right" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
            );
            let val_tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let val_tpe = type_cache.type_from_cache(val_tpe, tyctx, method_instance);
            let val = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let rot = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            match val_tpe {
                Type::U32 => place_set(
                    destination,
                    tyctx,
                    or!(
                        CILNode::ShrUn(Box::new(val.clone()), Box::new(rot.clone())),
                        CILNode::Shl(Box::new(val), Box::new(ldc_u32!(32) - rot))
                    ),
                    body,
                    method_instance,
                    type_cache,
                ),
                _ => todo!("Can't ror {val_tpe:?}"),
            }
        }
        "catch_unwind" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `minnumf32` MUST take in exactly 2 arguments!"
            );
            let try_fn = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let data_ptr = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let catch_fn = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);
            let _ = catch_fn;
            eprintln!("WARNING: catching unwinds currently not supported! the intrinic `catch_unwind` WILL NOT CATCH UNWINDS YET!");
            CILRoot::CallI {
                sig: FnSig::new(&[Type::Ptr(Type::U8.into())], Type::Void),
                fn_ptr: try_fn,
                args: Box::new([data_ptr]),
            }
        }
        "abort" => CILRoot::throw("Called abort!"),
        _ => intrinsic_slow(
            fn_name,
            args,
            destination,
            tyctx,
            body,
            method_instance,
            call_instance,
            type_cache,
            &signature,
            span,
        ),
    }
}
fn intrinsic_slow<'tyctx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
    signature: &FnSig,
    span: rustc_span::Span,
) -> CILRoot {
    let _ = call_instance;
    let _ = span;
    let _ = signature;
    if fn_name.contains("likely") {
        debug_assert_eq!(
            args.len(),
            1,
            "The intrinsic `fn_name` MUST take in exactly 1 argument!"
        );
        // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
        place_set(
            destination,
            tyctx,
            handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
            body,
            method_instance,
            type_cache,
        )
    } else if fn_name.contains("volitale_load") {
        return volitale_load(args, destination, tyctx, body, method_instance, type_cache);
    } else if fn_name.contains("is_val_statically_known") {
        debug_assert_eq!(
            args.len(),
            1,
            "The intrinsic `is_val_statically_known` MUST take in exactly 1 argument!"
        );
        // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
        place_set(
            destination,
            tyctx,
            ldc_i32!(0),
            body,
            method_instance,
            type_cache,
        )
    } else if fn_name.contains("typed_swap") {
        let pointed_ty = crate::utilis::monomorphize(
            &method_instance,
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
            tyctx,
        );
        let tpe = crate::utilis::monomorphize(&method_instance, pointed_ty, tyctx);
        let tpe = type_cache.type_from_cache(tpe, tyctx, method_instance);
        CILRoot::Call {
            site: CallSite::builtin(
                "swap_at_generic".into(),
                FnSig::new(
                    [
                        Type::Ptr(Box::new(Type::Void)),
                        Type::Ptr(Box::new(Type::Void)),
                        Type::USize,
                    ],
                    Type::Void,
                ),
                true,
            ),
            args: [
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                handle_operand(&args[1].node, tyctx, body, method_instance, type_cache),
                conv_usize!(size_of!(tpe)),
            ]
            .into(),
        }
    } else {
        todo!("Unhandled intrinsic {fn_name}.")
    }
}
fn volitale_load<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,

    type_cache: &mut TyCache,
) -> CILRoot {
    //TODO:fix volitale prefix!
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `volatile_load` MUST take in exactly 1 argument!"
    );
    let arg = crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
    let arg_ty = arg.builtin_deref(true).unwrap();
    let arg = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
    let ops = crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, arg);
    place_set(destination, tyctx, ops, body, method_instance, type_cache)
}
