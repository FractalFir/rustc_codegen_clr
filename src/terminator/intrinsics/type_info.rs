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
pub fn is_val_statically_known<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,

    type_cache: &mut TyCache,
) -> CILRoot {
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
pub fn size_of_val<'tyctx>(
    args: &[Spanned<Operand<'tyctx>>],
    destination: &Place<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    body: &'tyctx Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    call_instance: Instance<'tyctx>,
    type_cache: &mut TyCache,
) -> CILRoot {
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
        let ptr_ty =
            crate::utilis::monomorphize(&method_instance, args[0].node.ty(body, tyctx), tyctx);
        match pointed_ty.kind() {
            TyKind::Slice(inner) => {
                let slice_tpe: DotnetTypeRef = type_cache
                    .type_from_cache(ptr_ty, tyctx, method_instance)
                    .as_dotnet()
                    .unwrap();
                let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                let inner_type = type_cache.type_from_cache(inner, tyctx, method_instance);
                let descriptor = FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
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

                let descriptor = FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
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
                        ptr: Box::new(ld_field!(addr, descriptor) + (size_of!(Type::ISize))),
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
