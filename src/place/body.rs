use super::{pointed_type, PlaceTy};

use crate::{
    assembly::MethodCompileCtx,
    assert_morphic,
    place::{body_ty_is_by_adress, deref_op},
    r#type::Type,
};
use cilly::{
    call, call_site::CallSite, cil_node::CILNode, cil_root::CILRoot, conv_usize,
    field_desc::FieldDescriptor, fn_sig::FnSig, ld_field,
};
use rustc_middle::mir::PlaceElem;
use rustc_middle::ty::{Ty, TyKind};
pub fn local_body<'tcx>(
    local: usize,
    ctx: &mut MethodCompileCtx<'tcx, '_, '_>,
) -> (CILNode, Ty<'tcx>) {
    let ty = ctx.body().local_decls[local.into()].ty;
    let ty = ctx.monomorphize(ty);
    if body_ty_is_by_adress(ty) {
        (super::adress::local_adress(local, ctx.body()), ty)
    } else {
        (super::get::local_get(local, ctx.body()), ty)
    }
}
pub fn place_elem_body<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    ctx: &mut MethodCompileCtx<'ctx, '_, '_>,
    parrent_node: CILNode,
) -> (PlaceTy<'ctx>, CILNode) {
    let curr_ty = match curr_type {
        PlaceTy::Ty(ty) => PlaceTy::Ty(ctx.monomorphize(ty)),
        PlaceTy::EnumVariant(enm, idx) => PlaceTy::EnumVariant(ctx.monomorphize(enm), idx),
    };
    assert_morphic!(curr_ty);
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_ty);
            assert_morphic!(pointed);
            if body_ty_is_by_adress(pointed) {
                (pointed.into(), parrent_node)
            } else {
                (pointed.into(), deref_op(pointed.into(), ctx, parrent_node))
            }
        }
        PlaceElem::Field(index, field_ty) => match curr_ty {
            PlaceTy::Ty(curr_ty) => {
                let field_ty = ctx.monomorphize(*field_ty);
                if crate::r#type::pointer_to_is_fat(curr_ty, ctx.tyctx(), ctx.instance()) {
                    assert_eq!(
                        index.as_u32(),
                        0,
                        "Can't handle DST with more than 1 field."
                    );
                    let curr_type = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tyctx(),
                        curr_ty,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    let field_type = ctx.type_from_cache(Ty::new_ptr(
                        ctx.tyctx(),
                        field_ty,
                        rustc_middle::ty::Mutability::Mut,
                    ));
                    return (
                        field_ty.into(),
                        CILNode::TemporaryLocal(Box::new((
                            curr_type,
                            [CILRoot::SetTMPLocal {
                                value: parrent_node,
                            }]
                            .into(),
                            CILNode::LdObj {
                                ptr: Box::new(CILNode::TransmutePtr {
                                    val: CILNode::LoadAddresOfTMPLocal.into(),
                                    new_ptr: Box::new(Type::Ptr(Box::new(field_type.clone()))),
                                }),
                                obj: field_type.into(),
                            },
                        ))),
                    );

                    //todo!("Handle DST fields. DST:")
                }

                let field_desc = crate::utilis::field_descrptor(curr_ty, (*index).into(), ctx);
                if body_ty_is_by_adress(field_ty) {
                    (
                        (field_ty).into(),
                        CILNode::LDFieldAdress {
                            field: field_desc.into(),
                            addr: parrent_node.into(),
                        },
                    )
                } else {
                    (
                        (field_ty).into(),
                        CILNode::LDField {
                            field: field_desc.into(),
                            addr: parrent_node.into(),
                        },
                    )
                }
            }
            PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = ctx.monomorphize(enm);
                let field_desc =
                    crate::utilis::enum_field_descriptor(owner, index.as_u32(), var_idx, ctx);
                (
                    (*field_ty).into(),
                    CILNode::LDFieldAdress {
                        field: field_desc.into(),
                        addr: parrent_node.into(),
                    },
                )
            }
        },
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
                    let slice = ctx.slice_ty(inner).as_dotnet().unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let addr = CILNode::Add(
                        Box::new(CILNode::TransmutePtr {
                            val: CILNode::LDField {
                                addr: parrent_node.into(),
                                field: desc.into(),
                            }
                            .into(),
                            new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                        }),
                        CILNode::Mul(
                            index.into(),
                            CILNode::ZeroExtendToUSize(CILNode::SizeOf(inner_type.into()).into())
                                .into(),
                        )
                        .into(),
                    );

                    if body_ty_is_by_adress(inner) {
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
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if body_ty_is_by_adress(element) {
                        let ops = call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[Type::Ptr(array_type.into()), Type::USize],
                                    Type::Ptr(element_type.into()),
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
                                    &[Type::Ptr(array_type.into()), Type::USize],
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
                    let slice = ctx.slice_ty(inner).as_dotnet().unwrap();
                    let desc = FieldDescriptor::new(
                        slice.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let metadata = FieldDescriptor::new(slice, Type::USize, "metadata".into());
                    let addr = CILNode::TransmutePtr {
                        val: Box::new(ld_field!(parrent_node.clone(), desc)),
                        new_ptr: Box::new(Type::Ptr(Box::new(inner_type.clone()))),
                    } + call!(
                        CallSite::builtin(
                            "bounds_check".into(),
                            FnSig::new(&[Type::USize, Type::USize], Type::USize),
                            true
                        ),
                        [index, ld_field!(parrent_node.clone(), metadata)]
                    ) * conv_usize!(CILNode::SizeOf(inner_type.into()));
                    if body_ty_is_by_adress(inner) {
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
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if body_ty_is_by_adress(element_ty) {
                        let ops = call!(
                            CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[Type::Ptr(array_type.into()), Type::USize],
                                    Type::Ptr(element.into()),
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
                                FnSig::new(&[Type::Ptr(array_type.into()), Type::USize], element),
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
