use crate::{assembly::MethodCompileCtx, place::place_set, r#type::pointer_to_is_fat};
use cilly::{
    cil_node::CILNode, cil_root::CILRoot, conv_usize, field_desc::FieldDescriptor, ld_field,
    ldc_u32, ptr, size_of, ClassRef, Type,
};
use rustc_middle::{
    mir::{Operand, Place},
    ty::{Instance, TyKind},
};
use rustc_span::source_map::Spanned;
pub fn is_val_statically_known<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `is_val_statically_known` MUST take in exactly 1 argument!"
    );
    // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
    place_set(destination, CILNode::LdFalse, ctx)
}
pub fn size_of_val<'tcx>(
    args: &[Spanned<Operand<'tcx>>],
    destination: &Place<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_, '_>,
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
    if crate::utilis::is_zst(pointed_ty, ctx.tcx()) {
        return place_set(destination, conv_usize!(ldc_u32!(0)), ctx);
    }
    if pointer_to_is_fat(pointed_ty, ctx.tcx(), ctx.instance()) {
        let ptr_ty = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tcx()));
        match pointed_ty.kind() {
            TyKind::Str => {
                let slice_tpe: ClassRef = ctx.type_from_cache(ptr_ty).as_class_ref().unwrap();
                let descriptor =
                    FieldDescriptor::new(slice_tpe, Type::Int(Int::USize), crate::METADATA.into());
                let addr = crate::operand::operand_address(&args[0].node, ctx);
                return place_set(destination, ld_field!(addr, descriptor), ctx);
            }
            TyKind::Slice(inner) => {
                let slice_tpe: ClassRef = ctx.type_from_cache(ptr_ty).as_class_ref().unwrap();
                let inner = ctx.monomorphize(*inner);
                let inner_type = ctx.type_from_cache(inner);
                let descriptor =
                    FieldDescriptor::new(slice_tpe, Type::Int(Int::USize), crate::METADATA.into());
                let addr = crate::operand::operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    ld_field!(addr, descriptor) * conv_usize!(size_of!(inner_type)),
                    ctx,
                );
            }
            // WARNING: ASSUMES ANY NON-SLICE DST IS A DYN.
            _ => {
                let slice_tpe: ClassRef = ctx.type_from_cache(ptr_ty).as_class_ref().unwrap();

                let descriptor =
                    FieldDescriptor::new(slice_tpe, Type::Int(Int::USize), crate::METADATA.into());
                let addr = crate::operand::operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    CILNode::LDIndUSize {
                        ptr: Box::new(
                            ld_field!(addr, descriptor).cast_ptr(ptr!(Type::Int(Int::USize)))
                                + conv_usize!((size_of!(Type::Int(Int::ISize)))),
                        ),
                    },
                    ctx,
                );
            }
        }
    }
    let tpe = ctx.monomorphize(pointed_ty);
    let tpe = ctx.type_from_cache(tpe);
    place_set(destination, conv_usize!(size_of!(tpe)), ctx)
}
