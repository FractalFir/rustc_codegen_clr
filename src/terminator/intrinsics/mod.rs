use crate::{assembly::MethodCompileCtx, casts};
use cilly::{
    call,
    cil_node::V1Node,
    cil_root::V1Root,
    cilnode::MethodKind,
    conv_i16, conv_i32, conv_i64, conv_i8, conv_isize, conv_u16, conv_u32, conv_u64, conv_u8,
    conv_usize, Const, FieldDesc, IntoAsmIndex, MethodRef, Type, {ClassRef, Float, Int},
};
use ints::{ctlz, rotate_left, rotate_right};
use rustc_codegen_clr_place::{place_address, place_set, ptr_set_op};
use rustc_codegen_clr_type::GetTypeExt;
use rustc_codgen_clr_operand::{constant::load_const_value, handle_operand, operand_address};
use rustc_middle::ty::TypingEnv;
use rustc_middle::{
    mir::{Operand, Place},
    ty::{Instance, Ty, UintTy},
};
use rustc_span::source_map::Spanned;
use saturating::{saturating_add, saturating_sub};
use type_info::{is_val_statically_known, size_of_val};
use utilis::{
    atomic_add, atomic_and, atomic_max, atomic_min, atomic_nand, atomic_or, atomic_xor,
    compare_bytes,
};
mod bswap;
mod floats;
mod interop;
mod ints;
mod saturating;
mod type_info;
mod utilis;
use floats::{fmaf32, fmaf64, powf32, powf64, powif32, powif64, roundf32, roundf64};
mod ptr;
use ptr::arith_offset;
mod mem;
use mem::{copy, raw_eq, write_bytes};
mod atomic;
mod tpe;
mod vtable;
fn call_atomic<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    atomic: fn(addr: V1Node, addend: V1Node, tpe: Type, asm: &mut cilly::Assembly) -> V1Node,
) -> Vec<V1Root> {
    // *T
    let dst = handle_operand(&args[0].node, ctx);
    // T
    let arg = handle_operand(&args[1].node, ctx);

    let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
    let src_type = ctx.type_from_cache(src_type);

    vec![place_set(destination, atomic(dst, arg, src_type, ctx), ctx)]
}

pub fn breakpoint(args: &[Spanned<Operand<'_>>]) -> V1Root {
    debug_assert_eq!(
        args.len(),
        0,
        "The intrinsic `breakpoint` MUST take in no arguments!"
    );
    V1Root::Break
}
pub fn black_box<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Root {
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
        return V1Root::Nop;
    }
    // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
    place_set(destination, handle_operand(&args[0].node, ctx), ctx)
}

pub fn handle_intrinsic<'tcx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    call_instance: Instance<'tcx>,
    span: rustc_span::Span,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> Vec<V1Root> {
    match fn_name {
        "arith_offset" => vec![arith_offset(args, destination, call_instance, ctx)],
        "breakpoint" => vec![breakpoint(args)],
        "cold_path" | "assert_inhabited" | "assert_zero_valid" | "const_deallocate" => {
            vec![V1Root::Nop]
        }
        "black_box" => vec![black_box(args, destination, call_instance, ctx)],
        "caller_location" => vec![caller_location(destination, ctx, span)],
        "compare_bytes" => vec![place_set(
            destination,
            compare_bytes(
                handle_operand(&args[0].node, ctx),
                handle_operand(&args[1].node, ctx),
                handle_operand(&args[2].node, ctx),
                ctx,
            ),
            ctx,
        )],
        "ctpop" => vec![ints::ctpop(args, destination, call_instance, ctx)],
        "bitreverse" => vec![ints::bitreverse(args, destination, ctx, call_instance)],
        "ctlz" | "ctlz_nonzero" => vec![ctlz(args, destination, call_instance, ctx)],
        "unlikely" | "likely" => {
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `{fn_name}` MUST take in exactly 1 argument!"
            );
            // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
            vec![place_set(
                destination,
                handle_operand(&args[0].node, ctx),
                ctx,
            )]
        }
        "is_val_statically_known" => vec![is_val_statically_known(args, destination, ctx)],
        "needs_drop" => {
            debug_assert_eq!(
                args.len(),
                0,
                "The intrinsic `needs_drop` MUST take in exactly 0 argument!"
            );
            let needs_drop = ctx
                .monomorphize(
                    call_instance.args[0]
                        .as_type()
                        .expect("needs_drop works only on types!"),
                )
                .needs_drop(
                    ctx.tcx(),
                    rustc_middle::ty::TypingEnv::fully_monomorphized(),
                );
            let needs_drop = i32::from(needs_drop);
            vec![place_set(
                destination,
                V1Node::V2(ctx.alloc_node(needs_drop)),
                ctx,
            )]
        }
        "disjoint_bitor" => {
            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let ty = args[0].node.ty(ctx.body(), ctx.tcx());
            vec![place_set(
                destination,
                crate::binop::bitop::bit_or_unchecked(ty, ty, ctx, lhs, rhs),
                ctx,
            )]
        }
        "fmaf32" => vec![fmaf32(args, destination, ctx)],
        "fmaf64" => vec![fmaf64(args, destination, ctx)],
        "raw_eq" => vec![raw_eq(args, destination, call_instance, ctx)],
        "bswap" => vec![bswap::bswap(args, destination, ctx)],
        "cttz" | "cttz_nonzero" => vec![ints::cttz(args, destination, ctx, call_instance)],
        "rotate_left" => vec![rotate_left(args, destination, ctx, call_instance)],
        "write_bytes" => vec![write_bytes(args, call_instance, ctx)],
        "copy" => vec![copy(args, call_instance, ctx)],
        "exact_div" => {
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `exact_div` MUST take in exactly 2 argument!"
            );

            vec![place_set(
                destination,
                crate::binop::binop(
                    rustc_middle::mir::BinOp::Div,
                    &args[0].node,
                    &args[1].node,
                    ctx,
                ),
                ctx,
            )]
        }
        "type_id" => vec![tpe::type_id(destination, call_instance, ctx)],
        "atomic_load_acquire"
        | "atomic_load_seqcst"
        | "atomic_load_unordered"
        | "volatile_load" => vec![volitale_load(args, destination, ctx)],
        "volatile_store" => {
            let pointed_type = ctx.monomorphize(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let addr_calc = handle_operand(&args[0].node, ctx);
            let value_calc = handle_operand(&args[1].node, ctx);
            vec![V1Root::Volatile(Box::new(ptr_set_op(
                pointed_type.into(),
                ctx,
                addr_calc,
                value_calc,
            )))]
        }
        "atomic_store" => {
            // TODO: implement a proper atomic store.
            debug_assert_eq!(
                args.len(),
                2,
                "The intrinsic `{fn_name}` MUST take in exactly 1 argument!"
            );
            let addr = handle_operand(&args[0].node, ctx);
            let val = handle_operand(&args[1].node, ctx);
            let arg_ty = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));

            vec![ptr_set_op(arg_ty.into(), ctx, addr, val)]
        }
        "atomic_cxchg" | "atomic_cxchgweak" => atomic::cxchg(args, destination, ctx).into(),
        "atomic_xsub" => {
            // *T
            let dst = handle_operand(&args[0].node, ctx);
            // T
            let sub_amount = handle_operand(&args[1].node, ctx);
            // we sub by adding a negative number

            let src_type = ctx.monomorphize(args[1].node.ty(ctx.body(), ctx.tcx()));
            let src_type = ctx.type_from_cache(src_type);
            match src_type {
                Type::Int(int) => {
                    let add_amount = if int.is_signed() {
                        V1Node::Neg(Box::new(sub_amount.clone()))
                    } else {
                        crate::casts::int_to_int(
                            Type::Int(int.as_signed()),
                            src_type,
                            V1Node::Neg(Box::new(crate::casts::int_to_int(
                                src_type,
                                Type::Int(int.as_signed()),
                                sub_amount.clone(),
                                ctx,
                            ))),
                            ctx,
                        )
                    };
                    vec![place_set(
                        destination,
                        atomic_add(dst, add_amount.clone(), src_type, ctx),
                        ctx,
                    )]
                }
                Type::Ptr(_) => {
                    let add_amount = crate::casts::int_to_int(
                        Type::Int(Int::ISize),
                        Type::Int(Int::USize),
                        V1Node::Neg(Box::new(sub_amount.cast_ptr(Type::Int(Int::ISize)))),
                        ctx,
                    );
                    vec![place_set(
                        destination,
                        atomic_add(dst, add_amount.clone(), src_type, ctx).cast_ptr(src_type),
                        ctx,
                    )]
                }
                _ => panic!("{src_type:?} is not an int."),
            }
        }
        "atomic_or" => call_atomic(args, destination, ctx, atomic_or),
        "atomic_xor" => call_atomic(args, destination, ctx, atomic_xor),
        "atomic_and" => call_atomic(args, destination, ctx, atomic_and),
        "atomic_nand" => call_atomic(args, destination, ctx, atomic_nand),
        "atomic_fence" => {
            let thread = ClassRef::thread(ctx);
            let fence = MethodRef::new(
                thread,
                ctx.alloc_string("MemoryBarrier"),
                ctx.sig([], Type::Void),
                MethodKind::Static,
                vec![].into(),
            );
            vec![V1Root::Call {
                site: ctx.alloc_methodref(fence),
                args: [].into(),
            }]
        }
        "atomic_xadd" => call_atomic(args, destination, ctx, atomic_add),
        "atomic_umin" => call_atomic(args, destination, ctx, atomic_min),
        "atomic_umax" => call_atomic(args, destination, ctx, atomic_max),
        "atomic_xchg" => vec![atomic::xchg(args, destination, ctx)],
        // TODO: ensure those intrinsics are sound in C. perhaps time for a new cillyIR node?
        "ptr_offset_from_unsigned" => {
            vec![ptr::ptr_offset_from_unsigned(
                args,
                destination,
                call_instance,
                ctx,
            )]
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

            vec![place_set(
                destination,
                V1Node::And(
                    Box::new(handle_operand(&args[0].node, ctx).cast_ptr(Type::Int(Int::USize))),
                    Box::new(handle_operand(&args[1].node, ctx)),
                )
                .cast_ptr(tpe),
                ctx,
            )]
        }
        "ptr_offset_from" => vec![ptr::ptr_offset_from(args, destination, call_instance, ctx)],
        "saturating_add" => vec![saturating_add(args, destination, ctx, call_instance)],
        "saturating_sub" => vec![saturating_sub(args, destination, ctx, call_instance)],
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
            let align = rustc_codegen_clr_type::align_of(tpe, ctx.tcx());
            vec![place_set(
                destination,
                conv_usize!(V1Node::V2(ctx.alloc_node(align))),
                ctx,
            )]
        }
        // .NET guarantees all loads are tear-free. TODO: what bout C?
        "atomic_load" => {
            //I am not sure this is implemented properly
            debug_assert_eq!(
                args.len(),
                1,
                "The intrinsic `atomic_load_relaxed` MUST take in exactly 1 argument!"
            );
            let ops = handle_operand(&args[0].node, ctx);
            let arg = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
            let arg_ty = arg.builtin_deref(true).unwrap();
            let arg_type = ctx.type_from_cache(arg_ty);
            let ops = V1Node::LdObj {
                ptr: Box::new(ops),
                obj: Box::new(arg_type),
            }; //;deref_op(arg_ty.into(), ctx, ops);
            vec![place_set(destination, ops, ctx)]
        }
        "sqrtf32" => float_unop(args, destination, ctx, Float::F32, "Sqrt"),
        "carrying_mul_add"
            if !matches!(
                ctx.type_from_cache(
                    call_instance.args[1]
                        .as_type()
                        .expect("needs_drop works only on types!"),
                )
                .as_int()
                .expect("carrying_mul_add with a non-int type"),
                Int::U128 | Int::I128
            ) =>
        {
            let wrapping = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let wint = wrapping
                .as_int()
                .expect("carrying_mul_add with a non-int type");

            let overflow = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("needs_drop works only on types!"),
            );
            let oint = overflow
                .as_int()
                .expect("carrying_mul_add with a non-int type");
            let promoted = oint
                .promoted()
                .expect("Can't carrying_mul_add cause type is too large");
            let mul_a = casts::int_to_int(
                cilly::Type::Int(oint),
                cilly::Type::Int(promoted),
                handle_operand(&args[0].node, ctx),
                ctx,
            );
            let mul_b = casts::int_to_int(
                cilly::Type::Int(oint),
                cilly::Type::Int(promoted),
                handle_operand(&args[1].node, ctx),
                ctx,
            );
            let carry = casts::int_to_int(
                cilly::Type::Int(oint),
                cilly::Type::Int(promoted),
                handle_operand(&args[2].node, ctx),
                ctx,
            );
            let addend = casts::int_to_int(
                cilly::Type::Int(oint),
                cilly::Type::Int(promoted),
                handle_operand(&args[3].node, ctx),
                ctx,
            );
            let sum = if promoted.size() == Some(16) {
                let main_module = ctx.main_module();
                let op_mul = MethodRef::new(
                    *main_module,
                    ctx.alloc_string(format!("mul_{}", promoted.name())),
                    ctx.sig(
                        [Type::Int(promoted), Type::Int(promoted)],
                        Type::Int(promoted),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                let op_add = MethodRef::new(
                    *main_module,
                    ctx.alloc_string(format!("add_{}", promoted.name())),
                    ctx.sig(
                        [Type::Int(promoted), Type::Int(promoted)],
                        Type::Int(promoted),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                let op_mul = ctx.alloc_methodref(op_mul);
                let op_add = ctx.alloc_methodref(op_add);
                call!(
                    op_add,
                    [
                        call!(op_mul, [mul_a, mul_b]),
                        call!(op_add, [carry, addend])
                    ]
                )
            } else {
                (mul_a * mul_b) + carry + addend
            };
            let ovf = if promoted.size() == Some(16) {
                let main_module = ctx.main_module();
                let op_div = MethodRef::new(
                    *main_module,
                    ctx.alloc_string(format!("div_{}", promoted.name())),
                    ctx.sig(
                        [Type::Int(promoted), Type::Int(promoted)],
                        Type::Int(promoted),
                    ),
                    MethodKind::Static,
                    vec![].into(),
                );
                let op_div = ctx.alloc_methodref(op_div);
                call!(
                    op_div,
                    [
                        sum.clone(),
                        (casts::int_to_int(
                            cilly::Type::Int(Int::U128),
                            cilly::Type::Int(promoted),
                            V1Node::V2(ctx.alloc_node(1_u128 << (oint.size().unwrap_or(8) * 8))),
                            ctx,
                        ))
                    ]
                )
            } else {
                V1Node::DivUn(
                    Box::new(sum.clone()),
                    Box::new(casts::int_to_int(
                        cilly::Type::Int(Int::U64),
                        cilly::Type::Int(promoted),
                        V1Node::V2(ctx.alloc_node(1_u64 << (oint.size().unwrap_or(8) * 8))),
                        ctx,
                    )),
                )
            };

            let ovf =
                casts::int_to_int(cilly::Type::Int(promoted), cilly::Type::Int(wint), ovf, ctx);
            let wr =
                casts::int_to_int(cilly::Type::Int(promoted), cilly::Type::Int(oint), sum, ctx);
            let res_tpe = ctx
                .type_from_cache(destination.ty(ctx.body(), ctx.tcx()).ty)
                .as_class_ref()
                .unwrap();
            let dst = place_address(destination, ctx);
            let item1 = ctx.alloc_string("Item1");
            let item2 = ctx.alloc_string("Item2");
            vec![
                V1Root::SetField {
                    addr: Box::new(dst.clone()),
                    value: Box::new(wr),
                    desc: ctx.alloc_field(FieldDesc::new(res_tpe, item1, cilly::Type::Int(oint))),
                },
                V1Root::SetField {
                    addr: Box::new(dst),
                    value: Box::new(ovf),
                    desc: ctx.alloc_field(FieldDesc::new(res_tpe, item2, cilly::Type::Int(wint))),
                },
            ]
        }
        "powif32" => vec![powif32(args, destination, ctx)],
        "powif64" => vec![powif64(args, destination, ctx)],
        "size_of_val" => vec![size_of_val(args, destination, ctx, call_instance)],
        "typed_swap_nonoverlapping" => {
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
            vec![V1Root::Call {
                site: ctx.alloc_methodref(generic),
                args: [
                    handle_operand(&args[0].node, ctx).cast_ptr(void_ptr),
                    handle_operand(&args[1].node, ctx).cast_ptr(void_ptr),
                    conv_usize!(V1Node::V2(ctx.size_of(tpe).into_idx(ctx))),
                ]
                .into(),
            }]
        }
        "type_name" => {
            let const_val = ctx
                .tcx()
                .const_eval_instance(
                    rustc_middle::ty::TypingEnv::fully_monomorphized(),
                    call_instance,
                    span,
                )
                .unwrap();
            vec![place_set(
                destination,
                load_const_value(const_val, Ty::new_static_str(ctx.tcx()), ctx),
                ctx,
            )]
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
            vec![place_set(
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
            )]
        }
        "fabsf32" => float_unop(args, destination, ctx, Float::F32, "Abs"),
        "fabsf64" => float_unop(args, destination, ctx, Float::F64, "Abs"),
        "expf32" => float_unop(args, destination, ctx, Float::F32, "Exp"),
        "expf64" => float_unop(args, destination, ctx, Float::F64, "Exp"),
        "logf32" => float_unop(args, destination, ctx, Float::F32, "Log"),
        "logf64" => float_unop(args, destination, ctx, Float::F64, "Log"),
        "log2f32" => float_unop(args, destination, ctx, Float::F32, "Log2"),
        "log2f64" => float_unop(args, destination, ctx, Float::F64, "Log2"),
        "log10f32" => float_unop(args, destination, ctx, Float::F32, "Log10"),
        "log10f64" => float_unop(args, destination, ctx, Float::F64, "Log10"),
        "powf32" => vec![powf32(args, destination, ctx)],
        "powf64" => vec![powf64(args, destination, ctx)],
        "copysignf32" => float_binop(args, destination, ctx, Float::F32, "CopySign"),
        "copysignf64" => float_binop(args, destination, ctx, Float::F64, "CopySign"),
        "copysignf128" => {
            let main_module = ctx.main_module();
            let log = MethodRef::new(
                *main_module,
                ctx.alloc_string("copysignf128"),
                ctx.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Float::F128,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            vec![place_set(
                destination,
                call!(
                    ctx.alloc_methodref(log),
                    [
                        handle_operand(&args[0].node, ctx),
                        handle_operand(&args[1].node, ctx)
                    ]
                ),
                ctx,
            )]
        }
        "sinf32" => float_unop(args, destination, ctx, Float::F32, "Sin"),
        "sinf64" => float_unop(args, destination, ctx, Float::F64, "Sin"),
        "cosf32" => float_unop(args, destination, ctx, Float::F32, "Cos"),
        "cosf64" => float_unop(args, destination, ctx, Float::F64, "Cos"),
        "exp2f32" => float_unop(args, destination, ctx, Float::F32, "Exp2"),
        "exp2f64" => float_unop(args, destination, ctx, Float::F64, "Exp2"),
        "truncf32" => float_unop(args, destination, ctx, Float::F32, "Truncate"),
        "truncf64" => float_unop(args, destination, ctx, Float::F64, "Truncate"),
        // `roundf32` should be a differnt intrinsics, but it requires some .NET fuckery to implement(.NET enums are **wierd**)
        "nearbyintf32" | "rintf32" | "round_ties_even_f32" => {
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
            vec![place_set(destination, value_calc, ctx)]
        }
        "roundf32" => vec![roundf32(args, destination, ctx)],
        "roundf64" => vec![roundf64(args, destination, ctx)],
        "nearbyintf64" | "rintf64" | "round_ties_even_f64" => {
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
            vec![place_set(destination, value_calc, ctx)]
        }
        "floorf32" => float_unop(args, destination, ctx, Float::F32, "Floor"),
        "floorf64" => float_unop(args, destination, ctx, Float::F64, "Floor"),
        "ceilf32" => float_unop(args, destination, ctx, Float::F32, "Ceiling"),
        "ceilf64" => float_unop(args, destination, ctx, Float::F64, "Ceiling"),
        "maxnumf64" => float_binop(args, destination, ctx, Float::F64, "MaxNumber"),
        "maxnumf32" => float_binop(args, destination, ctx, Float::F32, "MaxNumber"),
        "minnumf64" => float_binop(args, destination, ctx, Float::F64, "MinNumber"),
        "minnumf32" => float_binop(args, destination, ctx, Float::F32, "MinNumber"),
        "variant_count" => {
            let const_val = ctx
                .tcx()
                .const_eval_instance(
                    rustc_middle::ty::TypingEnv::fully_monomorphized(),
                    call_instance,
                    span,
                )
                .unwrap();
            vec![place_set(
                destination,
                load_const_value(const_val, Ty::new_uint(ctx.tcx(), UintTy::Usize), ctx),
                ctx,
            )]
        }
        "sqrtf64" => float_unop(args, destination, ctx, Float::F64, "Sqrt"),
        "rotate_right" => vec![rotate_right(args, destination, ctx, call_instance)],
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
            vec![place_set(
                destination,
                call!(
                    ctx.alloc_methodref(catch_unwind),
                    [try_fn, data_ptr, catch_fn]
                ),
                ctx,
            )]
        }
        "abort" => vec![V1Root::throw("Called abort!", ctx)],
        "const_allocate" => {
            let null = ctx.alloc_node(Const::USize(0));
            let null = ctx.cast_ptr(null, Int::U8);
            vec![place_set(destination, V1Node::V2(null), ctx)]
        }
        "vtable_size" => vec![vtable::vtable_size(args, destination, ctx)],
        "vtable_align" => vec![vtable::vtable_align(args, destination, ctx)],
        "simd_eq" => {
            let comparands = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let result = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_eq");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[comparands, comparands], result, name, ctx);
            vec![place_set(destination, call!(eq, [lhs, rhs]), ctx)]
        }
        "simd_lt" => {
            let comparands = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_lt works only on types!"),
            );
            let result = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("simd_lt works only on types!"),
            );
            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_lt");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[comparands, comparands], result, name, ctx);
            vec![place_set(destination, call!(eq, [lhs, rhs]), ctx)]
        }
        "simd_gt" => {
            let comparands = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_gt works only on types!"),
            );
            let result = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("simd_gt works only on types!"),
            );
            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_gt");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[comparands, comparands], result, name, ctx);
            vec![place_set(destination, call!(eq, [lhs, rhs]), ctx)]
        }
        "simd_or" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_or works only on types!"),
            );

            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_or");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[vec, vec], vec, name, ctx);
            vec![place_set(destination, call!(eq, [lhs, rhs]), ctx)]
        }
        "simd_add" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_add works only on types!"),
            );

            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_add");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[vec, vec], vec, name, ctx);
            vec![place_set(destination, call!(eq, [lhs, rhs]), ctx)]
        }
        "simd_and" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_and works only on types!"),
            );

            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_and");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[vec, vec], vec, name, ctx);
            vec![place_set(destination, call!(eq, [lhs, rhs]), ctx)]
        }
        "simd_sub" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_sub works only on types!"),
            );

            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_sub");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let sub = main_module.static_mref(&[vec, vec], vec, name, ctx);
            vec![place_set(destination, call!(sub, [lhs, rhs]), ctx)]
        }

        "simd_mul" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_mul works only on types!"),
            );

            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let name = ctx.alloc_string("simd_mul");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let mul = main_module.static_mref(&[vec, vec], vec, name, ctx);
            vec![place_set(destination, call!(mul, [lhs, rhs]), ctx)]
        }
        "simd_fabs" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_fabs works only on types!"),
            );

            let lhs = handle_operand(&args[0].node, ctx);
            let name = ctx.alloc_string("simd_abs");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let abs = main_module.static_mref(&[vec], vec, name, ctx);
            vec![place_set(destination, call!(abs, [lhs,]), ctx)]
        }
        "simd_bitmask" => {
            let vec: Type = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_bitmask works only on types!"),
            );
            let int = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("simd_bitmask works only on types!"),
            );
            let int = int
                .as_int()
                .expect("simd_bitmask only currently supports bitpacking ints.");
            let lhs = handle_operand(&args[0].node, ctx);
            let name = ctx.alloc_string("simd_get_most_significant_bits");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let most_significant_bits = main_module.static_mref(&[vec], Type::Int(int), name, ctx);
            vec![place_set(
                destination,
                call!(most_significant_bits, [lhs]),
                ctx,
            )]
        }
        "simd_neg" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_neg works only on types!"),
            );
            let val = handle_operand(&args[0].node, ctx);
            let name = ctx.alloc_string("simd_neg");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[vec], vec, name, ctx);
            vec![place_set(destination, call!(eq, [val]), ctx)]
        }
        "simd_shuffle" => {
            let t_type = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let u_type = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let v_type = ctx.type_from_cache(
                call_instance.args[2]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let x = handle_operand(&args[0].node, ctx);
            let y = handle_operand(&args[1].node, ctx);
            // When the two vectors provided to simd shuffles are always the same, and have a length of 1(are scalar), the shuffle is equivalent to creating a vector [scalar,scalar].
            if x == y && matches!(t_type, Type::Int(_) | Type::Float(_)) {
                let name = ctx.alloc_string("simd_vec_from_val");
                let main_module = ctx.main_module();
                let main_module = ctx[*main_module].clone();
                let shuffle = main_module.static_mref(&[t_type], v_type, name, ctx);
                // SANITY: for this optimzation to work, the u(index vector) and v(result vector) both have to have be vectors.
                let (_u_type, _v_type) = (
                    u_type.as_simdvector().unwrap(),
                    v_type.as_simdvector().unwrap(),
                );
                return vec![place_set(destination, call!(shuffle, [x]), ctx)];
            }
            let idx = handle_operand(&args[2].node, ctx);
            let name = ctx.alloc_string("simd_shuffle");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let shuffle = main_module.static_mref(&[t_type, t_type, u_type], v_type, name, ctx);
            vec![place_set(destination, call!(shuffle, [x, y, idx]), ctx)]
        }
        "simd_ne" => {
            let comparands = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let result = ctx.type_from_cache(
                call_instance.args[1]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let lhs = handle_operand(&args[0].node, ctx);
            let rhs = handle_operand(&args[1].node, ctx);
            let eq = ctx.alloc_string("simd_eq");
            let ones_compliment = ctx.alloc_string("simd_ones_compliment");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[comparands, comparands], result, eq, ctx);
            let eq = call!(eq, [lhs, rhs]);
            let ones_compliment = main_module.static_mref(&[result], result, ones_compliment, ctx);
            let ne = call!(ones_compliment, [eq]);
            vec![place_set(destination, ne, ctx)]
        }
        "simd_reduce_any" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let x = handle_operand(&args[0].node, ctx);
            let simd_eq = ctx.alloc_string("simd_eq_any");
            let allset = ctx.alloc_string("simd_allset");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[vec, vec], Type::Bool, simd_eq, ctx);
            let allset = main_module.static_mref(&[], vec, allset, ctx);
            let allset = call!(allset, []);
            vec![place_set(destination, call!(eq, [x, allset]), ctx)]
        }
        "select_unpredictable" => {
            let tpe = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("select_unpredictable works only on types!"),
            );
            let cond = handle_operand(&args[0].node, ctx);

            if let Some(_) = tpe.as_class_ref() {
                let true_val = operand_address(&args[1].node, ctx);
                let false_val = operand_address(&args[2].node, ctx);
                let select = V1Node::select(ctx.nptr(tpe), true_val, false_val, cond, ctx);
                let select = V1Node::LdObj {
                    ptr: Box::new(select),
                    obj: Box::new(tpe),
                };
                return vec![place_set(destination, select, ctx)];
            }
            let true_val = handle_operand(&args[1].node, ctx);
            let false_val = handle_operand(&args[2].node, ctx);
            let select = V1Node::select(tpe, true_val, false_val, cond, ctx);
            vec![place_set(destination, select, ctx)]
        }
        "simd_reduce_all" => {
            let vec = ctx.type_from_cache(
                call_instance.args[0]
                    .as_type()
                    .expect("simd_eq works only on types!"),
            );
            let x = handle_operand(&args[0].node, ctx);
            let simd_eq = ctx.alloc_string("simd_eq_all");
            let allset = ctx.alloc_string("simd_allset");
            let main_module = ctx.main_module();
            let main_module = ctx[*main_module].clone();
            let eq = main_module.static_mref(&[vec, vec], Type::Bool, simd_eq, ctx);
            let allset = main_module.static_mref(&[], vec, allset, ctx);
            let allset = call!(allset, []);
            vec![place_set(destination, call!(eq, [x, allset]), ctx)]
        }
        _ => intrinsic_slow(fn_name, args, destination, ctx, call_instance, span),
    }
}
use rustc_middle::span_bug;
fn intrinsic_slow<'tcx>(
    fn_name: &str,
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    call_instance: Instance<'tcx>,
    span: rustc_span::Span,
) -> Vec<V1Root> {
    // Then, demangle the type name, converting it to a Rust-style one (eg. `core::option::Option::h8zc8s`)
    let demangled = rustc_demangle::demangle(fn_name);
    // Using formating preserves the generic hash.
    let demangled = format!("{demangled:#}");
    if demangled == fn_name {
        let intrinsic = ctx.tcx().intrinsic(call_instance.def_id()).unwrap();
        if intrinsic.must_be_overridden {
            span_bug!(
                span,
                "intrinsic {} must be overridden by rustc_codgen_clr, but isn't",
                intrinsic.name,
            );
        }
        super::call::call_inner(
            Instance::new_raw(call_instance.def_id(), call_instance.args)
                .ty(ctx.tcx(), TypingEnv::fully_monomorphized()),
            Instance::new_raw(call_instance.def_id(), call_instance.args),
            ctx,
            args,
            destination,
            span,
        )
    } else {
        assert!(demangled.contains("::"));
        let striped = demangled_to_stem(&demangled);
        handle_intrinsic(striped, args, destination, call_instance, span, ctx)
    }
}
fn volitale_load<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> V1Root {
    //TODO:fix volitale prefix!
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `volatile_load` MUST take in exactly 1 argument!"
    );
    let arg = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
    let arg_ty = arg.builtin_deref(true).unwrap();
    let arg_type = ctx.type_from_cache(arg_ty);
    let arg = handle_operand(&args[0].node, ctx);
    let ops = V1Node::Volatile(Box::new(V1Node::LdObj {
        ptr: Box::new(arg),
        obj: Box::new(arg_type),
    }));
    place_set(destination, ops, ctx)
}
fn caller_location<'tcx>(
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    span: rustc_span::Span,
) -> V1Root {
    let caller_loc = ctx.tcx().span_as_caller_location(span);
    let caller_loc_ty = ctx.tcx().caller_location_ty();
    place_set(
        destination,
        load_const_value(caller_loc, caller_loc_ty, ctx),
        ctx,
    )
}
fn demangled_to_stem(s: &str) -> &str {
    let mut res = None;
    //.filter(|part|!(part.contains('<') | part.contains('>'))).last().unwrap()
    for element in s.split("::") {
        if element.contains('<') {
            break;
        }
        res = Some(element);
    }
    res.unwrap()
}
fn float_unop<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    float: Float,
    name: &str,
) -> Vec<V1Root> {
    let log = MethodRef::new(
        float.class(ctx),
        ctx.alloc_string(name),
        ctx.sig([Type::Float(float)], float),
        MethodKind::Static,
        vec![].into(),
    );
    vec![place_set(
        destination,
        call!(
            ctx.alloc_methodref(log),
            [handle_operand(&args[0].node, ctx),]
        ),
        ctx,
    )]
}
fn float_binop<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    float: Float,
    name: &str,
) -> Vec<V1Root> {
    let log = MethodRef::new(
        float.class(ctx),
        ctx.alloc_string(name),
        ctx.sig([Type::Float(float), Type::Float(float)], float),
        MethodKind::Static,
        vec![].into(),
    );
    vec![place_set(
        destination,
        call!(
            ctx.alloc_methodref(log),
            [
                handle_operand(&args[0].node, ctx),
                handle_operand(&args[1].node, ctx)
            ]
        ),
        ctx,
    )]
}
#[test]
fn test_intrinsic_slow_escape() {
    const BAD:&str = "core::intrinsics::ptr_offset_from_unsigned::<(rustc_hir::def::LifetimeRes, rustc_resolve::late::diagnostics::LifetimeElisionCandidate)";
    assert_eq!(demangled_to_stem(BAD), "ptr_offset_from_unsigned");
}
