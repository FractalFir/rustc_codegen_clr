use crate::cil_op::{CILOp, CallSite, FieldDescriptor};
use crate::operand::handle_operand;
use crate::r#type::{DotnetTypeRef, TyCache, Type};
use rustc_middle::mir::{CastKind, NullOp};
use rustc_middle::{
    mir::{Place, Rvalue},
    ty::{Instance, TyCtxt, TyKind},
};
pub fn handle_rvalue<'tcx>(
    rvalue: &Rvalue<'tcx>,
    tyctx: TyCtxt<'tcx>,
    target_location: &Place<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
    tycache: &mut TyCache,
) -> Vec<CILOp> {
    let res = match rvalue {
        Rvalue::Use(operand) => handle_operand(operand, tyctx, method, method_instance, tycache),
        Rvalue::CopyForDeref(place) => {
            crate::place::place_get(place, tyctx, method, method_instance, tycache)
        }
        Rvalue::Ref(_region, _kind, place) => {
            crate::place::place_adress(place, tyctx, method, method_instance, tycache)
        }
        Rvalue::AddressOf(_mutability, place) => {
            crate::place::place_adress(place, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::PointerCoercion(_) | CastKind::PtrToPtr, operand, _) => {
            handle_operand(operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::BinaryOp(binop, operands) => crate::binop::binop_unchecked(
            *binop,
            &operands.0,
            &operands.1,
            tyctx,
            method,
            method_instance,
            tycache,
        ),
        Rvalue::CheckedBinaryOp(binop, operands) => crate::checked_binop::binop_checked(
            *binop,
            &operands.0,
            &operands.1,
            tyctx,
            method,
            method_instance,
            tycache,
        ),
        Rvalue::UnaryOp(binop, operand) => {
            crate::unop::unop(*binop, operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::IntToInt, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            [
                handle_operand(operand, tyctx, method, method_instance, tycache),
                crate::casts::int_to_int(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Cast(CastKind::FloatToInt, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            [
                handle_operand(operand, tyctx, method, method_instance, tycache),
                crate::casts::float_to_int(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Cast(CastKind::IntToFloat, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            [
                handle_operand(operand, tyctx, method, method_instance, tycache),
                crate::casts::int_to_float(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::NullaryOp(op, ty) => match op {
            NullOp::SizeOf => {
                let ty = crate::utilis::monomorphize(&method_instance, *ty, tyctx);
                let ty = Box::new(tycache.type_from_cache(ty, tyctx, Some(method_instance)));
                vec![CILOp::SizeOf(ty)]
            }
            NullOp::AlignOf => vec![CILOp::LdcI64(align_of(*ty) as i64), CILOp::ConvUSize(false)],
            _ => todo!("Unsuported nullary {op:?}!"),
        },
        Rvalue::Aggregate(aggregate_kind, field_index) => crate::aggregate::handle_aggregate(
            tyctx,
            target_location,
            method,
            aggregate_kind.as_ref(),
            field_index,
            method_instance,
            tycache,
        ),
        Rvalue::Cast(CastKind::Transmute, operand, dst) => {
            let dst = crate::utilis::monomorphize(&method_instance, *dst, tyctx);
            let dst_ty = dst;
            let dst = tycache.type_from_cache(dst, tyctx, Some(method_instance));
            let src = operand.ty(&method.local_decls, tyctx);
            let src = crate::utilis::monomorphize(&method_instance, src, tyctx);
            let src = tycache.type_from_cache(src, tyctx, Some(method_instance));
            match (&src, &dst) {
                (
                    Type::ISize | Type::USize | Type::Ptr(_),
                    Type::ISize | Type::USize | Type::Ptr(_),
                ) => handle_operand(operand, tyctx, method, method_instance, tycache),
                (Type::U16, Type::DotnetChar) => {
                    handle_operand(operand, tyctx, method, method_instance, tycache)
                }
                (Type::F64, Type::U64) => {
                    let mut res = handle_operand(operand, tyctx, method, method_instance, tycache);
                    res.extend([
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndI64,
                        CILOp::FreeTMPLocal,
                    ]);
                    res
                }
                (Type::F32, Type::U32) => {
                    let mut res = handle_operand(operand, tyctx, method, method_instance, tycache);
                    res.extend([
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndI32,
                        CILOp::FreeTMPLocal,
                    ]);
                    res
                }
                (Type::U32, Type::F32) => {
                    let mut res = handle_operand(operand, tyctx, method, method_instance, tycache);
                    res.extend([
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndF32,
                        CILOp::FreeTMPLocal,
                    ]);
                    res
                }
                (Type::U64, Type::F64) => {
                    let mut res = handle_operand(operand, tyctx, method, method_instance, tycache);
                    res.extend([
                        CILOp::NewTMPLocal(Type::Ptr(src.into()).into()),
                        CILOp::SetTMPLocal,
                        CILOp::LoadAddresOfTMPLocal,
                        CILOp::LDIndF64,
                        CILOp::FreeTMPLocal,
                    ]);
                    res
                }
                (_, _) => {
                    eprintln!(
                        "transmute from {src:?} to {dst:?} does not have special handling yet!"
                    );
                    let mut res = handle_operand(operand, tyctx, method, method_instance, tycache);
                    res.push(CILOp::NewTMPLocal(src.into()));
                    res.push(CILOp::SetTMPLocal);
                    res.push(CILOp::LoadAddresOfTMPLocal);
                    res.push(CILOp::FreeTMPLocal);
                    res.extend(crate::place::deref_op(
                        crate::place::PlaceTy::Ty(dst_ty),
                        tyctx,
                        &method_instance,
                        tycache,
                    ));
                    res
                }
            }
        }
        Rvalue::Cast(CastKind::PointerFromExposedAddress, operand, _) => {
            //FIXME: the documentation of this cast(https://doc.rust-lang.org/nightly/std/ptr/fn.from_exposed_addr.html) is a bit confusing,
            //since this seems to be something deeply linked to the rust memory model.
            // I assume this to be ALWAYS equivalent to `usize as *const/mut T`, but this may not always be the case.
            // If something breaks in the fututre, this is a place that needs checking.

            // Cast from usize/isize to any *T is a NOP, so we just have to load the operand.
            handle_operand(operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::PointerExposeAddress, operand, _) => {
            //FIXME: the documentation of this cast(https://doc.rust-lang.org/nightly/std/primitive.pointer.html#method.expose_addrl) is a bit confusing,
            //since this seems to be something deeply linked to the rust memory model.
            // I assume this to be ALWAYS equivalent to `*const/mut T as usize`, but this may not always be the case.
            // If something breaks in the fututre, this is a place that needs checking.

            // Cast to usize/isize from any *T is a NOP, so we just have to load the operand.
            handle_operand(operand, tyctx, method, method_instance, tycache)
        }
        Rvalue::Cast(CastKind::FloatToFloat, operand, target) => {
            let target = crate::utilis::monomorphize(&method_instance, *target, tyctx);
            let target = tycache.type_from_cache(target, tyctx, Some(method_instance));
            let mut ops = handle_operand(operand, tyctx, method, method_instance, tycache);
            match target {
                Type::F32 => ops.push(CILOp::ConvF32(false)),
                Type::F64 => ops.push(CILOp::ConvF64(false)),
                _ => panic!("Can't preform a FloatToFloat cast to type {target:?}"),
            }
            ops
        }
        Rvalue::Cast(kind, _operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        Rvalue::Discriminant(place) => {
            let mut ops =
                crate::place::place_adress(place, tyctx, method, method_instance, tycache);
            let owner_ty = place.ty(method, tyctx).ty;
            let owner_ty = crate::utilis::monomorphize(&method_instance, owner_ty, tyctx);
            let owner = tycache.type_from_cache(owner_ty, tyctx, Some(method_instance));
            //TODO: chose proper tag type based on variant count of `owner`
            let discr_type = crate::r#type::Type::U8; //owner_ty
            let owner = if let crate::r#type::Type::DotnetType(dotnet_type) = owner {
                dotnet_type.as_ref().clone()
            } else {
                panic!();
            };
            ops.push(CILOp::LDField(Box::new(
                crate::cil_op::FieldDescriptor::new(owner, discr_type, "_tag".into()),
            )));
            ops
        }
        Rvalue::Len(operand) => {
            let mut ops =
                crate::place::place_adress(operand, tyctx, method, method_instance, tycache);
            let ty = operand.ty(method, tyctx);
            let ty = crate::utilis::monomorphize(&method_instance, ty, tyctx);
            // let tpe = tycache.type_from_cache(ty.ty, tyctx, Some(method_instance));
            match ty.ty.kind() {
                TyKind::Slice(inner) => {
                    let name: String = format!(
                        "core.ptr.metadata.PtrComponents{}",
                        crate::r#type::mangle_ty(*inner)
                    )
                    .into();
                    let slice_tpe = DotnetTypeRef::new(None, &name);
                    let descriptor =
                        FieldDescriptor::new(slice_tpe, Type::USize, "metadata".into());
                    ops.extend([CILOp::LDField(descriptor.into())])
                }
                _ => todo!("Get length of type {ty:?}"),
            }
            ops
        }
        _ => todo!("Unhandled RValue {rvalue:?}"),
    };
    res
}
fn align_of(ty: rustc_middle::ty::Ty) -> u64 {
    use rustc_middle::ty::{IntTy, TyKind};
    match ty.kind() {
        TyKind::Int(int) => match int {
            IntTy::I8 => std::mem::align_of::<i8>() as u64,
            _ => todo!("Can't calcuate align of int type {int:?}"),
        },
        //TODO: While always returing 8 for ADTs won't cause crashes, it is inefficent.
        TyKind::Adt(_, _) => 8,
        _ => todo!("Can't calcualte the aligement of type {ty:?}"),
    }
}
