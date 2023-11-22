use crate::cil_op::{CILOp, FieldDescriptor};
use crate::r#type::{Type, DotnetTypeRef};
use crate::utilis::field_name;
use rustc_middle::mir::{Place, PlaceElem};
use rustc_middle::ty::{FloatTy, Instance, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy};

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
fn place_elem_get_at<'a>(
    curr_type: super::PlaceTy<'a>,
    tyctx: TyCtxt<'a>,
    method_instance: &Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    let curr_ty = curr_type.as_ty().expect("Can't index into enum!");
    let curr_ty = crate::utilis::monomorphize(method_instance, curr_ty, tyctx);
    let tpe = type_cache.type_from_cache(curr_ty, tyctx);
    let class = if let Type::DotnetType(dotnet) = &tpe {
        dotnet
    } else {
        panic!("Can't index into type {tpe:?}");
    };
    let index_ty = Type::USize;
    let _element_ty = crate::r#type::element_type(curr_ty);

    let signature = crate::function_sig::FnSig::new(&[tpe.clone(), index_ty], &Type::GenericArg(0));
    vec![CILOp::Call(crate::cil_op::CallSite::boxed(
        Some(class.as_ref().clone()),
        "get_Item".into(),
        signature,
        false,
    ))]
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
                let field_desc = crate::utilis::field_descrptor(
                    owner,
                    (var_idx + 1).into(),
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
            let curr_ty = curr_type.as_ty().expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                tyctx.optimized_mir(method_instance.def_id()),
            );
            
            match curr_ty.kind(){
                TyKind::Slice(inner)=>{
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, tyctx);
                    let inner_type = type_cache.type_from_cache(inner, tyctx);
                    let desc = FieldDescriptor::new(DotnetTypeRef::slice(),Type::Void.pointer_to(),"data_address".into());
                    let deref_op = super::deref_op(super::PlaceTy::Ty(inner), tyctx, &method_instance, type_cache);
                    let mut ops = vec![CILOp::LDField(desc.into()),index,CILOp::SizeOf(inner_type.into()),CILOp::Mul,CILOp::Add,];
                    ops.extend(deref_op);
                    ops
                },
                _=>todo!("Can't index into {curr_ty}!"),
            }
            /* 
            let mut ops = vec![crate::place::local_get(
                index.as_usize(),
                tyctx.optimized_mir(method_instance.def_id()),
            )];
            ops.extend(place_elem_get_at(
                curr_type,
                tyctx,
                &method_instance,
                type_cache,
            ));
            ops*/
        }/* 
        PlaceElem::ConstantIndex {
            offset,
            min_length: _,
            from_end,
        } => {
            let mut ops = if !from_end {
                vec![CILOp::LdcI64(*offset as i64)]
            } else {
                let mut get_len =
                    super::place_get_length(curr_type, tyctx, method_instance, type_cache);
                get_len.extend(vec![CILOp::LdcI64(*offset as i64), CILOp::Sub]);
                get_len
            };
            ops.extend(place_elem_get_at(
                curr_type,
                tyctx,
                &method_instance,
                type_cache,
            ));
            ops
        }*/
        _ => todo!("Can't handle porojection {place_elem:?} in get"),
    }
}
