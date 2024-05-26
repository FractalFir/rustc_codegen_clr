use super::{pointed_type, PlaceTy};

use crate::{
    cil::{CallSite, FieldDescriptor},
    cil_tree::{cil_node::CILNode, cil_root::CILRoot},
    place::{body_ty_is_by_adress, deref_op},
    r#type::Type,
    {assert_morphic, call, conv_usize, ld_field},
};
use cilly::   fn_sig::FnSig;
use rustc_middle::mir::PlaceElem;
use rustc_middle::ty::{Instance, Ty, TyCtxt, TyKind};
pub fn local_body<'tcx>(
    local: usize,
    method: &rustc_middle::mir::Body<'tcx>,
    tyctx: TyCtxt<'tcx>,
    method_instance: &Instance<'tcx>,
) -> (CILNode, Ty<'tcx>) {
    let ty = method.local_decls[local.into()].ty;
    let ty = crate::utilis::monomorphize(method_instance, ty, tyctx);
    if body_ty_is_by_adress(ty) {
        (super::adress::local_adress(local, method), ty)
    } else {
        (super::get::local_get(local, method), ty)
    }
}
pub fn place_elem_body<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
    _body: &rustc_middle::mir::Body,
    type_cache: &mut crate::r#type::TyCache,
    parrent_node: CILNode,
) -> (PlaceTy<'ctx>, CILNode) {
    let curr_ty = match curr_type {
        PlaceTy::Ty(ty) => PlaceTy::Ty(crate::utilis::monomorphize(&method_instance, ty, tyctx)),
        PlaceTy::EnumVariant(enm, idx) => PlaceTy::EnumVariant(
            crate::utilis::monomorphize(&method_instance, enm, tyctx),
            idx,
        ),
    };
    assert_morphic!(curr_ty);
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_ty);
            assert_morphic!(pointed);
            if body_ty_is_by_adress(pointed) {
                (pointed.into(), parrent_node)
            } else {
                (
                    pointed.into(),
                    deref_op(
                        pointed.into(),
                        tyctx,
                        &method_instance,
                        type_cache,
                        parrent_node,
                    ),
                )
            }
        }
        PlaceElem::Field(index, field_ty) => match curr_ty {
            PlaceTy::Ty(curr_ty) => {
                let field_ty = crate::utilis::monomorphize(&method_instance, *field_ty, tyctx);
                if crate::r#type::pointer_to_is_fat(curr_ty, tyctx, Some(method_instance)) {
                    assert_eq!(
                        index.as_u32(),
                        0,
                        "Can't handle DST with more than 1 field."
                    );
                    let curr_type = type_cache.type_from_cache(
                        Ty::new_ptr(tyctx, curr_ty, rustc_middle::ty::Mutability::Mut),
                        tyctx,
                        Some(method_instance),
                    );
                    let field_type = type_cache.type_from_cache(
                        Ty::new_ptr(tyctx, field_ty, rustc_middle::ty::Mutability::Mut),
                        tyctx,
                        Some(method_instance),
                    );
                    return (
                        curr_ty.into(),
                        CILNode::TemporaryLocal(Box::new((
                            curr_type,
                            [CILRoot::SetTMPLocal {
                                value: parrent_node,
                            }]
                            .into(),
                            CILNode::LdObj {
                                ptr: CILNode::LoadAddresOfTMPLocal.into(),
                                obj: field_type.into(),
                            },
                        ))),
                    );

                    //todo!("Handle DST fields. DST:")
                }
                let _curr_type = crate::utilis::monomorphize(&method_instance, curr_ty, tyctx);
                let field_desc = crate::utilis::field_descrptor(
                    curr_ty,
                    (*index).into(),
                    tyctx,
                    method_instance,
                    type_cache,
                );
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
                let owner = crate::utilis::monomorphize(&method_instance, enm, tyctx);
                let field_desc = crate::utilis::enum_field_descriptor(
                    owner,
                    index.as_u32(),
                    var_idx,
                    tyctx,
                    method_instance,
                    type_cache,
                );
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
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let variant_type = PlaceTy::EnumVariant(curr_type, variant.as_u32());

            (variant_type, parrent_node)
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_ty
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                tyctx.optimized_mir(method_instance.def_id()),
            );
            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                    let inner_type =
                        type_cache.type_from_cache(inner, tyctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, tyctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let addr = CILNode::Add(
                        CILNode::LDField {
                            addr: parrent_node.into(),
                            field: desc.into(),
                        }
                        .into(),
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
                            super::deref_op(
                                super::PlaceTy::Ty(inner),
                                tyctx,
                                &method_instance,
                                type_cache,
                                addr,
                            ),
                        )
                    }
                }
                TyKind::Array(element, _length) => {
                    let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let element_type =
                        type_cache.type_from_cache(element, tyctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if body_ty_is_by_adress(element) {
                        let ops = CILNode::Call {
                            site: crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[Type::Ptr(array_type.into()), Type::USize],
                                    Type::Ptr(element_type.into()),
                                ),
                                false,
                            )
                            .into(),
                            args: [parrent_node, CILNode::ZeroExtendToUSize(index.into())].into(),
                        };
                        ((element).into(), ops)
                    } else {
                        let ops = CILNode::Call {
                            site: crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Item".into(),
                                FnSig::new(
                                    &[Type::Ptr(array_type.into()), Type::USize],
                                    element_type,
                                ),
                                false,
                            )
                            .into(),
                            args: [parrent_node, CILNode::ZeroExtendToUSize(index.into())].into(),
                        };
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
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                    let inner_type =
                        type_cache.type_from_cache(inner, tyctx, Some(method_instance));
                    let slice = type_cache
                        .slice_ty(inner, tyctx, Some(method_instance))
                        .as_dotnet()
                        .unwrap();
                    let desc = FieldDescriptor::new(
                        slice.clone(),
                        Type::Ptr(Type::Void.into()),
                        "data_pointer".into(),
                    );
                    let metadata = FieldDescriptor::new(slice, Type::USize, "metadata".into());
                    let addr = ld_field!(parrent_node.clone(), desc)
                        + call!(
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
                            super::deref_op(
                                super::PlaceTy::Ty(inner),
                                tyctx,
                                &method_instance,
                                type_cache,
                                addr,
                            ),
                        )
                    }
                }
                TyKind::Array(element, _length) => {
                    let element_ty = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let element =
                        type_cache.type_from_cache(element_ty, tyctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if body_ty_is_by_adress(element_ty) {
                        let ops = CILNode::Call {
                            site: crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Address".into(),
                                FnSig::new(
                                    &[Type::Ptr(array_type.into()), Type::USize],
                                    Type::Ptr(element.into()),
                                ),
                                false,
                            )
                            .into(),
                            args: [parrent_node, CILNode::ZeroExtendToUSize(index.into())].into(),
                        };
                        ((element_ty).into(), ops)
                    } else {
                        let ops = CILNode::Call {
                            site: crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Item".into(),
                                FnSig::new(&[Type::Ptr(array_type.into()), Type::USize], element),
                                false,
                            )
                            .into(),
                            args: [parrent_node, CILNode::ZeroExtendToUSize(index.into())].into(),
                        };
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
