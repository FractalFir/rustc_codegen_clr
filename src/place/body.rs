use super::{pointed_type, PlaceTy};

use crate::{
    assembly::MethodCompileCtx,
    assert_morphic,
    place::{body_ty_is_by_adress, deref_op},
    r#type::fat_ptr_to,
};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_usize,
    field_desc::FieldDescriptor, fn_sig::FnSig, ld_field, size_of, v2::Int, Type,
};
use rustc_middle::mir::PlaceElem;
use rustc_middle::ty::{Ty, TyKind};
pub fn local_body<'tcx>(local: usize, ctx: &mut MethodCompileCtx<'tcx, '_>) -> (CILNode, Ty<'tcx>) {
    let ty = ctx.body().local_decls[local.into()].ty;
    let ty = ctx.monomorphize(ty);
    if body_ty_is_by_adress(ty, ctx) {
        (super::adress::local_adress(local, ctx.body()), ty)
    } else {
        (super::get::local_get(local, ctx.body()), ty)
    }
}
fn body_field<'a>(
    curr_type: super::PlaceTy<'a>,
    ctx: &mut MethodCompileCtx<'a, '_>,
    field_index: u32,
    field_ty: Ty<'a>,
    parrent_node: CILNode,
) -> (PlaceTy<'a>, CILNode) {
    match curr_type {
        super::PlaceTy::Ty(curr_type) => {
            let curr_type = ctx.monomorphize(curr_type);
            let field_type = ctx.monomorphize(field_ty);
            match (
                crate::r#type::pointer_to_is_fat(curr_type, ctx.tcx(), ctx.instance()),
                crate::r#type::pointer_to_is_fat(field_type, ctx.tcx(), ctx.instance()),
            ) {
                (false, false) => {
                    let field_desc = crate::utilis::field_descrptor(curr_type, field_index, ctx);
                    if body_ty_is_by_adress(field_type, ctx) {
                        (
                            (field_type).into(),
                            CILNode::LDFieldAdress {
                                field: field_desc.into(),
                                addr: parrent_node.into(),
                            },
                        )
                    } else {
                        (
                            (field_type).into(),
                            CILNode::LDField {
                                field: field_desc.into(),
                                addr: parrent_node.into(),
                            },
                        )
                    }
                }
                (false, true) => panic!("Sized type {curr_type:?} contains an unsized field of type {field_type}. This is a bug."),
                (true,false)=>todo!("Can't yet handle access of a sized field of an unsized type. "),
                (true,true)=>{
                    assert_eq!(
                        field_index,
                        0,
                        "Can't handle DST with more than 1 field."
                    );
                    let curr_type = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tcx(),
                        curr_type,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    let field_type = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tcx(),
                        field_type,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                   (
                    field_ty.into(),
                        CILNode::TemporaryLocal(Box::new((
                            curr_type.clone(),
                            [CILRoot::SetTMPLocal {
                                value: CILNode::LdObj {
                                    ptr: Box::new(parrent_node),
                                    obj: Box::new(curr_type),
                                },
                            }]
                            .into(),
                            CILNode::LoadAddresOfTMPLocal
                                .cast_ptr(ctx.asm_mut().nptr(field_type)),
                        )))
                    )
                }
            }
        }
        super::PlaceTy::EnumVariant(enm, var_idx) => {
            let owner = ctx.monomorphize(enm);
            let field_desc = crate::utilis::enum_field_descriptor(owner, field_index, var_idx, ctx);

            (
                field_ty.into(),
                CILNode::LDFieldAdress {
                    field: field_desc.into(),
                    addr: parrent_node.into(),
                },
            )
        }
    }
}
pub fn place_elem_body<'tcx>(
    place_elem: &PlaceElem<'tcx>,
    curr_type: PlaceTy<'tcx>,
    ctx: &mut MethodCompileCtx<'tcx, '_>,
    parrent_node: CILNode,
) -> (PlaceTy<'tcx>, CILNode) {
    let curr_ty = match curr_type {
        PlaceTy::Ty(ty) => PlaceTy::Ty(ctx.monomorphize(ty)),
        PlaceTy::EnumVariant(enm, idx) => PlaceTy::EnumVariant(ctx.monomorphize(enm), idx),
    };
    assert_morphic!(curr_ty);
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_ty);
            assert_morphic!(pointed);
            if body_ty_is_by_adress(pointed, ctx) {
                (pointed.into(), parrent_node)
            } else {
                (pointed.into(), deref_op(pointed.into(), ctx, parrent_node))
            }
        }
        PlaceElem::Field(field_index, field_ty) => body_field(
            curr_type,
            ctx,
            field_index.as_u32(),
            *field_ty,
            parrent_node,
        ),
        PlaceElem::Downcast(_, variant) => {
            let curr_type = curr_ty
                .as_ty()
                .expect("Can't get enum variant of an enum varaint!");
            let curr_type = ctx.monomorphize(curr_type);
            let variant_type = PlaceTy::EnumVariant(curr_type, variant.as_u32());

            (variant_type, parrent_node)
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_ty
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(index.as_usize(), ctx.body());
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);
                    let desc = FieldDescriptor::new(
                        slice,
                        ctx.asm_mut().nptr(Type::Void.into()),
                        crate::DATA_PTR.into(),
                    );
                    let addr = ld_field!(parrent_node, desc)
                        .cast_ptr(ctx.asm_mut().nptr(inner_type.clone()))
                        + (index * CILNode::ZeroExtendToUSize(size_of!(inner_type).into()));

                    if body_ty_is_by_adress(inner, ctx) {
                        (inner.into(), addr)
                    } else {
                        (
                            inner.into(),
                            super::deref_op(super::PlaceTy::Ty(inner), ctx, addr),
                        )
                    }
                }
                TyKind::Array(element, _length) => {
                    let element = ctx.monomorphize(*element);
                    let element_type = ctx.type_from_cache(element);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_class_ref().expect("Non array type");
                    if body_ty_is_by_adress(element, ctx) {
                        let ops = call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    [ctx.asm_mut().nref(array_type), Type::Int(Int::USize)],
                                    ctx.asm_mut().nptr(element_type),
                                ),
                                false,
                            ),
                            [parrent_node, CILNode::ZeroExtendToUSize(index.into())]
                        );
                        ((element).into(), ops)
                    } else {
                        let ops = call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Item".into(),
                                FnSig::new(
                                    [ctx.asm_mut().nref(array_type), Type::Int(Int::USize)],
                                    element_type,
                                ),
                                false,
                            ),
                            [parrent_node, CILNode::ZeroExtendToUSize(index.into())]
                        );
                        ((element).into(), ops)
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::ConstantIndex {
            offset,
            min_length: _,
            from_end,
        } => {
            let curr_ty = curr_ty
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = CILNode::LdcU64(*offset);
            assert!(!from_end);
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = ctx.monomorphize(*inner);
                    let inner_type = ctx.type_from_cache(inner);
                    let slice = fat_ptr_to(Ty::new_slice(ctx.tcx(), inner), ctx);
                    let desc = FieldDescriptor::new(
                        slice,
                        ctx.asm_mut().nptr(Type::Void),
                        crate::DATA_PTR.into(),
                    );
                    let metadata =
                        FieldDescriptor::new(slice, Type::Int(Int::USize), "metadata".into());
                    let addr = Box::new(ld_field!(parrent_node.clone(), desc))
                        .cast_ptr(ctx.asm_mut().nptr(inner_type))
                        + (index) * conv_usize!(CILNode::SizeOf(inner_type.into()));
                    if body_ty_is_by_adress(inner, ctx) {
                        (inner.into(), addr)
                    } else {
                        (
                            inner.into(),
                            super::deref_op(super::PlaceTy::Ty(inner), ctx, addr),
                        )
                    }
                }
                TyKind::Array(element, _length) => {
                    let element_ty = ctx.monomorphize(*element);
                    let element = ctx.type_from_cache(element_ty);
                    let array_type = ctx.type_from_cache(curr_ty);
                    let array_dotnet = array_type.as_class_ref().expect("Non array type");
                    if body_ty_is_by_adress(element_ty, ctx) {
                        let ops = call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    [ctx.asm_mut().nref(array_type.into()), Type::Int(Int::USize)],
                                    ctx.asm_mut().nptr(element.into()),
                                ),
                                false,
                            ),
                            [parrent_node, CILNode::ZeroExtendToUSize(index.into())]
                        );
                        ((element_ty).into(), ops)
                    } else {
                        let ops = call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Item".into(),
                                FnSig::new(
                                    [ctx.asm_mut().nref(array_type), Type::Int(Int::USize)],
                                    element
                                ),
                                false,
                            ),
                            [parrent_node, CILNode::ZeroExtendToUSize(index.into())]
                        );
                        ((element_ty).into(), ops)
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in body"),
    }
}
