use crate::{assembly::MethodCompileCtx, place::place_set, r#type::pointer_to_is_fat};
use cilly::{
    cil_node::CILNode, cil_root::CILRoot, conv_usize, field_desc::FieldDescriptor, ld_field,
    ldc_u32, size_of, DotnetTypeRef, Type,
};
use rustc_middle::{
    mir::{Operand, Place},
    ty::{Instance, TyKind},
};
use rustc_span::source_map::Spanned;
pub fn is_val_statically_known<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    ctx: &mut MethodCompileCtx<'tyctx, '_, '_>,
) -> CILRoot {
    debug_assert_eq!(
        args.len(),
        1,
        "The intrinsic `is_val_statically_known` MUST take in exactly 1 argument!"
    );
    // assert_eq!(args.len(),1,"The intrinsic `unlikely` MUST take in exactly 1 argument!");
    place_set(destination, CILNode::LdFalse, ctx)
}
pub fn size_of_val<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    ctx: &mut MethodCompileCtx<'tyctx, '_, '_>,
    call_instance: Instance<'tyctx>,
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
    if crate::utilis::is_zst(pointed_ty, ctx.tyctx()) {
        return place_set(destination, conv_usize!(ldc_u32!(0)), ctx);
    }
    if pointer_to_is_fat(pointed_ty, ctx.tyctx(), ctx.instance()) {
        let ptr_ty = ctx.monomorphize(args[0].node.ty(ctx.body(), ctx.tyctx()));
        match pointed_ty.kind() {
            TyKind::Slice(inner) => {
                let slice_tpe: DotnetTypeRef = ctx.type_from_cache(ptr_ty).as_dotnet().unwrap();
                let inner = ctx.monomorphize(*inner);
                let inner_type = ctx.type_from_cache(inner);
                let descriptor = FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
                let addr = crate::operand::operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    ld_field!(addr, descriptor) * conv_usize!(size_of!(inner_type)),
                    ctx,
                );
            }
            TyKind::Dynamic(_, _, _) => {
                let slice_tpe: DotnetTypeRef = ctx.type_from_cache(ptr_ty).as_dotnet().unwrap();

                let descriptor = FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
                let addr = crate::operand::operand_address(&args[0].node, ctx);
                return place_set(
                    destination,
                    CILNode::LDIndUSize {
                        ptr: Box::new(
                            CILNode::TransmutePtr {
                                val: Box::new(ld_field!(addr, descriptor)),
                                new_ptr: Box::new(Type::Ptr(Box::new(Type::USize))),
                            } + conv_usize!((size_of!(Type::ISize))),
                        ),
                    },
                    ctx,
                );
            }
            _ => todo!("Can't yet get `size_of_val` on non-slice dst. dst:{ptr_ty}"),
        }
    }
    let tpe = ctx.monomorphize(pointed_ty);
    let tpe = ctx.type_from_cache(tpe);
    place_set(destination, conv_usize!(size_of!(tpe)), ctx)
}
