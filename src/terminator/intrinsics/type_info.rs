use crate::assembly::MethodCompileCtx;
use cilly::{
    cil_node::V1Node,
    cil_root::CILRoot,
    conv_usize, ld_field, Const, IntoAsmIndex, Type, {FieldDesc, Int},
};
use rustc_codegen_clr_place::place_set;
use rustc_codegen_clr_type::{
    utilis::{is_zst, pointer_to_is_fat},
    GetTypeExt,
};
use rustc_codgen_clr_operand::operand_address;
use rustc_middle::{
    mir::{Operand, Place},
    ty::{Instance, TyKind},
};
use rustc_span::source_map::Spanned;
pub fn is_val_statically_known<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `is_val_statically_known` MUST take in exactly 1 argument!"
    );
    // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
    place_set(destination, V1Node::V2(ctx.alloc_node(false)), ctx)
}
pub fn size_of_val<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    call_instance: Instance<'tcx>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `size_of_val` MUST take in exactly 1 argument!"
    );

    let pointed_ty = ctx.monomorphize(
        call_instance.args[0]
            .as_type()
            .expect("needs_drop works only on types!"),
    );
    if is_zst(pointed_ty, ctx.tcx()) {
        return place_set(
            destination,
            V1Node::V2(ctx.alloc_node(Const::USize(0))),
            ctx,
        );
    }
    if pointer_to_is_fat(pointed_ty, ctx.tcx(), ctx.instance()) {
        let ptr_ty = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
        match pointed_ty.kind() {
            TyKind::Str => {
                let slice_tpe = ctx.type_from_cache(ptr_ty).as_class_ref().unwrap();
                let descriptor = FieldDesc::new(
                    slice_tpe,
                    ctx.alloc_string(crate::METADATA),
                    Type::Int(Int::USize),
                );
                let addr = operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    ld_field!(addr, ctx.alloc_field(descriptor)),
                    ctx,
                );
            }
            TyKind::Slice(inner) => {
                let slice_tpe = ctx.type_from_cache(ptr_ty).as_class_ref().unwrap();
                let inner = ctx.monomorphize(*inner);
                let inner_type = ctx.type_from_cache(inner);
                let descriptor = FieldDesc::new(
                    slice_tpe,
                    ctx.alloc_string(crate::METADATA),
                    Type::Int(Int::USize),
                );
                let addr = operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    ld_field!(addr, ctx.alloc_field(descriptor))
                        * conv_usize!(V1Node::V2(ctx.size_of(inner_type).into_idx(ctx))),
                    ctx,
                );
            }
            // WARNING: ASSUMES ANY NON-SLICE DST IS A DYN.
            _ => {
                let slice_tpe = ctx.type_from_cache(ptr_ty).as_class_ref().unwrap();

                let descriptor = FieldDesc::new(
                    slice_tpe,
                    ctx.alloc_string(crate::METADATA),
                    Type::Int(Int::USize),
                );
                let addr = operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    V1Node::LDIndUSize {
                        ptr: Box::new(
                            ld_field!(addr, ctx.alloc_field(descriptor))
                                .cast_ptr(ctx.nptr(Type::Int(Int::USize)))
                                + conv_usize!(V1Node::V2(ctx.size_of(Int::ISize).into_idx(ctx))),
                        ),
                    },
                    ctx,
                );
            }
        }
    }
    let tpe = ctx.monomorphize(pointed_ty);
    let tpe = ctx.type_from_cache(tpe);
    place_set(
        destination,
        conv_usize!(V1Node::V2(ctx.size_of(tpe).into_idx(ctx))),
        ctx,
    )
}
