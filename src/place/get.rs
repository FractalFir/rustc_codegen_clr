use crate::cil::{CILOp, FieldDescriptor};
use crate::function_sig::FnSig;
use crate::r#type::{DotnetTypeRef, Type};

use rustc_middle::mir::{Place, PlaceElem};
use rustc_middle::ty::{Instance, TyCtxt, TyKind};

pub(super) fn local_get(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::LDLoc(0)
    } else if local > method.arg_count {
        CILOp::LDLoc((local - method.arg_count) as u32)
    } else {
        CILOp::LDArg((local - 1) as u32)
    }
}
/// Returns the ops for getting the value of place.
pub fn place_get<'a>(
    place: &Place<'a>,
    ctx: TyCtxt<'a>,
    method: &rustc_middle::mir::Body<'a>,
    method_instance: Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    let mut ops = Vec::with_capacity(place.projection.len());
    if place.projection.is_empty() {
        ops.push(local_get(place.local.as_usize(), method));
        ops
    } else {
        let (op, mut ty) = super::local_body(place.local.as_usize(), method);
        ty = crate::utilis::monomorphize(&method_instance, ty, ctx);
        let mut ty = ty.into();
        ops.push(op);
        let (head, body) = super::slice_head(place.projection);
        for elem in body {
            let (curr_ty, curr_ops) =
                super::place_elem_body(elem, ty, ctx, method_instance, method, type_cache);
            ty = curr_ty.monomorphize(&method_instance, ctx);
            ops.extend(curr_ops);
        }
        ops.extend(place_elem_get(head, ty, ctx, method_instance, type_cache));
        ops
    }
}
fn place_elem_get<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: super::PlaceTy<'a>,
    tyctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => super::deref_op(
            super::pointed_type(curr_type).into(),
            tyctx,
            &method_instance,
            type_cache,
        ),
        PlaceElem::Field(index, _field_type) => match curr_type {
            super::PlaceTy::Ty(curr_type) => {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
                let _field_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);

                let field_desc = crate::utilis::field_descrptor(
                    curr_type,
                    (*index).into(),
                    tyctx,
                    method_instance,
                    type_cache,
                );
                vec![CILOp::LDField(field_desc.into())]
            }
            super::PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = crate::utilis::monomorphize(&method_instance, enm, tyctx);
                let field_desc = crate::utilis::enum_field_descriptor(
                    owner,
                    index.as_u32(),
                    var_idx,
                    tyctx,
                    method_instance,
                    type_cache,
                );
                //let field_desc = crate::utilis::field_descrptor(enm, field_idx, ctx, method_instance, type_cache);
                let ops = vec![CILOp::LDField(field_desc.into())];
                ops
                //todo!("Can't get fields of enum variants yet!");
            }
        },
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
                    let slice = type_cache.slice_ty(inner,tyctx,Some(method_instance)).as_dotnet().unwrap();
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
                    ops.extend(deref_op);
                    ops
                }
                TyKind::Array(_element, _length) => {
                    //let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    let ops = vec![
                        index,
                        CILOp::Call(
                            crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Item".into(),
                                FnSig::new(&[array_type, Type::USize], &Type::GenericArg(0)),
                                false,
                            )
                            .into(),
                        ),
                    ];
                    ops
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! {todo!("Can't index into {curr_ty}!")}
                }
            }
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
                    let slice = type_cache.slice_ty(inner,tyctx,Some(method_instance)).as_dotnet().unwrap();
                    let desc = FieldDescriptor::new(
                        slice,
                        Type::Ptr(Type::Void.into()),
                        "data_address".into(),
                    );
                    let derf_op = super::deref_op(
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
                    ops.extend(derf_op);
                    ops
                }
                TyKind::Array(_element, _length) => {
                    //let element = crate::utilis::monomorphize(&method_instance, *element, tyctx);
                    let array_type =
                        type_cache.type_from_cache(curr_ty, tyctx, Some(method_instance));
                    let array_dotnet = array_type.as_dotnet().expect("Non array type");
                    let ops = vec![
                        index,
                        CILOp::Call(
                            crate::cil::CallSite::new(
                                Some(array_dotnet),
                                "get_Item".into(),
                                FnSig::new(&[array_type, Type::USize], &Type::GenericArg(0)),
                                false,
                            )
                            .into(),
                        ),
                    ];
                    ops
                }
                _ => {
                    rustc_middle::ty::print::with_no_trimmed_paths! { todo!("Can't index into {curr_ty}!")}
                }
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in get"),
    }
}
