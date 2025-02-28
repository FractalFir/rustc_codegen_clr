use crate::{assembly::MethodCompileCtx, operand::handle_operand};
use cilly::{
    call,
    cil_node::CILNode,
    cil_root::CILRoot,
    cilnode::MethodKind,
    conv_f32, conv_f64,
    v2::{ClassRef, Float},
    Int, MethodRef, Type,
};
use rustc_codegen_clr_place::place_set;
use rustc_middle::mir::{Operand, Place};
use rustc_span::source_map::Spanned;
/// Implementation of the fmaf32 intrinsics. Takes in 3 arguments: a, b, c. Calcualtes a * b + c
pub fn fmaf32<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
/// Implementation of the fmaf64 intrinsics. Takes in 3 arguments: a, b, c. Calcualtes a * b + c
pub fn fmaf64<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
pub fn powif32<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
pub fn powif64<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
pub fn powf32<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
pub fn powf64<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
                handle_operand(&args[1].node, ctx),
            ]
        ),
        ctx,
    )
}
pub fn roundf32<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
            CILNode::V2(ctx.alloc_node(1_i32)).transmute_on_stack(
                Type::Int(Int::I32),
                Type::ClassRef(rounding),
                ctx
            )
        ]
    );
    place_set(destination, value_calc, ctx)
}
pub fn roundf64<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,

    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
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
            CILNode::V2(ctx.alloc_node(1_i32)).transmute_on_stack(
                Type::Int(Int::I32),
                Type::ClassRef(rounding),
                ctx
            )
        ]
    );
    place_set(destination, value_calc, ctx)
}
