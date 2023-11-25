use super::{place_get_length, pointed_type, PlaceTy};
use crate::cil_op::{CILOp, FieldDescriptor};
use crate::r#type::{DotnetTypeRef, Type};
use crate::utilis::field_name;
use rustc_middle::mir::{Place, PlaceElem};
use rustc_middle::ty::{FloatTy, Instance, IntTy, ParamEnv, Ty, TyCtxt, TyKind, UintTy};

pub fn local_set(local: usize, method: &rustc_middle::mir::Body) -> CILOp {
    if local == 0 {
        CILOp::STLoc(0)
    } else if local > method.arg_count {
        CILOp::STLoc((local - method.arg_count) as u32)
    } else {
        CILOp::STArg((local - 1) as u32)
    }
}
fn place_elem_set_at<'a>(
    curr_type: PlaceTy<'a>,
    tyctx: TyCtxt<'a>,
    method_instance: &Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    let curr_ty = curr_type.as_ty().expect("Can't index into enum!");
    let curr_ty = crate::utilis::monomorphize(method_instance, curr_ty, tyctx);
    let tpe = type_cache.type_from_cache(curr_ty, tyctx, Some(*method_instance));
    let class = if let Type::DotnetType(dotnet) = &tpe {
        dotnet
    } else {
        panic!("Can't index into type {tpe:?}");
    };
    let index_ty = Type::USize;
    let _element_ty = crate::r#type::element_type(curr_ty);

    let signature =
        crate::function_sig::FnSig::new(&[tpe.clone(), index_ty, Type::GenericArg(0)], &Type::Void);
    vec![CILOp::Call(crate::cil_op::CallSite::boxed(
        Some(class.as_ref().clone()),
        "set_Item".into(),
        signature,
        false,
    ))]
}
pub fn place_elem_set<'a>(
    place_elem: &PlaceElem<'a>,
    curr_type: PlaceTy<'a>,
    ctx: TyCtxt<'a>,
    method_instance: Instance<'a>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    match place_elem {
        PlaceElem::Deref => {
            let pointed_type = pointed_type(curr_type);
            ptr_set_op(pointed_type.into(), ctx, &method_instance, type_cache)
        }
        PlaceElem::Field(index, _field_type) => {
            if let PlaceTy::Ty(curr_type) = curr_type {
                let curr_type = crate::utilis::monomorphize(&method_instance, curr_type, ctx);
                let field_desc = crate::utilis::field_descrptor(
                    curr_type,
                    (*index).into(),
                    ctx,
                    method_instance,
                    type_cache,
                );
                vec![CILOp::STField(field_desc.into())]
            } else {
                todo!("Can't set fields of enum variants yet!");
            }
        }
        PlaceElem::Index(index) => {
            let curr_ty = curr_type
                .as_ty()
                .expect("INVALID PLACE: Indexing into enum variant???");
            let index = crate::place::local_get(
                index.as_usize(),
                ctx.optimized_mir(method_instance.def_id()),
            );

            match curr_ty.kind() {
                TyKind::Slice(inner) => {
                    let inner = crate::utilis::monomorphize(&method_instance, *inner, ctx);
                    let inner_type = type_cache.type_from_cache(inner, ctx, Some(method_instance));
                    let desc = FieldDescriptor::new(
                        DotnetTypeRef::slice(),
                        Type::Void.pointer_to(),
                        "data_address".into(),
                    );
                    let ptr_set_op =
                        ptr_set_op(super::PlaceTy::Ty(inner), ctx, &method_instance, type_cache);
                    let mut ops = vec![
                        CILOp::LDField(desc.into()),
                        index,
                        CILOp::SizeOf(inner_type.into()),
                        CILOp::Mul,
                        CILOp::Add,
                    ];
                    ops.extend(ptr_set_op);
                    ops
                }
                _ => todo!("Can't index into {curr_ty}!"),
            }
        }
        /*
        PlaceElem::Index(index) => {
            let mut ops = vec![crate::place::local_adress(
                index.as_usize(),
                ctx.optimized_mir(method_instance.def_id()),
            )];
            ops.extend(place_elem_set_at(
                curr_type,
                ctx,
                &method_instance,
                type_cache,
            ));
            ops
        }
        PlaceElem::ConstantIndex {
            offset,
            min_length: _,
            from_end,
        } => {
            let mut ops = if !from_end {
                vec![CILOp::LdcI64(*offset as i64)]
            } else {
                let mut get_len = place_get_length(curr_type, ctx, method_instance, type_cache);
                get_len.extend(vec![CILOp::LdcI64(*offset as i64), CILOp::Sub]);
                get_len
            };
            ops.extend(place_elem_set_at(
                curr_type,
                ctx,
                &method_instance,
                type_cache,
            ));
            ops
        }*/
        _ => todo!("Can't handle porojection {place_elem:?} in set"),
    }
}
/// Returns a set of instructons to set a pointer to a pointed_type to a value from the stack.
fn ptr_set_op<'ctx>(
    pointed_type: PlaceTy<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method_instance: &Instance<'ctx>,
    type_cache: &mut crate::r#type::TyCache,
) -> Vec<CILOp> {
    if let PlaceTy::Ty(pointed_type) = pointed_type {
        match pointed_type.kind() {
            TyKind::Int(int_ty) => match int_ty {
                IntTy::I8 => vec![CILOp::STIndI8],
                IntTy::I16 => vec![CILOp::STIndI16],
                IntTy::I32 => vec![CILOp::STIndI32],
                IntTy::I64 => vec![CILOp::STIndI64],
                IntTy::Isize => vec![CILOp::STIndISize],
                IntTy::I128 => vec![CILOp::STObj(Box::new(DotnetTypeRef::int_128().into()))],
            },
            TyKind::Uint(int_ty) => match int_ty {
                UintTy::U8 => vec![CILOp::STIndI8],
                UintTy::U16 => vec![CILOp::STIndI16],
                UintTy::U32 => vec![CILOp::STIndI32],
                UintTy::U64 => vec![CILOp::STIndI64],
                UintTy::Usize => vec![CILOp::STIndISize],
                UintTy::U128 => vec![CILOp::STObj(Box::new(DotnetTypeRef::uint_128().into()))],
            },
            TyKind::Float(float_ty) => match float_ty {
                FloatTy::F32 => vec![CILOp::STIndF32],
                FloatTy::F64 => vec![CILOp::STIndF64],
            },
            TyKind::Bool => vec![CILOp::STIndI8], // Both Rust bool and a managed bool are 1 byte wide. .NET bools are 4 byte wide only in the context of Marshaling/PInvoke,
            // due to historic reasons(BOOL was an alias for int in early Windows, and it stayed this way.) - FractalFir
            TyKind::Char => vec![CILOp::STIndI32], // always 4 bytes wide: https://doc.rust-lang.org/std/primitive.char.html#representation
            TyKind::Adt(_, _) => {
                let pointed_type =
                    type_cache.type_from_cache(pointed_type, tyctx, Some(*method_instance));
                vec![CILOp::STObj(pointed_type.into())]
            }
            TyKind::Tuple(_) => {
                let pointed_type =
                    type_cache.type_from_cache(pointed_type, tyctx, Some(*method_instance));
                // This is interpreted as a System.ValueTuple and can be treated as an ADT
                vec![CILOp::STObj(pointed_type.into())]
            }
            TyKind::Ref(_, _, _) => vec![CILOp::STIndISize],
            TyKind::RawPtr(_) => vec![CILOp::STIndISize],
            _ => todo!(" can't deref type {pointed_type:?} yet"),
        }
    } else {
        todo!("Can't set the value behind a poitner to an enum variant!");
    }
}
