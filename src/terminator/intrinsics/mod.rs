use crate::{
    assembly::MethodCompileCtx,
    operand::handle_operand,
    place::{place_adress, place_set},
    utilis::field_descrptor,
};
use cilly::{
    call, call_site::CallSite, call_virt, cil_node::CILNode, cil_root::CILRoot, conv_f32, conv_f64,
    conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8,
    conv_usize, eq, fn_sig::FnSig, ld_field, ldc_i32, ldc_u64, ptr, size_of, sub, DotnetTypeRef,
    Type,
};
use ints::{ctlz, rotate_left, rotate_right};
use rustc_middle::{
    mir::{Operand, Place},
    ty::{Instance, ParamEnv, Ty, UintTy},
};
use rustc_span::source_map::Spanned;
use saturating::{saturating_add, saturating_sub};
use type_info::{is_val_statically_known, size_of_val};
use utilis::{compare_bytes, interlocked_add, interlocked_or};
mod bswap;
mod interop;
mod ints;
mod saturating;
mod type_info;
mod utilis;
pub fn handle_intrinsic<'tcx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    span: rustc_span::Span,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILRoot {
    match fn_name {
        "arith_offset" => {
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);

            place_set(
                destination,
                handle_operand(&args[0].node, ctx)
                    + handle_operand(&args[1].node, ctx) * conv_isize!(size_of!(tpe)),
                ctx,
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
        "black_box" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `black_box` MUST take in exactly 1 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
            if tpe == Type::Void {
                return CILRoot::Nop;
            }
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(destination, handle_operand(&args[0].node, ctx), ctx)
        }
        "caller_location" => caller_location(destination, ctx, span),
        "compare_bytes" => place_set(
            destination,
            compare_bytes(
                handle_operand(&args[0].node, ctx),
                handle_operand(&args[1].node, ctx),
                handle_operand(&args[2].node, ctx),
            ),
            ctx,
        ),
        "ctpop" => ints::ctpop(args, destination, call_instance, ctx),
        "ctlz" | "ctlz_nonzero" => ctlz(args, destination, call_instance, ctx),
        "unlikely" | "likely" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `unlikely` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            place_set(destination, handle_operand(&args[0].node, ctx), ctx)
        }
        "is_val_statically_known" => is_val_statically_known(args, destination, ctx),
        "needs_drop" => {
            debug_assert_eq!(
                args.len(),
                0,
                "The intrinsic `needs_drop` MUST take in exactly 0 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let needs_drop = tpe.needs_drop(ctx.tcx(), ParamEnv::reveal_all());
            let needs_drop = i32::from(needs_drop);
            place_set(destination, ldc_i32!(needs_drop), ctx)
        }

        "fmaf64" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `fmaf64` MUST take in exactly 1 argument!"
            );
            let a = handle_operand(&args[0].node, ctx);
            let b = handle_operand(&args[1].node, ctx);
            let c = handle_operand(&args[2].node, ctx);

            place_set(destination, a * b + c, ctx)
        }

        "raw_eq" => {
            // Raw eq returns 0 if values are not equal, and 1 if they are, unlike memcmp, which does the oposite.
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
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
                        eq!(
                            handle_operand(&args[0].node, ctx),
                            handle_operand(&args[1].node, ctx)
                        ),
                        ctx,
                    );
                }
                _ => size_of!(tpe),
            };
            place_set(
                destination,
                eq!(
                    compare_bytes(
                        CILNode::TransmutePtr {
                            val: Box::new(handle_operand(&args[0].node, ctx)),
                            new_ptr: Box::new(Type::Ptr(Box::new(Type::U8))),
                        },
                        CILNode::TransmutePtr {
                            val: Box::new(handle_operand(&args[1].node, ctx)),
                            new_ptr: Box::new(Type::Ptr(Box::new(Type::U8))),
                        },
                        conv_usize!(size),
                    ),
                    ldc_i32!(0)
                ),
                ctx,
            )
        }
        "bswap" => bswap::bswap(args, destination, ctx),
        "cttz" | "cttz_nonzero" => ints::cttz(args, destination, ctx, call_instance),
        "rotate_left" => rotate_left(args, destination, ctx, call_instance),
        "write_bytes" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `write_bytes` MUST take in exactly 3 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
            let dst = handle_operand(&args[0].node, ctx);
            let val = handle_operand(&args[1].node, ctx);
            let count = handle_operand(&args[2].node, ctx) * conv_usize!(size_of!(tpe));
            CILRoot::InitBlk {
                dst: Box::new(dst),
                val: Box::new(val),
                count: Box::new(count),
            }
        }
        "copy" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `copy` MUST take in exactly 3 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
            let src = handle_operand(&args[0].node, ctx);
            let dst = handle_operand(&args[1].node, ctx);
            let count = handle_operand(&args[2].node, ctx) * conv_usize!(size_of!(tpe));

            CILRoot::CpBlk {
                src: Box::new(src),
                dst: Box::new(dst),
                len: Box::new(count),
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
                crate::binop::binop(
                    rustc_middle::mir::BinOp::Div,
                    &args[0].node,
                    &args[1].node,
                    ctx,
                ),
                ctx,
            )
        }
        "type_id" => {
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
            let sig = FnSig::new(
                &[DotnetTypeRef::type_handle_type().into()],
                DotnetTypeRef::type_type(),
            );
            let gethash_sig = FnSig::new(&[DotnetTypeRef::type_type().into()], Type::I32);
            place_set(
                destination,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Implicit".into(),
                        FnSig::new(&[Type::U32], Type::U128),
                        true,
                    ),
                    [conv_u32!(call_virt!(
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
                    ))]
                ),
                ctx,
            )
        }
        "volatile_load" => volitale_load(args, destination, ctx),
        "volatile_store" => {
            let pointed_type = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let addr_calc = handle_operand(&args[0].node, ctx);
            let value_calc = handle_operand(&args[1].node, ctx);
            CILRoot::Volatile(Box::new(crate::place::ptr_set_op(
                pointed_type.into(),
                ctx,
                addr_calc,
                value_calc,
            )))
        }
        "atomic_load_unordered" => {
            // This is already implemented by default in .NET when volatile is used. TODO: ensure this is 100% right.
            //TODO:fix volitale prefix!
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_unordered` MUST take in exactly 1 argument!"
            );
            let arg = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
            let arg_ty = arg.builtin_deref(true).unwrap();
            let arg = handle_operand(&args[0].node, ctx);
            let ops = crate::place::deref_op(arg_ty.into(), ctx, arg);
            place_set(destination, ops, ctx)
        }
        "atomic_load_acquire" | "atomic_load_seqcst" => {
            //I am not sure this is implemented propely
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_acquire` MUST take in exactly 1 argument!"
            );
            let ops = handle_operand(&args[0].node, ctx);
            let arg = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
            let arg_ty = arg.builtin_deref(true).unwrap();

            let ops = crate::place::deref_op(arg_ty.into(), ctx, ops);
            place_set(destination, ops, ctx)
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
            let addr = handle_operand(&args[0].node, ctx);
            let val = handle_operand(&args[1].node, ctx);
            let arg_ty = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));

            crate::place::ptr_set_op(arg_ty.into(), ctx, addr, val)
        }
        "atomic_cxchgweak_acquire_acquire"
        | "atomic_cxchgweak_acquire_relaxed"
        | "atomic_cxchgweak_relaxed_relaxed"
        | "atomic_cxchgweak_relaxed_acquire"
        | "atomic_cxchgweak_seqcst_acquire"
        | "atomic_cxchgweak_seqcst_seqcst"
        | "atomic_cxchgweak_seqcst_relaxed"
        | "atomic_cxchg_acqrel_acquire"
        | "atomic_cxchg_acquire_seqcst"
        | "atomic_cxchg_release_relaxed"
        | "atomic_cxchg_relaxed_acquire"
        | "atomic_cxchg_acquire_relaxed"
        | "atomic_cxchg_relaxed_seqcst"
        | "atomic_cxchg_acquire_acquire"
        | "atomic_cxchg_release_acquire"
        | "atomic_cxchg_release_seqcst"
        | "atomic_cxchgweak_relaxed_seqcst"
        | "atomic_cxchgweak_acquire_seqcst"
        | "atomic_cxchgweak_release_relaxed"
        | "atomic_cxchgweak_release_acquire"
        | "atomic_cxchgweak_release_seqcst"
        | "atomic_cxchgweak_acqrel_relaxed"
        | "atomic_cxchgweak_acqrel_acquire"
        | "atomic_cxchgweak_acqrel_seqcst"
        | "atomic_cxchg_seqcst_seqcst"
        | "atomic_cxchg_seqcst_acquire"
        | "atomic_cxchg_seqcst_relaxed"
        | "atomic_cxchg_acqrel_relaxed"
        | "atomic_cxchg_relaxed_relaxed"
        | "atomic_cxchg_acqrel_seqcst" => {
            let interlocked = DotnetTypeRef::interlocked();
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let old = handle_operand(&args[1].node, ctx);
            // T
            let src = handle_operand(&args[2].node, ctx);
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `atomic_cxchgweak_acquire_acquire` MUST take in exactly 3 argument!"
            );
            let src_type = ctx.monomorphize(args[2].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            let value = src;
            let comaprand = old.clone();
            #[allow(clippy::single_match_else)]
            let exchange_res = match &src_type {
                Type::Ptr(_) => {
                    let call_site = CallSite::new(
                        Some(interlocked),
                        "CompareExchange".into(),
                        FnSig::new(
                            &[
                                Type::ManagedReference(Type::USize.clone().into()),
                                Type::USize.clone(),
                                Type::USize.clone(),
                            ],
                            Type::USize.clone(),
                        ),
                        true,
                    );
                    call!(
                        call_site,
                        [
                            Box::new(dst)
                                .cast_ptr(Type::ManagedReference(Type::USize.clone().into())),
                            conv_usize!(value),
                            conv_usize!(comaprand)
                        ]
                    )
                    .cast_ptr(src_type.clone())
                }
                _ => {
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
                    call!(call_site, [dst, value, comaprand])
                }
            };

            // Set a field of the destination
            let dst_ty = destination.ty(ctx.body(), ctx.tcx());
            let fld_desc = field_descrptor(dst_ty.ty, 0, ctx);
            assert_eq!(*fld_desc.tpe(), src_type);
            // Set the value of the result.
            let set_val = CILRoot::SetField {
                addr: Box::new(place_adress(destination, ctx)),
                value: Box::new(exchange_res),
                desc: Box::new(fld_desc.clone()),
            };
            // Get the result back
            let val = CILNode::SubTrees(Box::new((
                [set_val].into(),
                ld_field!(place_adress(destination, ctx), fld_desc).into(),
            )));
            // Compare the result to comparand(aka `old`)
            let cmp = eq!(val, old);
            let fld_desc = field_descrptor(dst_ty.ty, 1, ctx);
            assert_eq!(*fld_desc.tpe(), Type::Bool);

            CILRoot::SetField {
                addr: Box::new(place_adress(destination, ctx)),
                value: Box::new(cmp),
                desc: Box::new(fld_desc.clone()),
            }
        }
        "atomic_xsub_release"
        | "atomic_xsub_acqrel"
        | "atomic_xsub_acquire"
        | "atomic_xsub_relaxed"
        | "atomic_xsub_seqcst" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let sub_ammount = handle_operand(&args[1].node, ctx);
            // we sub by adding a negative number
            let add_ammount = CILNode::Neg(Box::new(sub_ammount.clone()));
            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(
                destination,
                interlocked_add(dst, add_ammount, src_type),
                ctx,
            )
        }
        "atomic_or_seqcst" | "atomic_or_release" | "atomic_or_acqrel" | "atomic_or_acquire"
        | "atomic_or_relaxed" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let orand = handle_operand(&args[1].node, ctx);
            // we sub by adding a negative number

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(destination, interlocked_or(dst, orand, src_type), ctx)
        }
        "atomic_fence_acquire"
        | "atomic_fence_seqcst"
        | "atomic_fence_release"
        | "atomic_fence_acqrel" => {
            let thread = DotnetTypeRef::thread();
            CILRoot::Call {
                site: Box::new(CallSite::new(
                    Some(thread),
                    "MemoryBarrier".into(),
                    FnSig::new(&[], Type::Void),
                    true,
                )),
                args: [].into(),
            }
        }
        "atomic_xadd_release"
        | "atomic_xadd_relaxed"
        | "atomic_xadd_seqcst"
        | "atomic_xadd_acqrel"
        | "atomic_xadd_acquire" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let add_ammount = handle_operand(&args[1].node, ctx);
            // we sub by adding a negative number

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(
                destination,
                interlocked_add(dst, add_ammount, src_type),
                ctx,
            )
        }
        "atomic_xchg_release"
        | "atomic_xchg_acquire"
        | "atomic_xchg_acqrel"
        | "atomic_xchg_relaxed"
        | "atomic_xchg_seqcst" => {
            let interlocked = DotnetTypeRef::interlocked();
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let new = handle_operand(&args[1].node, ctx);

            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `atomic_xchg_release` MUST take in exactly 3 argument!"
            );
            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);
            match src_type {
                Type::U8 => {
                    return place_set(
                        destination,
                        call!(
                            CallSite::builtin(
                                "interlocked_emulate_xchng_byte".into(),
                                FnSig::new(
                                    &[Type::ManagedReference(Box::new(Type::U8)), Type::U8],
                                    Type::U8
                                ),
                                true
                            ),
                            [dst, new]
                        ),
                        ctx,
                    )
                }
                Type::Ptr(_) => {
                    let call_site = CallSite::new(
                        Some(interlocked),
                        "Exchange".into(),
                        FnSig::new(
                            &[
                                Type::ManagedReference(Type::USize.clone().into()),
                                Type::USize.clone(),
                            ],
                            Type::USize.clone(),
                        ),
                        true,
                    );
                    return place_set(
                        destination,
                        call!(
                            call_site,
                            [
                                Box::new(dst)
                                    .cast_ptr(Type::ManagedReference(Type::USize.clone().into())),
                                conv_usize!(new),
                            ]
                        )
                        .cast_ptr(src_type.clone()),
                        ctx,
                    );
                }
                Type::I8 | Type::Bool | Type::U16 | Type::I16 | Type::DotnetChar => {
                    todo!("can't {fn_name} {src_type:?}")
                }
                _ => (),
            }
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
            place_set(destination, call!(call_site, [dst, new]), ctx)
        }
        // TODO:Those are not stricly neccessary, but SHOULD be implemented at some point.
        "assert_inhabited" | "assert_zero_valid" => CILRoot::Nop,
        "ptr_offset_from_unsigned" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `min_align_of_val` MUST take in exactly 1 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
            place_set(
                destination,
                CILNode::DivUn(
                    CILNode::TransmutePtr {
                        val: sub!(
                            handle_operand(&args[0].node, ctx),
                            handle_operand(&args[1].node, ctx)
                        )
                        .into(),
                        new_ptr: Box::new(Type::USize),
                    }
                    .into(),
                    conv_usize!(size_of!(tpe)).into(),
                ),
                ctx,
            )
        }
        "ptr_mask" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `ptr_mask` MUST take in exactly 2 arguments!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ptr!(ctx.type_from_cache(tpe));

            place_set(
                destination,
                CILNode::And(
                    Box::new(handle_operand(&args[0].node, ctx).cast_ptr(Type::USize)),
                    Box::new(handle_operand(&args[1].node, ctx)),
                )
                .cast_ptr(tpe),
                ctx,
            )
        }
        "ptr_offset_from" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `ptr_offset_from` MUST take in exactly 1 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.type_from_cache(tpe);
            place_set(
                destination,
                CILNode::Div(
                    CILNode::TransmutePtr {
                        val: sub!(
                            handle_operand(&args[0].node, ctx),
                            handle_operand(&args[1].node, ctx)
                        )
                        .into(),
                        new_ptr: Box::new(Type::ISize),
                    }
                    .into(),
                    conv_isize!(size_of!(tpe)).into(),
                ),
                ctx,
            )
        }
        "saturating_add" => saturating_add(args, destination, ctx, call_instance),
        "saturating_sub" => saturating_sub(args, destination, ctx, call_instance),
        "min_align_of_val" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `min_align_of_val` MUST take in exactly 1 argument!"
            );
            let tpe = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            place_set(
                destination,
                conv_usize!(ldc_u64!(crate::utilis::align_of(tpe, ctx.tcx()))),
                ctx,
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
            let ops = handle_operand(&args[0].node, ctx);
            let arg = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
            let arg_ty = arg.builtin_deref(true).unwrap();

            let ops = crate::place::deref_op(arg_ty.into(), ctx, ops);
            place_set(destination, ops, ctx)
        }
        "sqrtf32" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `sqrtf32` MUST take in exactly 1 argument!"
            );
            place_set(
                destination,
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Sqrt".into(),
                        FnSig::new(&[Type::F32], Type::F32),
                        true,
                    ),
                    [handle_operand(&args[0].node, ctx)]
                ),
                ctx,
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
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Floor".into(),
                        FnSig::new(&[Type::F32], Type::F32),
                        true,
                    ),
                    [handle_operand(&args[0].node, ctx)]
                ),
                ctx,
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
                call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::mathf()),
                        "Ceiling".into(),
                        FnSig::new(&[Type::F32], Type::F32),
                        true,
                    ),
                    [handle_operand(&args[0].node, ctx)]
                ),
                ctx,
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
                conv_f32!(call!(
                    CallSite::boxed(
                        Some(DotnetTypeRef::math()),
                        "Pow".into(),
                        FnSig::new(&[Type::F64, Type::F64], Type::F64),
                        true,
                    ),
                    [
                        conv_f64!(handle_operand(&args[0].node, ctx)),
                        conv_f64!(handle_operand(&args[1].node, ctx))
                    ]
                )),
                ctx,
            )
        }
        "size_of_val" => size_of_val(args, destination, ctx, call_instance),
        "typed_swap" => {
            let pointed_ty = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.monomorphize(pointed_ty);
            let tpe = ctx.type_from_cache(tpe);
            CILRoot::Call {
                site: Box::new(CallSite::builtin(
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
                )),
                args: [
                    CILNode::TransmutePtr {
                        val: Box::new(handle_operand(&args[0].node, ctx)),
                        new_ptr: Box::new(Type::Ptr(Box::new(Type::Void))),
                    },
                    CILNode::TransmutePtr {
                        val: Box::new(handle_operand(&args[1].node, ctx)),
                        new_ptr: Box::new(Type::Ptr(Box::new(Type::Void))),
                    },
                    conv_usize!(size_of!(tpe)),
                ]
                .into(),
            }
        }
        "type_name" => {
            let ty = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let const_val = ctx
                .tcx()
                .const_eval_instance(ParamEnv::reveal_all(), call_instance, span)
                .unwrap();
            place_set(
                destination,
                crate::constant::load_const_value(const_val, Ty::new_static_str(ctx.tcx()), ctx),
                ctx,
            )
        }
        "float_to_int_unchecked" => {
            let tpe = ctx.monomorphize(
                call_instance.args[1]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let tpe = ctx.monomorphize(tpe);
            let tpe = ctx.type_from_cache(tpe);
            let input = handle_operand(&args[0].node, ctx);
            place_set(
                destination,
                match tpe {
                    Type::U8 => conv_u8!(input),
                    Type::U16 => conv_u16!(input),
                    Type::U32 => conv_u32!(input),
                    Type::U64 => conv_u64!(input),
                    Type::USize => conv_usize!(input),
                    Type::I8 => conv_i8!(input),
                    Type::I16 => conv_i16!(input),
                    Type::I32 => conv_i32!(input),
                    Type::I64 => conv_i64!(input),
                    Type::ISize => conv_isize!(input),
                    _ => todo!("can't float_to_int_unchecked on {tpe:?}"),
                },
                ctx,
            )
        }
        "fabsf32" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::single(),
                    "Abs".into(),
                    FnSig::new([Type::F32], Type::F32),
                    true
                ),
                [handle_operand(&args[0].node, ctx),]
            ),
            ctx,
        ),
        "fabsf64" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::double(),
                    "Abs".into(),
                    FnSig::new([Type::F64], Type::F64),
                    true
                ),
                [handle_operand(&args[0].node, ctx),]
            ),
            ctx,
        ),
        "truncf32" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::mathf(),
                    "Truncate".into(),
                    FnSig::new([Type::F32], Type::F32),
                    true
                ),
                [handle_operand(&args[0].node, ctx),]
            ),
            ctx,
        ),
        "truncf64" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::math(),
                    "Truncate".into(),
                    FnSig::new([Type::F64], Type::F64),
                    true
                ),
                [handle_operand(&args[0].node, ctx),]
            ),
            ctx,
        ),
        "maxnumf64" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::double(),
                    "Max".into(),
                    FnSig::new([Type::F64, Type::F64], Type::F64),
                    true
                ),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            ),
            ctx,
        ),
        "maxnumf32" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::single(),
                    "Max".into(),
                    FnSig::new([Type::F32, Type::F32], Type::F32),
                    true
                ),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            ),
            ctx,
        ),
        "minnumf64" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::double(),
                    "Min".into(),
                    FnSig::new([Type::F64, Type::F64], Type::F64),
                    true
                ),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            ),
            ctx,
        ),
        "minnumf32" => place_set(
            destination,
            call!(
                CallSite::new_extern(
                    DotnetTypeRef::single(),
                    "Min".into(),
                    FnSig::new([Type::F32, Type::F32], Type::F32),
                    true
                ),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            ),
            ctx,
        ),
        "variant_count" => {
            let ty = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let const_val = ctx
                .tcx()
                .const_eval_instance(ParamEnv::reveal_all(), call_instance, span)
                .unwrap();
            place_set(
                destination,
                crate::constant::load_const_value(
                    const_val,
                    Ty::new_uint(ctx.tcx(), UintTy::Usize),
                    ctx,
                ),
                ctx,
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
                    Some(DotnetTypeRef::math()),
                    "Sqrt".into(),
                    FnSig::new(&[Type::F64], Type::F64),
                    true,
                ),
                [handle_operand(&args[0].node, ctx)]
            );
            place_set(destination, ops, ctx)
        }
        "rotate_right" => rotate_right(args, destination, ctx, call_instance),
        "catch_unwind" => {
            debug_assert_eq!(
                args.len(),
                3,
                "The intrinsic `catch_unwind` MUST take in exactly 3 arguments!"
            );
            let try_fn = handle_operand(&args[0].node, ctx);
            let data_ptr = handle_operand(&args[1].node, ctx);
            let catch_fn = handle_operand(&args[2].node, ctx);

            place_set(
                destination,
                call!(
                    CallSite::builtin(
                        "catch_unwind".into(),
                        FnSig::new(
                            &[
                                Type::DelegatePtr(Box::new(FnSig::new(
                                    &[ptr!(Type::U8)],
                                    Type::Void
                                ))),
                                ptr!(Type::U8),
                                Type::DelegatePtr(Box::new(FnSig::new(
                                    &[ptr!(Type::U8), ptr!(Type::U8)],
                                    Type::Void
                                ))),
                            ],
                            Type::I32,
                        ),
                        true
                    ),
                    [try_fn, data_ptr, catch_fn]
                ),
                ctx,
            )
        }
        "abort" => CILRoot::throw("Called abort!"),
        _ => intrinsic_slow(fn_name, args, destination, ctx, call_instance, span),
    }
}
fn intrinsic_slow<'tcx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    call_instance: Instance<'tcx>,
    span: rustc_span::Span,
) -> CILRoot {
    let _ = span;

    if fn_name.contains("likely") {
        debug_assert_eq!(
            args.len(),
            1,
            "The intrinsic `fn_name` MUST take in exactly 1 argument!"
        );
        // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
        place_set(destination, handle_operand(&args[0].node, ctx), ctx)
    } else if fn_name.contains("volitale_load") {
        return volitale_load(args, destination, ctx);
    } else if fn_name.contains("is_val_statically_known") {
        is_val_statically_known(args, destination, ctx)
    } else if fn_name.contains("typed_swap") {
        let pointed_ty = ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        );
        let tpe = ctx.monomorphize(pointed_ty);
        let tpe = ctx.type_from_cache(tpe);
        CILRoot::Call {
            site: Box::new(CallSite::builtin(
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
            )),
            args: [
                CILNode::TransmutePtr {
                    val: Box::new(handle_operand(&args[0].node, ctx)),
                    new_ptr: Box::new(Type::Ptr(Box::new(Type::Void))),
                },
                CILNode::TransmutePtr {
                    val: Box::new(handle_operand(&args[1].node, ctx)),
                    new_ptr: Box::new(Type::Ptr(Box::new(Type::Void))),
                },
                conv_usize!(size_of!(tpe)),
            ]
            .into(),
        }
    } else {
        todo!("Unhandled intrinsic {fn_name}.")
    }
}
fn volitale_load<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> CILRoot {
    //TODO:fix volitale prefix!
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `volatile_load` MUST take in exactly 1 argument!"
    );
    let arg = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
    let arg_ty = arg.builtin_deref(true).unwrap();
    let arg = handle_operand(&args[0].node, ctx);
    let ops = CILNode::Volatile(Box::new(crate::place::deref_op(arg_ty.into(), ctx, arg)));
    place_set(destination, ops, ctx)
}
fn caller_location<'tcx>(
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
    span: rustc_span::Span,
) -> CILRoot {
    let caller_loc = ctx.tcx().span_as_caller_location(span);
    let caller_loc_ty = ctx.tcx().caller_location_ty();
    crate::place::place_set(
        destination,
        crate::constant::load_const_value(caller_loc, caller_loc_ty, ctx),
        ctx,
    )
}
