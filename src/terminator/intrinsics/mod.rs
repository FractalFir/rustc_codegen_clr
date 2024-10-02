use crate::{
    assembly::MethodCompileCtx,
    operand::handle_operand,
    place::{place_adress, place_set},
    utilis::field_descrptor,
};
use cilly::{
    call, call_virt,
    cil_node::CILNode,
    cil_root::CILRoot,
    cilnode::MethodKind,
    conv_f32, conv_f64, conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u16, conv_u32,
    conv_u64, conv_u8, conv_usize, eq, ld_field, ldc_i32, ldc_u32, ldc_u64, size_of, sub,
    v2::{ClassRef, Float, Int},
    MethodRef, Type,
};
use ints::{ctlz, rotate_left, rotate_right};
use rustc_middle::{
    mir::{Operand, Place},
    ty::{Instance, ParamEnv, Ty, UintTy},
};
use rustc_span::source_map::Spanned;
use saturating::{saturating_add, saturating_sub};
use type_info::{is_val_statically_known, size_of_val};
use utilis::{
    atomic_add, atomic_and, atomic_max, atomic_min, atomic_nand, atomic_or, atomic_xor,
    compare_bytes,
};
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
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
                ctx,
            ),
            ctx,
        ),
        "ctpop" => ints::ctpop(args, destination, call_instance, ctx),
        "bitreverse" => ints::bitreverse(args, destination, ctx, call_instance),
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
        "fmaf32" => {
            let mref = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("FusedMultiplyAdd"),
                ctx.sig(
                    [
                        Type::Float(Float::F32),
                        Type::Float(Float::F32),
                        Type::Float(Float::F32),
                    ],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(mref),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                    handle_operand(&args[2].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "fmaf64" => {
            let mref = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("FusedMultiplyAdd"),
                ctx.sig(
                    [
                        Type::Float(Float::F64),
                        Type::Float(Float::F64),
                        Type::Float(Float::F64),
                    ],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(mref),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                    handle_operand(&args[2].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
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
                | Type::Int(
                    Int::U8
                    | Int::I8
                    | Int::U16
                    | Int::I16
                    | Int::U32
                    | Int::I32
                    | Int::U64
                    | Int::I64
                    | Int::USize
                    | Int::ISize,
                )
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
                        handle_operand(&args[0].node, ctx).cast_ptr(ctx.nptr(Type::Int(Int::U8))),
                        handle_operand(&args[1].node, ctx).cast_ptr(ctx.nptr(Type::Int(Int::U8))),
                        conv_usize!(size),
                        ctx
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
            let type_type = ClassRef::type_type(ctx);
            let runtime_handle = ClassRef::runtime_type_hadle(ctx);
            let sig = ctx.sig([runtime_handle.into()], type_type);
            let gethash_sig = ctx.sig([type_type.into()], Type::Int(Int::I32));
            let op_implict = MethodRef::new(
                ClassRef::uint_128(ctx),
                ctx.alloc_string("op_Implicit"),
                ctx.sig([Type::Int(Int::U32)], Type::Int(Int::U128)),
                MethodKind::Static,
                vec![].into(),
            );
            let get_hash_code = MethodRef::new(
                ClassRef::object(ctx),
                ctx.alloc_string("GetHashCode"),
                gethash_sig,
                MethodKind::Virtual,
                vec![].into(),
            );
            let get_type_handle = MethodRef::new(
                type_type,
                ctx.alloc_string("GetTypeFromHandle"),
                sig,
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(op_implict),
                    [conv_u32!(call_virt!(
                        ctx.alloc_methodref(get_hash_code),
                        [call!(
                            ctx.alloc_methodref(get_type_handle),
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
                2,
                "The intrinsic `{fn_name}` MUST take in exactly 1 argument!"
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
            let interlocked = ClassRef::interlocked(ctx);
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
                    let usize_ref = ctx.nref(Type::Int(Int::USize));
                    let call_site = MethodRef::new(
                        interlocked,
                        ctx.alloc_string("CompareExchange"),
                        ctx.sig(
                            [usize_ref, Type::Int(Int::USize), Type::Int(Int::USize)],
                            Type::Int(Int::USize),
                        ),
                        MethodKind::Static,
                        vec![].into(),
                    );
                    call!(
                        ctx.alloc_methodref(call_site),
                        [
                            Box::new(dst).cast_ptr(usize_ref),
                            conv_usize!(value),
                            conv_usize!(comaprand)
                        ]
                    )
                    .cast_ptr(src_type)
                }
                // TODO: this is a bug, on purpose. The 1 byte compare exchange is not supported untill .NET 9. Remove after November, when .NET 9 Releases.
                Type::Int(Int::U8) => comaprand,
                _ => {
                    let src_ref = ctx.nref(src_type);
                    let call_site = MethodRef::new(
                        interlocked,
                        ctx.alloc_string("CompareExchange"),
                        ctx.sig([src_ref, src_type, src_type], src_type),
                        MethodKind::Static,
                        vec![].into(),
                    );
                    call!(ctx.alloc_methodref(call_site), [dst, value, comaprand])
                }
            };

            // Set a field of the destination
            let dst_ty = destination.ty(ctx.body(), ctx.tcx());
            let fld_desc = field_descrptor(dst_ty.ty, 0, ctx);

            // Set the value of the result.
            let set_val = CILRoot::SetField {
                addr: Box::new(place_adress(destination, ctx)),
                value: Box::new(exchange_res),
                desc: fld_desc,
            };
            // Get the result back
            let val = CILNode::SubTrees(Box::new((
                [set_val].into(),
                ld_field!(place_adress(destination, ctx), fld_desc).into(),
            )));
            // Compare the result to comparand(aka `old`)
            let cmp = eq!(val, old);
            let fld_desc = field_descrptor(dst_ty.ty, 1, ctx);

            CILRoot::SetField {
                addr: Box::new(place_adress(destination, ctx)),
                value: Box::new(cmp),
                desc: fld_desc,
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
                sub!(
                    atomic_add(dst, add_ammount.clone(), src_type, ctx),
                    add_ammount
                ),
                ctx,
            )
        }
        "atomic_or_seqcst" | "atomic_or_release" | "atomic_or_acqrel" | "atomic_or_acquire"
        | "atomic_or_relaxed" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let orand = handle_operand(&args[1].node, ctx);

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(destination, atomic_or(dst, orand, src_type, ctx), ctx)
        }
        "atomic_xor_seqcst" | "atomic_xor_release" | "atomic_xor_acqrel" | "atomic_xor_acquire"
        | "atomic_xor_relaxed" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let xorand = handle_operand(&args[1].node, ctx);

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(destination, atomic_xor(dst, xorand, src_type, ctx), ctx)
        }
        "atomic_and_seqcst" | "atomic_and_release" | "atomic_and_acqrel" | "atomic_and_acquire"
        | "atomic_and_relaxed" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let andand = handle_operand(&args[1].node, ctx);

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(destination, atomic_and(dst, andand, src_type, ctx), ctx)
        }
        "atomic_nand_seqcst"
        | "atomic_nand_release"
        | "atomic_nand_acqrel"
        | "atomic_nand_acquire"
        | "atomic_nand_relaxed" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let andand = handle_operand(&args[1].node, ctx);

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(destination, atomic_nand(dst, andand, src_type, ctx), ctx)
        }
        "atomic_fence_acquire"
        | "atomic_fence_seqcst"
        | "atomic_fence_release"
        | "atomic_fence_acqrel" => {
            let thread = ClassRef::thread(ctx);
            let fence = MethodRef::new(
                thread,
                ctx.alloc_string("MemoryBarrier"),
                ctx.sig([], Type::Void),
                MethodKind::Static,
                vec![].into(),
            );
            CILRoot::Call {
                site: ctx.alloc_methodref(fence),
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
                atomic_add(dst, add_ammount, src_type, ctx),
                ctx,
            )
        }
        "atomic_umin_release"
        | "atomic_umin_relaxed"
        | "atomic_umin_seqcst"
        | "atomic_umin_acqrel"
        | "atomic_umin_acquire"
        | "atomic_min_release"
        | "atomic_min_relaxed"
        | "atomic_min_seqcst"
        | "atomic_min_acqrel"
        | "atomic_min_acquire" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let min_ammount = handle_operand(&args[1].node, ctx);
            // we sub by mining a negative number

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(
                destination,
                atomic_min(dst, min_ammount, src_type, ctx),
                ctx,
            )
        }
        "atomic_umax_release"
        | "atomic_umax_relaxed"
        | "atomic_umax_seqcst"
        | "atomic_umax_acqrel"
        | "atomic_umax_acquire"
        | "atomic_max_release"
        | "atomic_max_relaxed"
        | "atomic_max_seqcst"
        | "atomic_max_acqrel"
        | "atomic_max_acquire" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let max_ammount = handle_operand(&args[1].node, ctx);
            // we sub by maxing a negative number

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);

            place_set(
                destination,
                atomic_max(dst, max_ammount, src_type, ctx),
                ctx,
            )
        }
        "atomic_xchg_release"
        | "atomic_xchg_acquire"
        | "atomic_xchg_acqrel"
        | "atomic_xchg_relaxed"
        | "atomic_xchg_seqcst" => {
            let interlocked = ClassRef::interlocked(ctx);
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
            let uint8_ref = ctx.nref(Type::Int(Int::U8));
            let xchng = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("atomic_xchng_u8"),
                ctx.sig([uint8_ref, Type::Int(Int::U8)], Type::Int(Int::U8)),
                MethodKind::Static,
                vec![].into(),
            );
            match src_type {
                Type::Int(Int::U8) => {
                    return place_set(
                        destination,
                        call!(ctx.alloc_methodref(xchng), [dst, new]),
                        ctx,
                    )
                }
                Type::Ptr(_) => {
                    let usize_ref = ctx.nref(Type::Int(Int::USize));
                    let call_site = MethodRef::new(
                        interlocked,
                        ctx.alloc_string("Exchange"),
                        ctx.sig([usize_ref, Type::Int(Int::USize)], Type::Int(Int::USize)),
                        MethodKind::Static,
                        vec![].into(),
                    );
                    return place_set(
                        destination,
                        call!(
                            ctx.alloc_methodref(call_site),
                            [
                                Box::new(dst).cast_ptr(ctx.nref(Type::Int(Int::USize))),
                                conv_usize!(new),
                            ]
                        )
                        .cast_ptr(src_type),
                        ctx,
                    );
                }
                Type::Int(Int::I8 | Int::U16 | Int::I16) | Type::Bool | Type::PlatformChar => {
                    todo!("can't {fn_name} {src_type:?}")
                }
                _ => (),
            }
            let src_ref = ctx.nref(src_type);
            let call_site = MethodRef::new(
                interlocked,
                ctx.alloc_string("Exchange"),
                ctx.sig([src_ref, src_type], src_type),
                MethodKind::Static,
                vec![].into(),
            );
            // T
            place_set(
                destination,
                call!(ctx.alloc_methodref(call_site), [dst, new]),
                ctx,
            )
        }
        // TODO:Those are not stricly neccessary, but SHOULD be implemented at some point.
        "assert_inhabited" | "assert_zero_valid" | "const_deallocate" => CILRoot::Nop,
        "ptr_offset_from_unsigned" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `ptr_offset_from_unsigned` MUST take in exactly 1 argument!"
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
                    (handle_operand(&args[0].node, ctx) - handle_operand(&args[1].node, ctx))
                        .cast_ptr(Type::Int(Int::USize))
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
            let tpe = ctx.type_from_cache(tpe);
            let tpe = ctx.nptr(tpe);

            place_set(
                destination,
                CILNode::And(
                    Box::new(handle_operand(&args[0].node, ctx).cast_ptr(Type::Int(Int::USize))),
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
                    (handle_operand(&args[0].node, ctx) - handle_operand(&args[1].node, ctx))
                        .cast_ptr(Type::Int(Int::ISize))
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
            let sqrt = MethodRef::new(
                ClassRef::mathf(ctx),
                ctx.alloc_string("Sqrt"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(sqrt),
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
            let pow = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Pow"),
                ctx.sig(
                    [Type::Float(Float::F32), Type::Float(Float::F32)],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(pow),
                    [
                        handle_operand(&args[0].node, ctx),
                        conv_f32!(handle_operand(&args[1].node, ctx))
                    ]
                ),
                ctx,
            )
        }
        "powif64" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `powif64` MUST take in exactly 2 arguments!"
            );
            let pow = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Pow"),
                ctx.sig(
                    [Type::Float(Float::F64), Type::Float(Float::F64)],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(pow),
                    [
                        handle_operand(&args[0].node, ctx),
                        conv_f64!(handle_operand(&args[1].node, ctx))
                    ]
                ),
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
            let void_ptr = ctx.nptr(Type::Void);
            let generic = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("swap_at_generic"),
                ctx.sig([void_ptr, void_ptr, Type::Int(Int::USize)], Type::Void),
                MethodKind::Static,
                vec![].into(),
            );
            CILRoot::Call {
                site: ctx.alloc_methodref(generic),
                args: [
                    handle_operand(&args[0].node, ctx).cast_ptr(void_ptr),
                    handle_operand(&args[1].node, ctx).cast_ptr(void_ptr),
                    conv_usize!(size_of!(tpe)),
                ]
                .into(),
            }
        }

        "type_name" => {
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
                    Type::Int(Int::U8) => conv_u8!(input),
                    Type::Int(Int::U16) => conv_u16!(input),
                    Type::Int(Int::U32) => conv_u32!(input),
                    Type::Int(Int::U64) => conv_u64!(input),
                    Type::Int(Int::USize) => conv_usize!(input),
                    Type::Int(Int::I8) => conv_i8!(input),
                    Type::Int(Int::I16) => conv_i16!(input),
                    Type::Int(Int::I32) => conv_i32!(input),
                    Type::Int(Int::I64) => conv_i64!(input),
                    Type::Int(Int::ISize) => conv_isize!(input),
                    _ => todo!("can't float_to_int_unchecked on {tpe:?}"),
                },
                ctx,
            )
        }
        "fabsf32" => {
            let abs = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Abs"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(abs),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "fabsf64" => {
            let abs = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Abs"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let place_set = place_set(
                destination,
                call!(
                    ctx.alloc_methodref(abs),
                    [handle_operand(&args[0].node, ctx),]
                ),
                ctx,
            );
            place_set
        }
        "expf32" => {
            let exp = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Exp"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(exp),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "expf64" => {
            let exp = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Exp"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(exp),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "logf32" => {
            let log = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Log"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(log),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "logf64" => {
            let log = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Log"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let place_set = place_set(
                destination,
                call!(
                    ctx.alloc_methodref(log),
                    [handle_operand(&args[0].node, ctx),]
                ),
                ctx,
            );
            place_set
        }
        "log2f32" => {
            let log = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Log2"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(log),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "log2f64" => {
            let log = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Log2"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(log),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "log10f32" => {
            let log = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Log10"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(log),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "log10f64" => {
            let log = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Log10"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(log),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "powf32" => {
            let pow = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Pow"),
                ctx.sig(
                    [Type::Float(Float::F32), Type::Float(Float::F32)],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(pow),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "powf64" => {
            let pow = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Pow"),
                ctx.sig(
                    [Type::Float(Float::F64), Type::Float(Float::F64)],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let place_set = place_set(
                destination,
                call!(
                    ctx.alloc_methodref(pow),
                    [
                        handle_operand(&args[0].node, ctx),
                        handle_operand(&args[1].node, ctx),
                    ]
                ),
                ctx,
            );
            place_set
        }
        "copysignf32" => {
            let copy_sign = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("CopySign"),
                ctx.sig(
                    [Type::Float(Float::F32), Type::Float(Float::F32)],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(copy_sign),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "copysignf64" => {
            let copy_sign = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("CopySign"),
                ctx.sig(
                    [Type::Float(Float::F64), Type::Float(Float::F64)],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(copy_sign),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "sinf32" => {
            let sin = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Sin"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(sin),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "sinf64" => {
            let sin = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Sin"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(sin),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "cosf32" => {
            let cos = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Cos"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(cos),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "cosf64" => {
            let cos = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Cos"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(cos),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "exp2f32" => {
            let exp = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("Exp2"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(exp),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "exp2f64" => {
            let exp = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("Exp2"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(exp),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "truncf32" => {
            let trunc = MethodRef::new(
                ClassRef::mathf(ctx),
                ctx.alloc_string("Truncate"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(trunc),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "truncf64" => {
            let trunc = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Truncate"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(trunc),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        // `roundf32` should be a differnt intrinsics, but it requires some .NET fuckery to implement(.NET enums are **wierd**)
        "nearbyintf32" | "rintf32" | "roundevenf32" => {
            let round = MethodRef::new(
                ClassRef::mathf(ctx),
                ctx.alloc_string("Round"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(round),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "roundf32" => {
            let rounding = ClassRef::midpoint_rounding(ctx);
            let round = MethodRef::new(
                ClassRef::mathf(ctx),
                ctx.alloc_string("Round"),
                ctx.sig(
                    [Type::Float(Float::F32), Type::ClassRef(rounding)],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(round),
                [
                    handle_operand(&args[0].node, ctx),
                    ldc_i32!(1).transmute_on_stack(
                        Type::Int(Int::I32),
                        Type::ClassRef(rounding),
                        ctx
                    )
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "nearbyintf64" | "rintf64" | "roundevenf64" => {
            let round = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Round"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(round),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "roundf64" => {
            let rounding = ClassRef::midpoint_rounding(ctx);
            let round = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Round"),
                ctx.sig(
                    [Type::Float(Float::F64), Type::ClassRef(rounding)],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(round),
                [
                    handle_operand(&args[0].node, ctx),
                    ldc_i32!(1).transmute_on_stack(
                        Type::Int(Int::I32),
                        Type::ClassRef(rounding),
                        ctx
                    )
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "floorf32" => {
            let floor = MethodRef::new(
                ClassRef::mathf(ctx),
                ctx.alloc_string("Floor"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(floor),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "floorf64" => {
            let floor = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Floor"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(floor),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "ceilf32" => {
            let ceil = MethodRef::new(
                ClassRef::mathf(ctx),
                ctx.alloc_string("Ceiling"),
                ctx.sig([Type::Float(Float::F32)], Type::Float(Float::F32)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(ceil),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "ceilf64" => {
            let ceil = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Ceiling"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(ceil),
                [handle_operand(&args[0].node, ctx),]
            );
            place_set(destination, value_calc, ctx)
        }
        "maxnumf64" => {
            let max = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("MaxNumber"),
                ctx.sig(
                    [Type::Float(Float::F64), Type::Float(Float::F64)],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(max),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "maxnumf32" => {
            let max = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("MaxNumber"),
                ctx.sig(
                    [Type::Float(Float::F32), Type::Float(Float::F32)],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(max),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "minnumf64" => {
            let min = MethodRef::new(
                ClassRef::double(ctx),
                ctx.alloc_string("MinNumber"),
                ctx.sig(
                    [Type::Float(Float::F64), Type::Float(Float::F64)],
                    Type::Float(Float::F64),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(min),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "minnumf32" => {
            let min = MethodRef::new(
                ClassRef::single(ctx),
                ctx.alloc_string("MinNumber"),
                ctx.sig(
                    [Type::Float(Float::F32), Type::Float(Float::F32)],
                    Type::Float(Float::F32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            let value_calc = call!(
                ctx.alloc_methodref(min),
                [
                    handle_operand(&args[0].node, ctx),
                    handle_operand(&args[1].node, ctx),
                ]
            );
            place_set(destination, value_calc, ctx)
        }
        "variant_count" => {
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
            let sqrt = MethodRef::new(
                ClassRef::math(ctx),
                ctx.alloc_string("Sqrt"),
                ctx.sig([Type::Float(Float::F64)], Type::Float(Float::F64)),
                MethodKind::Static,
                vec![].into(),
            );
            let ops = call!(
                ctx.alloc_methodref(sqrt),
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
            let uint8_ptr = ctx.nptr(Type::Int(Int::U8));
            let try_ptr = ctx.sig([uint8_ptr], Type::Void);
            let catch_ptr = ctx.sig([uint8_ptr, uint8_ptr], Type::Void);
            let catch_unwind = MethodRef::new(
                *ctx.main_module(),
                ctx.alloc_string("catch_unwind"),
                ctx.sig(
                    [Type::FnPtr(try_ptr), uint8_ptr, Type::FnPtr(catch_ptr)],
                    Type::Int(Int::I32),
                ),
                MethodKind::Static,
                vec![].into(),
            );
            place_set(
                destination,
                call!(
                    ctx.alloc_methodref(catch_unwind),
                    [try_fn, data_ptr, catch_fn]
                ),
                ctx,
            )
        }
        "abort" => CILRoot::throw("Called abort!", ctx),
        "const_allocate" => place_set(destination, conv_usize!(ldc_u32!(0)), ctx),
        "vtable_size" => {
            let vtableptr = handle_operand(&args[0].node, ctx);
            place_set(
                destination,
                CILNode::LDIndUSize {
                    ptr: Box::new(
                        (vtableptr + conv_usize!((size_of!(Type::Int(Int::ISize)))))
                            .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                    ),
                },
                ctx,
            )
        }
        "vtable_align" => {
            let vtableptr = handle_operand(&args[0].node, ctx);
            place_set(
                destination,
                CILNode::LDIndUSize {
                    ptr: Box::new(
                        (vtableptr + conv_usize!((size_of!(Type::Int(Int::ISize))) * ldc_i32!(2)))
                            .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                    ),
                },
                ctx,
            )
        }
        _ => intrinsic_slow(fn_name, args, destination, ctx, call_instance, span),
    }
}
fn intrinsic_slow<'tcx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
        volitale_load(args, destination, ctx)
    } else if fn_name.contains("type_id") {
        let tpe = ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        );
        let tpe = ctx.type_from_cache(tpe);
        let type_type = ClassRef::type_type(ctx);
        let rt_type_handle = ClassRef::runtime_type_hadle(ctx);
        let sig = ctx.sig([rt_type_handle.into()], type_type);
        let gethash_sig = ctx.sig([type_type.into()], Type::Int(Int::I32));
        let op_implict = MethodRef::new(
            ClassRef::uint_128(ctx),
            ctx.alloc_string("op_Implicit"),
            ctx.sig([Type::Int(Int::U32)], Type::Int(Int::U128)),
            MethodKind::Static,
            vec![].into(),
        );
        let hash_code = MethodRef::new(
            ClassRef::object(ctx).into(),
            ctx.alloc_string("GetHashCode"),
            gethash_sig,
            MethodKind::Virtual,
            vec![].into(),
        );
        let type_handle = MethodRef::new(
            ClassRef::type_type(ctx).into(),
            ctx.alloc_string("GetTypeFromHandle"),
            sig,
            MethodKind::Static,
            vec![].into(),
        );
        place_set(
            destination,
            call!(
                ctx.alloc_methodref(op_implict),
                [conv_u32!(call_virt!(
                    ctx.alloc_methodref(hash_code),
                    [call!(
                        ctx.alloc_methodref(type_handle),
                        [CILNode::LDTypeToken(tpe.into())]
                    )]
                ))]
            ),
            ctx,
        )
    } else if fn_name.contains("size_of_val") {
        size_of_val(args, destination, ctx, call_instance)
    } else if fn_name.contains("is_val_statically_known") {
        is_val_statically_known(args, destination, ctx)
    } else if fn_name.contains("min_align_of_val") {
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
    } else if fn_name.contains("typed_swap") {
        let pointed_ty = ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        );
        let tpe = ctx.monomorphize(pointed_ty);
        let tpe = ctx.type_from_cache(tpe);
        let void_ptr = ctx.nptr(Type::Void);
        let swap = MethodRef::new(
            *ctx.main_module(),
            ctx.alloc_string("swap_at_generic"),
            ctx.sig([void_ptr, void_ptr, Type::Int(Int::USize)], Type::Void),
            MethodKind::Static,
            vec![].into(),
        );
        CILRoot::Call {
            site: ctx.alloc_methodref(swap),
            args: [
                handle_operand(&args[0].node, ctx).cast_ptr(void_ptr),
                handle_operand(&args[1].node, ctx).cast_ptr(void_ptr),
                conv_usize!(size_of!(tpe)),
            ]
            .into(),
        }
    } else if fn_name.contains("type_name") {
        let const_val = ctx
            .tcx()
            .const_eval_instance(ParamEnv::reveal_all(), call_instance, span)
            .unwrap();
        place_set(
            destination,
            crate::constant::load_const_value(const_val, Ty::new_static_str(ctx.tcx()), ctx),
            ctx,
        )
    } else if fn_name.contains("select_unpredictable") {
        let select_ty = ctx.monomorphize(
            call_instance.args[0]
                .as_type()
                .expect("needs_drop works only on types!"),
        );
        let select_ty = ctx.monomorphize(select_ty);
        let select_ty = ctx.type_from_cache(select_ty);
        place_set(
            destination,
            CILNode::select(
                select_ty,
                handle_operand(&args[1].node, ctx),
                handle_operand(&args[2].node, ctx),
                handle_operand(&args[0].node, ctx),
                ctx,
            ),
            ctx,
        )
    } else if fn_name.contains("const_allocate") {
        place_set(destination, conv_usize!(ldc_u32!(0)), ctx)
    } else if fn_name.contains("const_deallocate") {
        CILRoot::Nop
    } else if fn_name.contains("vtable_size") {
        let vtableptr = handle_operand(&args[0].node, ctx);
        place_set(
            destination,
            CILNode::LDIndUSize {
                ptr: Box::new(
                    (vtableptr + conv_usize!((size_of!(Type::Int(Int::ISize)))))
                        .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                ),
            },
            ctx,
        )
    } else if fn_name.contains("vtable_align") {
        let vtableptr = handle_operand(&args[0].node, ctx);
        place_set(
            destination,
            CILNode::LDIndUSize {
                ptr: Box::new(
                    (vtableptr + conv_usize!((size_of!(Type::Int(Int::ISize))) * ldc_i32!(2)))
                        .cast_ptr(ctx.nptr(Type::Int(Int::USize))),
                ),
            },
            ctx,
        )
    } else if fn_name.contains("ptr_guaranteed_cmp") {
        let lhs = handle_operand(&args[0].node, ctx);
        let rhs = handle_operand(&args[0].node, ctx);
        place_set(destination, conv_u8!(eq!(lhs, rhs)), ctx)
    } else {
        todo!("Unhandled intrinsic {fn_name}.")
    }
}
fn volitale_load<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
    ctx: &mut MethodCompileCtx<'tcx, '_>,
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
