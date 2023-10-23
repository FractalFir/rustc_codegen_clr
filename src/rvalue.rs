use crate::cil_op::{CILOp, CallSite};
use crate::operand::handle_operand;
use crate::r#type::Type;
use rustc_middle::mir::{CastKind, NullOp};
use rustc_middle::{
    mir::{Place, Rvalue},
    ty::{Instance, TyCtxt},
};
pub fn handle_rvalue<'tcx>(
    rvalue: &Rvalue<'tcx>,
    tcx: TyCtxt<'tcx>,
    target_location: &Place<'tcx>,
    method: &rustc_middle::mir::Body<'tcx>,
    method_instance: Instance<'tcx>,
) -> Vec<CILOp> {
    let res = match rvalue {
        Rvalue::Use(operand) => {
            crate::operand::handle_operand(operand, tcx, method, method_instance)
        }
        Rvalue::CopyForDeref(place) => crate::place::place_get(place, tcx, method, method_instance),
        Rvalue::Ref(_region, _kind, place) => {
            crate::place::place_adress(place, tcx, method, method_instance)
        }
        Rvalue::AddressOf(_mutability, place) => {
            crate::place::place_adress(place, tcx, method, method_instance)
        }
        Rvalue::Cast(CastKind::PointerCoercion(_) | CastKind::PtrToPtr, operand, _) => {
            crate::operand::handle_operand(operand, tcx, method, method_instance)
        }
        Rvalue::BinaryOp(binop, operands) => crate::binop::binop_unchecked(
            *binop,
            &operands.0,
            &operands.1,
            tcx,
            method,
            method_instance,
        ),
        Rvalue::CheckedBinaryOp(binop, operands) => crate::checked_binop::binop_checked(
            *binop,
            &operands.0,
            &operands.1,
            tcx,
            method,
            method_instance,
        ),
        Rvalue::UnaryOp(binop, operand) => {
            crate::unop::unop(*binop, operand, tcx, method, method_instance)
        }
        Rvalue::Cast(CastKind::IntToInt, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target, tcx,&method_instance);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src, tcx,&method_instance);
            [
                crate::operand::handle_operand(operand, tcx, method, method_instance),
                crate::casts::int_to_int(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Cast(CastKind::FloatToInt, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target, tcx,&method_instance);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src, tcx,&method_instance);
            [
                crate::operand::handle_operand(operand, tcx, method, method_instance),
                crate::casts::float_to_int(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::Cast(CastKind::IntToFloat, operand, target) => {
            let target = crate::r#type::Type::from_ty(*target, tcx,&method_instance);
            let src = operand.ty(&method.local_decls, tcx);
            let src = crate::r#type::Type::from_ty(src, tcx,&method_instance);
            [
                crate::operand::handle_operand(operand, tcx, method, method_instance),
                crate::casts::int_to_float(src, target),
            ]
            .into_iter()
            .flatten()
            .collect()
        }
        Rvalue::NullaryOp(op, ty) => match op {
            NullOp::SizeOf => {
                let ty = crate::utilis::monomorphize(&method_instance, *ty, tcx);
                let ty = Box::new(crate::r#type::Type::from_ty(ty, tcx,&method_instance));
                vec![CILOp::SizeOf(ty)]
            }
            NullOp::AlignOf=>vec![CILOp::LdcI64(align_of(*ty) as i64),CILOp::ConvUSize(false)],
            _ => todo!("Unsuported nullary {op:?}!"),
        },
        Rvalue::Aggregate(aggregate_kind, field_index) => crate::aggregate::handle_aggregate(
            tcx,
            target_location,
            method,
            aggregate_kind.as_ref(),
            field_index,
            method_instance,
        ),
        Rvalue::Cast(CastKind::Transmute, operand, dst) => {
            let src = operand.ty(method, tcx);
            let src = Type::from_ty(src, tcx,&method_instance);
            let dst = Type::from_ty(*dst, tcx,&method_instance);
            match (&src, &dst) {
                (Type::ISize | Type::USize, Type::Ptr(_)) => {
                    handle_operand(operand, tcx, method, method_instance)
                }
                (Type::Ptr(_), Type::ISize | Type::USize) => {
                    handle_operand(operand, tcx, method, method_instance)
                }
                (Type::U16, Type::DotnetChar) => {
                    handle_operand(operand, tcx, method, method_instance)
                }
                _ => todo!("Unhandled transmute from {src:?} to {dst:?}"),
            }
        }
        Rvalue::Cast(kind, _operand, _) => todo!("Unhandled cast kind {kind:?}, rvalue:{rvalue:?}"),
        Rvalue::Discriminant(place) => {
            let mut ops = crate::place::place_adress(place, tcx, method, method_instance);
            let owner_ty = place.ty(method, tcx).ty;
            let owner = crate::r#type::Type::from_ty(owner_ty, tcx,&method_instance);
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
            let mut ops = crate::place::place_adress(operand, tcx, method, method_instance);
            let tpe = operand.ty(method, tcx);
            let class = crate::r#type::Type::from_ty(tpe.ty, tcx,&method_instance)
                .as_dotnet()
                .expect("Can't get the dotnet type!");
            let signature = crate::function_sig::FnSig::new(
                &[class.clone().into()],
                &crate::r#type::Type::USize,
            );
            ops.push(CILOp::Call(CallSite::boxed(
                Some(class),
                "GetLength".into(),
                signature,
                false,
            )));
            ops
            //todo!("Can't get the length of {operand:?}");
        }
        _ => todo!("Unhandled RValue {rvalue:?}"),
    };
    res
}
fn align_of(ty:rustc_middle::ty::Ty)->u64{
    use rustc_middle::ty::{IntTy,TyKind};
    match ty.kind(){
        TyKind::Int(int)=>match int{
            IntTy::I8=>std::mem::align_of::<i8>() as u64,
            _=>todo!("Can't calcuate align of int type {int:?}"),
        },
        //TODO: While always returing 8 for ADTs won't cause crashes, it is inefficent.
        TyKind::Adt(_,_)=>8,
        _=>todo!("Can't calcualte the aligement of type {ty:?}"),
    }
}