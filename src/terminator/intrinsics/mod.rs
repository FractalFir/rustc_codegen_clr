use crate::cil_tree::cil_root::CILRoot;
use crate::{
    add, call, call_virt, conv_f32, conv_f64, conv_usize, div, ldc_i32, ldc_u64, mul, size_of, sub,
};
use crate::{
    cil::{CILOp, CallSite},
    cil_tree::cil_node::CILNode,
    conv_u64,
    function_sig::FnSig,
    operand::handle_operand,
    place::place_set,
    r#type::{tycache, DotnetTypeRef, Type},
};
use rustc_middle::{
    mir::{Body, Operand, Place},
    ty::{Instance, ParamEnv, TyCtxt},
};
use rustc_span::source_map::Spanned;
use tycache::TyCache;
mod bswap;
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
) -> CILRoot {
    match fn_name {
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
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));

            place_set(
                destination,
                tyctx,
                add!(
                    handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                    mul!(
                        handle_operand(&args[1].node, tyctx, body, method_instance, type_cache),
                        conv_usize!(size_of!(tpe))
                    )
                ),
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
            let needs_drop = if needs_drop { 1 } else { 0 };
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
            if signature.output() == &Type::Void {
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
                add!(mul!(a, b), c),
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
                Some(method_instance),
            );
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);

            place_set(
                destination,
                tyctx,
                crate::casts::int_to_int(
                    Type::I32,
                    tpe,
                    conv_u64!(call!(
                        CallSite::boxed(
                            bit_operations.clone(),
                            "PopCount".into(),
                            FnSig::new(&[Type::U64], &Type::I32),
                            true,
                        ),
                        [operand]
                    )),
                ),
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

            let mut res = Vec::new();
            res.extend(
                handle_operand(&args[0].node, tyctx, body, method_instance, type_cache).flatten(),
            );
            let tpe = crate::utilis::monomorphize(
                &method_instance,
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
                tyctx,
            );
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            let sub = match tpe {
                Type::ISize | Type::USize | Type::Ptr(_) => 0,
                Type::I64 | Type::U64 => 0,
                Type::I32 | Type::U32 => 32,
                Type::I16 | Type::U16 => 48,
                Type::I8 | Type::U8 => 56,
                _ => todo!("Can't `ctlz`  type {tpe:?} yet!"),
            };
            res.extend([
                CILOp::ConvU64(false),
                CILOp::Call(CallSite::boxed(
                    bit_operations.clone(),
                    "LeadingZeroCount".into(),
                    FnSig::new(&[Type::U64], &Type::I32),
                    true,
                )),
                CILOp::LdcI32(sub),
                CILOp::Sub,
            ]);

            place_set(
                destination,
                tyctx,
                crate::casts::int_to_int(
                    Type::I32,
                    tpe,
                    sub!(
                        call!(
                            CallSite::boxed(
                                bit_operations.clone(),
                                "LeadingZeroCount".into(),
                                FnSig::new(&[Type::U64], &Type::I32),
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
                        ldc_i32!(sub)
                    ),
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "compare_bytes" => {
            /*let arg0 = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let arg1 = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let arg2 = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);

            */
            todo!("Can't use `memcmp` yet!");
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
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            let bit_operations = Some(bit_operations);
            let operand = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);

            place_set(
                destination,
                tyctx,
                crate::casts::int_to_int(
                    Type::I32,
                    tpe.clone(),
                    call!(
                        CallSite::boxed(
                            bit_operations.clone(),
                            "TrailingZeroCount".into(),
                            FnSig::new(&[tpe], &Type::I32),
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
            //TODO: ROL is buggy for some int types!
            let bit_operations =
                DotnetTypeRef::new("System.Runtime".into(), "System.Numerics.BitOperations")
                    .with_valuetype(false);
            let bit_operations = Some(bit_operations);
            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        bit_operations.clone(),
                        "RotateLeft".into(),
                        FnSig::new(&[Type::U64, Type::U64], &Type::I32),
                        true,
                    ),
                    [
                        handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                        conv_u64!(handle_operand(
                            &args[1].node,
                            tyctx,
                            body,
                            method_instance,
                            type_cache
                        ))
                    ]
                ),
                body,
                method_instance,
                type_cache,
            )
        }
        "write_bytes" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `write_bytes` MUST take in exactly 3 argument!"
            );
            let dst = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let val = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);
            CILRoot::InitBlk { dst, val, count }
        }
        "copy" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `copy` MUST take in exactly 3 argument!"
            );
            let src = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let dst = handle_operand(&args[1].node, tyctx, body, method_instance, type_cache);
            let count = handle_operand(&args[2].node, tyctx, body, method_instance, type_cache);
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
                div!(
                    handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                    handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
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
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            let sig = FnSig::new(
                &[DotnetTypeRef::type_handle_type().into()],
                &DotnetTypeRef::type_type().into(),
            );
            let gethash_sig = FnSig::new(&[DotnetTypeRef::type_type().into()], &Type::I32);
            place_set(
                destination,
                tyctx,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Implicit".into(),
                        crate::function_sig::FnSig::new(&[Type::U32], &Type::U128),
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
            //TODO:fix volitale prefix!
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `volatile_load` MUST take in exactly 1 argument!"
            );
            let arg =
                crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
            let arg_ty = arg.builtin_deref(true).unwrap().ty;
            let arg = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let ops =
                crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, arg);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
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
            let arg_ty = arg.builtin_deref(true).unwrap().ty;
            let arg = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let ops =
                crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, arg);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        "atomic_load_acquire" => {
            //I am not sure this is implemented propely
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_acquire` MUST take in exactly 1 argument!"
            );
            let ops = handle_operand(&args[0].node, tyctx, body, method_instance, type_cache);
            let arg =
                crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
            let arg_ty = arg.builtin_deref(true).unwrap().ty;

            let ops =
                crate::place::deref_op(arg_ty.into(), tyctx, &method_instance, type_cache, ops);
            place_set(destination, tyctx, ops, body, method_instance, type_cache)
        }
        //"bswap"
        "assert_inhabited" => CILRoot::Nop,
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
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            place_set(
                destination,
                tyctx,
                div!(
                    sub!(
                        handle_operand(&args[0].node, tyctx, body, method_instance, type_cache),
                        handle_operand(&args[1].node, tyctx, body, method_instance, type_cache)
                    ),
                    conv_usize!(size_of!(tpe))
                ),
                body,
                method_instance,
                type_cache,
            )
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
                        Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                        "Sqrt".into(),
                        FnSig::new(&[Type::F32], &Type::F32),
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
                        Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                        "Floor".into(),
                        FnSig::new(&[Type::F32], &Type::F32),
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
                        Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                        "Ceiling".into(),
                        FnSig::new(&[Type::F32], &Type::F32),
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
                        Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                        "Max".into(),
                        FnSig::new(&[Type::F32, Type::F32], &Type::F32),
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
                        Some(DotnetTypeRef::new("System.Runtime".into(), "System.MathF")),
                        "Min".into(),
                        FnSig::new(&[Type::F32, Type::F32], &Type::F32),
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
                        Some(DotnetTypeRef::new("System.Runtime".into(), "System.Math")),
                        "Pow".into(),
                        FnSig::new(&[Type::F64, Type::F64], &Type::F64),
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
            let tpe = args[0].node.ty(body, tyctx);
            let tpe = crate::utilis::monomorphize(&method_instance, tpe, tyctx);
            let tpe = type_cache.type_from_cache(tpe, tyctx, Some(method_instance));
            place_set(
                destination,
                tyctx,
                conv_usize!(size_of!(tpe)),
                body,
                method_instance,
                type_cache,
            )
        }
        "sqrtf64" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf64` MUST take in exactly 1 argument!"
            );

            let ops = call!(
                CallSite::boxed(
                    Some(DotnetTypeRef::new("System.Runtime".into(), "System.Math")),
                    "Sqrt".into(),
                    FnSig::new(&[Type::F64], &Type::F64),
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
            signature,
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
    _call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
    _signature: FnSig,
) -> CILRoot {
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
    } else {
        todo!("Unhandled intrinsic {fn_name}.")
    }
}
