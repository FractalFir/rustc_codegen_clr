// This file contains many unnecesary morphize calls. 
use crate::cil_op::{CILOp, FieldDescriptor};
use rustc_middle::mir::{Place, PlaceElem};
use rustc_middle::ty::{IntTy, Ty, TyCtxt, TyKind,Instance};
use crate::utilis::field_name;
fn slice_head<T>(slice: &[T]) -> (&T, &[T]) {
    assert!(!slice.is_empty());
    let last = &slice[slice.len() - 1];
    (last, &slice[..(slice.len() - 1)])
}
fn pointed_type(ty: Ty) -> Ty {
    if let TyKind::Ref(_region, inner, _mut) = ty.kind() {
        *inner
    } else if let TyKind::RawPtr(inner_and_mut) = ty.kind() {
        inner_and_mut.ty   
    }else {
        panic!("{ty:?} is not a pointer type!");
    }
}
fn body_ty_is_by_adress(last_ty: &Ty) -> bool {
    match *last_ty.kind() {
        TyKind::Int(_) => false,
        TyKind::Adt(_, _) => true,
        TyKind::Ref(_region, inner, _mut) => false,
        TyKind::RawPtr(_) => false,
        _ => todo!("TODO: body_ty_is_by_adress does not support type {last_ty:?}"),
    }
}
fn local_get(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::LDLoc(0)
    } else if local > method.arg_count {
        CILOp::LDLoc((local - method.arg_count) as u32)
    } else {
        CILOp::LDArg((local - 1) as u32)
    }
}
fn local_set(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::STLoc(0)
    } else if local > method.arg_count {
        CILOp::STLoc((local - method.arg_count) as u32)
    } else {
        CILOp::STArg((local - 1) as u32)
    }
}
fn local_adress(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::LDLocA(0)
    } else if local > method.arg_count {
        CILOp::LDLocA((local - method.arg_count) as u32)
    } else {
        CILOp::LDArgA((local - 1) as u32)
    }
}
fn local_body<'tcx>(local: usize, method: &rustc_middle::mir::Body<'tcx>) -> (CILOp, Ty<'tcx>) {
    let ty = method.local_decls[local.into()].ty;
    if body_ty_is_by_adress(&ty) {
        (local_adress(local, method), ty)
    } else {
        (local_get(local, method), ty)
    }
}
fn place_elem_get<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: Ty<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => deref_op(curr_type,ctx),
        PlaceElem::Field(index, field_type) => {
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, ctx);
            let field_type = crate::utilis::monomorphize(&method_instance, *field_type, ctx);
            let field_name = field_name(curr_type, index.as_u32());
            let curr_type = crate::r#type::Type::from_ty(curr_type, ctx);
            let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) = curr_type {
                dotnet_type.as_ref().clone()
            } else {
                panic!();
            };
            let field_desc = FieldDescriptor::boxed(
                curr_type,
                crate::r#type::Type::from_ty(field_type, ctx),
                field_name,
            );
            vec![CILOp::LDField(field_desc)]
        }
        _ => todo!("Can't handle porojection {place_elem:?} in get"),
    }
}
fn place_elem_set<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: Ty<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => ptr_set_op(curr_type,ctx),
        PlaceElem::Field(index, field_type) => {
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, ctx);
            let field_type = crate::utilis::monomorphize(&method_instance, *field_type, ctx);
            let field_name = field_name(curr_type, index.as_u32());
            
            let curr_type = crate::r#type::Type::from_ty(curr_type, ctx);
            let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) = curr_type {
                dotnet_type.as_ref().clone()
            } else {
                panic!();
            };
            let field_desc = FieldDescriptor::boxed(
                curr_type,
                crate::r#type::Type::from_ty(field_type, ctx),
                field_name,
            );
            vec![CILOp::STField(field_desc)]
        }
        _ => todo!("Can't handle porojection {place_elem:?} in set"),
    }
}

fn place_elem_body<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
) -> (Ty<'ctx>, Vec<CILOp>) {
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_type);
            if body_ty_is_by_adress(&pointed) {
                (pointed, vec![])
            } else {
                (pointed, deref_op(curr_type,tyctx))
            }
        }
        PlaceElem::Field(index, field_type) => {
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let field_type = crate::utilis::monomorphize(&method_instance, *field_type, tyctx);
            let field_name = field_name(curr_type, index.as_u32());
            let curr_type = crate::r#type::Type::from_ty(curr_type, tyctx);
            let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) = curr_type {
                dotnet_type.as_ref().clone()
            } else {
                panic!();
            };
            let field_desc = FieldDescriptor::boxed(
                curr_type,
                crate::r#type::Type::from_ty(field_type, tyctx),
                field_name,
            );
            if body_ty_is_by_adress(&field_type) {
                (field_type, vec![CILOp::LDFieldAdress(field_desc)])
            } else {
                (field_type, vec![CILOp::LDField(field_desc)])
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in body"),
    }
}
fn ptr_set_op<'ctx>(curr_type: Ty<'ctx>,tyctx: TyCtxt<'ctx>) -> Vec<CILOp> {
    match curr_type.kind() {
        TyKind::Int(int_ty) => match int_ty {
            IntTy::I8 => vec![CILOp::STIndI8],
            _ => todo!("TODO: can't deref int type {int_ty:?} yet"),
        },
        TyKind::Adt(_,_)=>{
            let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) = crate::r#type::Type::from_ty(curr_type,tyctx) {
                dotnet_type
            } else {
                panic!();
            };
            vec![CILOp::STObj(curr_type)]
        },
        TyKind::Ref(_,_,_)=>vec![CILOp::STIndISize],
        _ => todo!("TODO: can't deref type {curr_type:?} yet"),
    }
}
fn deref_op<'ctx>(curr_type: Ty<'ctx>,tyctx: TyCtxt<'ctx>) -> Vec<CILOp> {
    match curr_type.kind() {
        TyKind::Int(int_ty) => match int_ty {
            IntTy::I8 => vec![CILOp::LDIndI8],
            _ => todo!("TODO: can't deref int type {int_ty:?} yet"),
        },
        TyKind::Adt(_,_)=>{
            let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) = crate::r#type::Type::from_ty(curr_type,tyctx) {
                dotnet_type
            } else {
                panic!();
            };
            vec![CILOp::LdObj(curr_type)]
        },
        TyKind::Ref(_,_,_)=>vec![CILOp::LDIndISize],
        _ => todo!("TODO: can't deref type {curr_type:?} yet"),
    }
}
/// Returns the ops for getting the value of place.
pub fn place_get<'a>(
    place: &Place<'a>,
    ctx: TyCtxt<'a>,
    method: &rustc_middle::mir::Body<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    let mut ops = Vec::with_capacity(place.projection.len());
    if place.projection.is_empty() {
        ops.push(local_get(place.local.as_usize(), method));
        return ops;
    } else {
        let (op, mut ty) = local_body(place.local.as_usize(), method);
        ty = crate::utilis::monomorphize(&method_instance, ty, ctx);
        ops.push(op);
        let (head, body) = slice_head(place.projection);
        for elem in body {
            println!("elem:{elem:?} ty:{ty:?}");
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx,method_instance);
            ty = crate::utilis::monomorphize(&method_instance, curr_ty, ctx);
            ops.extend(curr_ops);
        }
        ops.extend(place_elem_get(head, ty, ctx,method_instance));
        ops
    }
}
/// Returns the ops for getting the value of place.
pub fn place_adress<'a>(
    place: &Place<'a>,
    ctx: TyCtxt<'a>,
    method: &rustc_middle::mir::Body<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    let mut ops = Vec::with_capacity(place.projection.len());
    if place.projection.is_empty() {
        ops.push(local_adress(place.local.as_usize(), method));
        return ops;
    } else {
        let (op, mut ty) = local_body(place.local.as_usize(), method);
        ops.push(op);
        todo!();
    }
}
pub(crate) fn place_set<'a>(
    place: &Place<'a>,
    ctx: TyCtxt<'a>,
    value_calc: Vec<CILOp>,
    method: &rustc_middle::mir::Body<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    let mut ops = Vec::with_capacity(place.projection.len());
    if place.projection.is_empty() {
        ops.extend(value_calc);
        ops.push(local_set(place.local.as_usize(), method));
        return ops;
    } else {
        let (op, mut ty) = local_body(place.local.as_usize(), method);
        ops.push(op);
        let (head, body) = slice_head(place.projection);
        for elem in body {
            println!("elem:{elem:?} ty:{ty:?}");
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx,method_instance);
            ty = crate::utilis::monomorphize(&method_instance, curr_ty, ctx);
            ops.extend(curr_ops);
        }
        ops.extend(value_calc);
        ty = crate::utilis::monomorphize(&method_instance, ty, ctx);
        ops.extend(place_elem_set(head, ty, ctx,method_instance));
        ops
    }
}
