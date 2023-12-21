use super::{pointed_type, PlaceTy};
use crate::assert_morphic;
use crate::cil::{CILOp, FieldDescriptor};
use crate::function_sig::FnSig;
use crate::place::{body_ty_is_by_adress, deref_op};
use crate::r#type::Type;

use rustc_middle::mir::PlaceElem;
use rustc_middle::ty::{Instance, Ty, TyCtxt, TyKind};
pub fn local_body<'tcx>(local: usize, method: &rustc_middle::mir::Body<'tcx>) -> (CILOp, Ty<'tcx>) {
    let ty = method.local_decls[local.into()].ty;
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
) -> (PlaceTy<'ctx>, Vec<CILOp>) {
    let curr_type = match curr_type {
        PlaceTy::Ty(ty) => PlaceTy::Ty(crate::utilis::monomorphize(&method_instance, ty, tyctx)),
        PlaceTy::EnumVariant(enm, idx) => PlaceTy::EnumVariant(
            crate::utilis::monomorphize(&method_instance, enm, tyctx),
            idx,
        ),
    };
    assert_morphic!(curr_type);
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_type);
            assert_morphic!(pointed);
            if body_ty_is_by_adress(pointed) {
                (pointed.into(), vec![])
            } else {
                (
                    pointed.into(),
                    deref_op(pointed.into(), tyctx, &method_instance, type_cache),
                )
            }
        }
        PlaceElem::Field(index, field_type) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                //TODO: Why was this commented out?
                let field_type = crate::utilis::monomorphize(&method_instance, *field_type, tyctx);
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
                let field_desc = crate::utilis::field_descrptor(
                    curr_type,
                    (*index).into(),
                    tyctx,
                    method_instance,
                    type_cache,
                );
                if body_ty_is_by_adress(field_type) {
                    (
                        (field_type).into(),
                        vec![CILOp::LDFieldAdress(field_desc.into())],
                    )
                } else {
                    ((field_type).into(), vec![CILOp::LDField(field_desc.into())])
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
                let ops = vec![CILOp::LDFieldAdress(field_desc.into())];
                ((*field_type).into(), ops)
            }
        },
        PlaceElem::Downcast(symbol, variant) => {
            let curr_type = curr_type
                .as_ty()
                .expect("Can't get enum variant of an enum varaint!");
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let curr_dotnet_type =
                type_cache.type_from_cache(curr_type, tyctx, Some(method_instance));
            let curr_dotnet_type =
                if let crate::r#type::Type::DotnetType(dotnet_type) = curr_dotnet_type {
                    dotnet_type.as_ref().clone()
                } else {
                    panic!();
                };
            let variant_name = symbol.unwrap();
            let field_name = format!("v_{variant_name}").into();
            let _curr_type_name = (curr_dotnet_type).name_path();
            let mut field_type = curr_dotnet_type.clone();
            field_type.append_path(&format!("/{variant_name}"));
            let field_desc = FieldDescriptor::boxed(
                curr_dotnet_type.clone(),
                crate::r#type::Type::DotnetType(Box::new(field_type)),
                field_name,
            );
            let variant_type = PlaceTy::EnumVariant(curr_type, variant.as_u32());
            (variant_type, vec![CILOp::LDFieldAdress(field_desc)])
        }
        PlaceElem::ConstantIndex {
            offset,
            min_length,
            from_end,
        } => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = CILOp::LdcI64(*offset as i64);
            assert!(!from_end, "Indexing slice form end");
            eprintln!("WARNING: ConstantIndex has required min_length of {min_length}, but bounds checking on const access not supported yet!");
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
                        "data_address".into(),
                    );
                    let ops = vec![
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::Mul,
                        CILOp::Add,
                    ];
                    (inner.into(), ops)
                }

                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
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
                        "data_address".into(),
                    );
                    let deref_op = super::deref_op(
                        super::PlaceTy::Ty(inner),
                        tyctx,
                        &method_instance,
                        type_cache,
                    );
                    let mut ops = vec![
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::Mul,
                        CILOp::Add,
                    ];
                    if !body_ty_is_by_adress(inner) {
                        ops.extend(deref_op);
                    }
                    (inner.into(), ops)
                }
                TyKind::Array(element, _length) => {
                    let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let element_type = type_cache.type_from_cache(element, tyctx, Some(method_instance));
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    if body_ty_is_by_adress(element) {
                        let ops = vec![
                            index,
                            CILOp::Call(
                                crate::cil::CallSite::new(
                                    Some(array_dotnet),
                                    "get_Address".into(),
                                    FnSig::new(
                                        &[array_type, Type::USize],
                                        &Type::Ptr(element_type.into()),
                                    ),
                                    false,
                                )
                                .into(),
                            ),
                        ];
                        ((element).into(), ops)
                    } else {
                        let ops = vec![
                            index,
                            CILOp::Call(
                                crate::cil::CallSite::new(
                                    Some(array_dotnet),
                                    "get_Item".into(),
                                    FnSig::new(&[array_type, Type::USize], &element_type),
                                    false,
                                )
                                .into(),
                            ),
                        ];
                        ((element).into(), ops)
                    }
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in body"),
    }
}
