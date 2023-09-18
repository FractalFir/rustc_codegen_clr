use crate::cil_op::{CILOp, CallSite};
use crate::r#type::{DotnetTypeRef, Type};
use rustc_middle::mir::{interpret::ConstValue, interpret::Scalar, Constant, ConstantKind};
use rustc_middle::ty::{FloatTy, IntTy, Ty, TyCtxt, TyKind, UintTy};
pub fn handle_constant<'ctx>(
    constant: &Constant<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
) -> Vec<CILOp> {
    let const_kind = constant.literal;
    match const_kind {
        ConstantKind::Val(value, const_ty) => load_const_value(value, const_ty, tyctx),
        _ => todo!("Unhanded const kind {const_kind:?}!"),
    }
}
fn load_const_value<'ctx>(
    const_val: ConstValue<'ctx>,
    const_ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
) -> Vec<CILOp> {
    match const_val {
        ConstValue::Scalar(scalar) => load_const_scalar(scalar, const_ty, tyctx),
        //ConstValue::ZeroSized => vec![BaseIR::DebugComment("ZeroSized!".into())],
        _ => todo!("Unhandled const value {const_val:?} of type {const_ty:?}"),
    }
}
fn load_const_scalar<'ctx>(
    scalar: Scalar,
    scalar_type: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
) -> Vec<CILOp> {
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int
            .try_to_uint(scalar.size())
            .expect("IMPOSSIBLE. Size of scalar was not equal to itself."),
        Scalar::Ptr(_, _) => todo!("Can't handle scalar pointers yet!"),
    };
    match scalar_type.kind() {
        TyKind::Int(int_type) => load_const_int(scalar_u128, int_type, tyctx),
        TyKind::Uint(uint_type) => load_const_uint(scalar_u128, uint_type, tyctx),
        TyKind::Float(ftype) => load_const_float(scalar_u128, ftype, tyctx),
        _ => todo!("Can't load scalar constants of type {scalar_type:?}!"),
    }
}
fn load_const_float(value: u128, int_type: &FloatTy, tyctx: TyCtxt) -> Vec<CILOp> {
    match int_type {
        FloatTy::F32 => {
            let value = f32::from_ne_bytes((value as u32).to_ne_bytes());
            vec![CILOp::LdcF32(value)]
        }
        FloatTy::F64 => {
            let value = f64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcF64(value)]
        }
    }
}
fn load_const_int(value: u128, int_type: &IntTy, tyctx: TyCtxt) -> Vec<CILOp> {
    match int_type {
        IntTy::I8 => {
            let value = i8::from_ne_bytes([value as u8]);
            vec![CILOp::LdcI32(value as i32), CILOp::ConvI8(false)]
        }
        IntTy::I16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            vec![CILOp::LdcI32(value as i32), CILOp::ConvI16(false)]
        }
        IntTy::I32 => {
            let value = i32::from_ne_bytes((value as u32).to_ne_bytes());
            vec![CILOp::LdcI32(value)]
        }
        IntTy::I64 => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvI64(false)]
        }
        IntTy::Isize => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvISize(true)]
        }
        IntTy::I128 => {
            let low = (value & (u64::MAX as u128)) as u64;
            let high = (value << 64) as u64;
            let low = i64::from_ne_bytes((low as u64).to_ne_bytes());
            let high = i64::from_ne_bytes((high as u64).to_ne_bytes());
            let i128_class = DotnetTypeRef::new(Some("System.Runtime"), "System.Int128");
            let ctor_sig = crate::function_sig::FnSig::new(&[Type::U64, Type::U64], &Type::I128);
            vec![
                CILOp::LdcI64(high),
                CILOp::LdcI64(low),
                CILOp::Call(CallSite::boxed(
                    Some(i128_class),
                    ".ctor".into(),
                    ctor_sig,
                    true,
                )),
            ]
        }
    }
}
fn load_const_uint(value: u128, int_type: &UintTy, tyctx: TyCtxt) -> Vec<CILOp> {
    match int_type {
        UintTy::U8 => {
            let value = i8::from_ne_bytes([value as u8]);
            vec![CILOp::LdcI32(value as i32), CILOp::ConvU8(false)]
        }
        UintTy::U16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            vec![CILOp::LdcI32(value as i32), CILOp::ConvU16(false)]
        }
        UintTy::U32 => {
            let value = i32::from_ne_bytes((value as u32).to_ne_bytes());
            vec![CILOp::LdcI32(value)]
        }
        UintTy::U64 => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvU64(false)]
        }
        UintTy::Usize => {
            let value = i64::from_ne_bytes((value as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvUSize(true)]
        }
        UintTy::U128 => {
            let low = (value & (u64::MAX as u128)) as u64;
            let high = (value << 64) as u64;
            let low = i64::from_ne_bytes((low as u64).to_ne_bytes());
            let high = i64::from_ne_bytes((high as u64).to_ne_bytes());
            let i128_class = DotnetTypeRef::new(Some("System.Runtime"), "System.UInt128");
            let ctor_sig = crate::function_sig::FnSig::new(&[Type::U64, Type::U64], &Type::U128);
            vec![
                CILOp::LdcI64(high),
                CILOp::LdcI64(low),
                CILOp::Call(CallSite::boxed(
                    Some(i128_class),
                    ".ctor".into(),
                    ctor_sig,
                    true,
                )),
            ]
        }
    }
}
