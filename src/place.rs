// This file contains many unnecesary morphize calls.
use crate::cil_op::{CILOp, FieldDescriptor};
use crate::r#type::DotnetTypeRef;
use crate::utilis::field_name;
use rustc_middle::mir::{Place, PlaceElem};
use rustc_middle::ty::{Instance, IntTy, Ty, TyCtxt, TyKind, UintTy};
fn slice_head<T>(slice: &[T]) -> (&T, &[T]) {
    assert!(!slice.is_empty());
    let last = &slice[slice.len() - 1];
    (last, &slice[..(slice.len() - 1)])
}
fn pointed_type(ty: PlaceTy) -> Ty {
    if let PlaceTy::Ty(ty) = ty {
        if let TyKind::Ref(_region, inner, _mut) = ty.kind() {
            *inner
        } else if let TyKind::RawPtr(inner_and_mut) = ty.kind() {
            inner_and_mut.ty
        } else {
            panic!("{ty:?} is not a pointer type!");
        }
    } else {
        panic!("Can't dereference enum variant!");
    }
}
fn body_ty_is_by_adress(last_ty: &Ty) -> bool {
    match *last_ty.kind() {
        TyKind::Int(_) => false,
        TyKind::Uint(_) => false,
        TyKind::Adt(_, _) => true,
        TyKind::Ref(_region, _inner, _mut) => false,
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
    curr_type: PlaceTy<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => deref_op(pointed_type(curr_type).into(), ctx),
        PlaceElem::Field(index, field_type) => match curr_type {
            PlaceTy::Ty(curr_type) => {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, ctx);
                let field_type = crate::utilis::generic_field_ty(curr_type, index.as_u32(), ctx);
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
            PlaceTy::EnumVariant(enm, var_idx) => {
                let owner = crate::utilis::monomorphize(&method_instance, enm, ctx);
                let variant_name = crate::utilis::variant_name(owner, var_idx);
                let owner = crate::utilis::monomorphize(&method_instance, enm, ctx);
                let field_type = crate::utilis::generic_field_ty(owner, index.as_u32(), ctx);
                let owner = crate::r#type::Type::from_ty(owner, ctx);
                let owner = if let crate::r#type::Type::DotnetType(owner) = owner {
                    owner.as_ref().clone()
                } else {
                    panic!();
                };
                let field_name = field_name(enm, index.as_u32());
                let mut field_owner = owner;

                field_owner.append_path(&format!("/{variant_name}"));
                let field_desc = FieldDescriptor::boxed(
                    field_owner,
                    crate::r#type::Type::from_ty(field_type, ctx),
                    field_name,
                );
                let ops = vec![CILOp::LDField(field_desc)];
                println!("Using ops:{ops:?} to get field of an enum variant!");
                ops
                //todo!("Can't get fields of enum variants yet!");
            }
        },
        _ => todo!("Can't handle porojection {place_elem:?} in get"),
    }
}
fn place_elem_set<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: PlaceTy<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => {
            let pointed_type = pointed_type(curr_type);
            ptr_set_op(pointed_type.into(), ctx)
        }
        PlaceElem::Field(index, _field_type) => {
            if let PlaceTy::Ty(curr_type) = curr_type {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, ctx);
                let field_type = crate::utilis::generic_field_ty(curr_type, index.as_u32(), ctx);
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
            } else {
                todo!("Can't set fields of enum variants yet!");
            }
        }
        _ => todo!("Can't handle porojection {place_elem:?} in set"),
    }
}

fn place_elem_body<'ctx>(
    place_elem: &PlaceElem<'ctx>,
    curr_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: Instance<'ctx>,
) -> (PlaceTy<'ctx>, Vec<CILOp>) {
    match place_elem {
        PlaceElem::Deref => {
            let pointed = pointed_type(curr_type);
            println!("Dereferencing {curr_type:?} in place_elem_body ");
            if body_ty_is_by_adress(&pointed) {
                (pointed.into(), vec![])
            } else {
                (pointed.into(), deref_op(pointed.into(), tyctx))
            }
        }
        PlaceElem::Field(index, _field_type) => {
            if let PlaceTy::Ty(curr_type) = curr_type {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
                let field_type = crate::utilis::generic_field_ty(curr_type, index.as_u32(), tyctx);
                //let field_type = crate::utilis::monomorphize(&method_instance, *field_type, tyctx);
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
                    (field_type.into(), vec![CILOp::LDFieldAdress(field_desc)])
                } else {
                    (field_type.into(), vec![CILOp::LDField(field_desc)])
                }
            } else {
                todo!("Can't get fields of enum variants yet!");
            }
        }
        PlaceElem::Downcast(symbol, variant) => {
            let curr_type = curr_type
                .as_ty()
                .expect("Can't get enum variant of an enum varaint!");
            let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, tyctx);
            let curr_dotnet_type = crate::r#type::Type::from_ty(curr_type, tyctx);
            let curr_dotnet_type =
                if let crate::r#type::Type::DotnetType(dotnet_type) = curr_dotnet_type {
                    dotnet_type.as_ref().clone()
                } else {
                    panic!();
                };
            let variant_name = symbol.unwrap();
            let field_name = format!("v_{variant_name}").into();
            let curr_type_name = (curr_dotnet_type).name_path();
            let mut field_type = curr_dotnet_type.clone();
            field_type.append_path(&format!("/{variant_name}"));
            field_type.set_generics_identity();
            let field_desc = FieldDescriptor::boxed(
                curr_dotnet_type.clone(),
                crate::r#type::Type::DotnetType(Box::new(field_type)),
                field_name,
            );
            let variant_type = PlaceTy::EnumVariant(curr_type, variant.as_u32());
            (variant_type, vec![CILOp::LDFieldAdress(field_desc)])
        }
        _ => todo!("Can't handle porojection {place_elem:?} in body"),
    }
}
fn ptr_set_op<'ctx>(curr_type: PlaceTy<'ctx>, tyctx: TyCtxt<'ctx>) -> Vec<CILOp> {
    if let PlaceTy::Ty(curr_type) = curr_type {
        match curr_type.kind() {
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I8 => vec![CILOp::STIndI8],
                IntTy::I16 => vec![CILOp::STIndI16],
                IntTy::I32 => vec![CILOp::STIndI32],
                IntTy::I64 => vec![CILOp::STIndI64],
                IntTy::Isize => vec![CILOp::STIndISize],
                _ => todo!("TODO: can't deref int type {int_ty:?} yet"),
            },
            TyKind::Uint(int_ty) => match int_ty {
                UintTy::U8 => vec![CILOp::STIndI8],
                _ => todo!("TODO: can't deref int type {int_ty:?} yet"),
            },
            TyKind::Adt(_, _) => {
                let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) =
                    crate::r#type::Type::from_ty(curr_type, tyctx)
                {
                    dotnet_type
                } else {
                    panic!();
                };
                vec![CILOp::STObj(curr_type)]
            }
            TyKind::Ref(_, _, _) => vec![CILOp::STIndISize],
            TyKind::RawPtr(_) => vec![CILOp::STIndISize],
            _ => todo!(" can't deref type {curr_type:?} yet"),
        }
    } else {
        todo!("Can't set the value behind a poitner to an enum variant!");
    }
}
pub fn deref_op<'ctx>(curr_type: PlaceTy<'ctx>, tyctx: TyCtxt<'ctx>) -> Vec<CILOp> {
    let res = if let PlaceTy::Ty(curr_type) = curr_type {
        match curr_type.kind() {
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I8 => vec![CILOp::LDIndI8],
                IntTy::I16 => vec![CILOp::LDIndI16],
                IntTy::I32 => vec![CILOp::LDIndI32],
                IntTy::I64 => vec![CILOp::LDIndI64],
                IntTy::Isize => vec![CILOp::LDIndISize],
                IntTy::I128 => todo!("Can't dereference 128 bit intigers!"), //vec![CILOp::LdObj(Box::new())],
                                                                             //_ => todo!("TODO: can't deref int type {int_ty:?} yet"),
            },
            TyKind::Uint(int_ty) => match int_ty {
                UintTy::U8 => vec![CILOp::LDIndI8],
                UintTy::U16 => vec![CILOp::LDIndI16],
                UintTy::U32 => vec![CILOp::LDIndI32],
                UintTy::U64 => vec![CILOp::LDIndI64],
                UintTy::Usize => vec![CILOp::LDIndISize],
                UintTy::U128 => todo!("Can't dereference 128 bit intigers!"), //vec![CILOp::LdObj(Box::new())],
                                                                             //_ => todo!("TODO: can't deref int type {int_ty:?} yet"),
            },
            TyKind::Adt(_, _) => {
                let curr_type = if let crate::r#type::Type::DotnetType(dotnet_type) =
                    crate::r#type::Type::from_ty(curr_type, tyctx)
                {
                    dotnet_type
                } else {
                    panic!();
                };
                vec![CILOp::LdObj(curr_type)]
            }
            TyKind::Ref(_, _, _) => vec![CILOp::LDIndISize],
            TyKind::RawPtr(_) => vec![CILOp::LDIndISize],
            _ => todo!("TODO: can't deref type {curr_type:?} yet"),
        }
    } else {
        todo!("Can't dereference enum variants yet!")
    };
    println!("using ops {res:?} to deref type {curr_type:?}");
    res
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
        ops
    } else {
        let (op, mut ty) = local_body(place.local.as_usize(), method);
        ty = crate::utilis::monomorphize(&method_instance, ty, ctx);
        let mut ty = ty.into();
        ops.push(op);
        let (head, body) = slice_head(place.projection);
        for elem in body {
            println!("elem:{elem:?} ty:{ty:?}");
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx, method_instance);
            ty = curr_ty.monomorphize(&method_instance, ctx);
            ops.extend(curr_ops);
        }
        ops.extend(place_elem_get(head, ty, ctx, method_instance));
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
        ops
    } else {
        let (op, mut ty) = local_body(place.local.as_usize(), method);
        ty = crate::utilis::monomorphize(&method_instance, ty, ctx);
        let mut ty = ty.into();
        ops.push(op);
        let (head, body) = slice_head(place.projection);
        for elem in body {
            println!("elem:{elem:?} ty:{ty:?}");
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx, method_instance);
            ty = curr_ty.monomorphize(&method_instance, ctx);
            ops.extend(curr_ops);
        }
        ops.extend(place_elem_body(head, ty, ctx, method_instance).1);
        ops
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
        ops
    } else {
        let (op, ty) = local_body(place.local.as_usize(), method);
        let mut ty = ty.into();
        ops.push(op);
        let (head, body) = slice_head(place.projection);
        for elem in body {
            println!("elem:{elem:?} ty:{ty:?}");
            let (curr_ty, curr_ops) = place_elem_body(elem, ty, ctx, method_instance);
            ty = curr_ty.monomorphize(&method_instance, ctx);
            ops.extend(curr_ops);
        }
        ops.extend(value_calc);
        ty = ty.monomorphize(&method_instance, ctx);
        ops.extend(place_elem_set(head, ty, ctx, method_instance));
        ops
    }
}
#[derive(Debug, Clone, Copy)]
pub enum PlaceTy<'ctx> {
    Ty(Ty<'ctx>),
    EnumVariant(Ty<'ctx>, u32),
}
impl<'ctx> From<Ty<'ctx>> for PlaceTy<'ctx> {
    fn from(ty: Ty<'ctx>) -> Self {
        Self::Ty(ty)
    }
}
impl<'ctx> PlaceTy<'ctx> {
    pub fn monomorphize(&self, method_instance: &Instance<'ctx>, ctx: TyCtxt<'ctx>) -> Self {
        match self {
            Self::Ty(inner) => Self::Ty(crate::utilis::monomorphize(method_instance, *inner, ctx)),
            Self::EnumVariant(enm, variant) => Self::EnumVariant(
                crate::utilis::monomorphize(method_instance, *enm, ctx),
                *variant,
            ),
        }
    }
    pub fn as_ty(&self) -> Option<Ty<'ctx>> {
        match self {
            Self::Ty(inner) => Some(*inner),
            _ => None,
        }
    }
}
