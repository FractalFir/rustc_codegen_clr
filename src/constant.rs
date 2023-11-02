use crate::cil_op::{CILOp, CallSite};
use crate::r#type::{DotnetTypeRef, Type};
use rustc_abi::Size;
use rustc_middle::mir::{
    interpret::{AllocId, AllocRange, GlobalAlloc, Scalar},
    Const, ConstValue,
};
use rustc_middle::ty::{
    AdtDef, AdtKind, FloatTy, Instance, IntTy, List, Ty, TyCtxt, TyKind, UintTy,
};
pub fn handle_constant<'ctx>(
    constant: &Const<'ctx>,
    tyctx: TyCtxt<'ctx>,

    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    match constant {
        Const::Val(value, const_ty) => {
            load_const_value(*value, *const_ty, tyctx, method, method_instance)
        }
        Const::Unevaluated(_, _) => {
            todo!("Can't handle unevaluated constants yet!");
        }
        _ => todo!("Unhanded const {constant:?}!"),
    }
}
/// Returns the ops neceasry to create constant ADT of type represented by `adt_def` and `subst` with byte values matching the ones in the slice bytes
fn create_const_adt_from_bytes<'ctx>(
    ty: Ty<'ctx>,
    adt_def: AdtDef<'ctx>,
    subst: &'ctx List<rustc_middle::ty::GenericArg<'ctx>>,
    tyctx: TyCtxt<'ctx>,
    bytes: &[u8],
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    match adt_def.adt_kind() {
        AdtKind::Struct => {
            let mut curr_offset = 0;
            let cil_ty = Type::from_ty(ty, tyctx, &method_instance);
            let dotnet_ty = cil_ty.as_dotnet().expect("ADT must be a value type!");
            let mut creator_ops = vec![CILOp::NewTMPLocal(cil_ty.clone().into())];
            for (field_idx, field) in adt_def.all_fields().enumerate() {
                let ftype = field.ty(tyctx, subst);
                let sizeof = crate::utilis::compiletime_sizeof(ftype);
                let field_bytes = &bytes[curr_offset..(curr_offset + sizeof)];
                let field_ops = create_const_from_slice(ftype, tyctx, field_bytes, method_instance);
                creator_ops.push(CILOp::LoadAddresOfTMPLocal);
                creator_ops.extend(field_ops);
                let cil_ftype =
                    crate::utilis::generic_field_ty(ty, field_idx as u32, tyctx, method_instance);
                creator_ops.push(CILOp::STField(crate::cil_op::FieldDescriptor::boxed(
                    dotnet_ty.clone(),
                    cil_ftype,
                    field.name.to_string().into(),
                )));
                println!(
                    "Const field {name} of type {ftype} with bytes {field_bytes:?}",
                    name = field.name
                );
                // Increment the offset.
                curr_offset += sizeof;
            }
            creator_ops.push(CILOp::LoadTMPLocal);
            creator_ops.push(CILOp::FreeTMPLocal);
            creator_ops
        }
        AdtKind::Enum => todo!("Can't load const enum from bytes {bytes:?}!"),
        AdtKind::Union => todo!("Can't load const union from bytes {bytes:?}!"),
    }
}
/// Returns the ops neceasry to create constant value of type `ty` with byte values matching the ones in the slice bytes
fn create_const_from_slice<'ctx>(
    ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    bytes: &[u8],
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    // TODO: Read up on the order of bytes inside a const allocation and ensure it is correct. All .NET target will be Little Enidian, but if we want to support
    // big enidian targets in the future, this will need to be revised.
    match ty.kind() {
        TyKind::Adt(adt_def, subst) => {
            create_const_adt_from_bytes(ty, *adt_def, subst, tyctx, bytes, method_instance)
        }
        TyKind::Int(int) => match int {
            IntTy::I32 => vec![CILOp::LdcI32(i32::from_le_bytes(
                bytes[..std::mem::size_of::<i32>()].try_into().unwrap(),
            ))],
            _ => todo!("Can't yet load const value of type {int:?} with bytes:{bytes:?}"),
        },
        _ => todo!("Can't yet load const value of type {ty:?} with bytes:{bytes:?}"),
    }
}
/// Returns the ops neceasry to create constant value of type `ty` with byte values matching the ones in the allocation
fn create_const_from_data<'ctx>(
    ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    alloc_id: AllocId,
    offset_bytes: u64,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    let alloc = tyctx.global_alloc(alloc_id);
    // Constant should be memory:
    let memory = alloc.unwrap_memory();
    let len = memory.0.len();
    let range = AllocRange {
        start: Size::from_bytes(offset_bytes),
        size: Size::from_bytes((len as u64) - offset_bytes),
    };
    let bytes = memory.0.get_bytes_unchecked(range);
    create_const_from_slice(ty, tyctx, bytes, method_instance)
}
fn load_const_value<'ctx>(
    const_val: ConstValue<'ctx>,
    const_ty: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    match const_val {
        ConstValue::Scalar(scalar) => {
            load_const_scalar(scalar, const_ty, tyctx, method, method_instance)
        }
        ConstValue::ZeroSized => {
            let tpe = Type::from_ty(const_ty, tyctx, &method_instance);
            vec![
                CILOp::NewTMPLocal(tpe.into()),
                CILOp::LoadTMPLocal,
                CILOp::FreeTMPLocal,
            ]
        }
        ConstValue::Slice { data, meta } => {
            todo!("Constant slice allocations are not supported yet data:{data:?},meta:{meta:?}!")
        }
        ConstValue::Indirect { alloc_id, offset } => {
            create_const_from_data(const_ty, tyctx, alloc_id, offset.bytes(), method_instance)
            //todo!("Can't handle by-ref allocation {alloc_id:?} {offset:?}")
        } //_ => todo!("Unhandled const value {const_val:?} of type {const_ty:?}"),
    }
}
fn load_const_scalar<'ctx>(
    scalar: Scalar,
    scalar_type: Ty<'ctx>,
    tyctx: TyCtxt<'ctx>,
    _method: &rustc_middle::mir::Body<'ctx>,
    method_instance: Instance<'ctx>,
) -> Vec<CILOp> {
    let scalar_u128 = match scalar {
        Scalar::Int(scalar_int) => scalar_int
            .try_to_uint(scalar.size())
            .expect("IMPOSSIBLE. Size of scalar was not equal to itself."),
        Scalar::Ptr(ptr, _size) => {
            let (alloc_id, _offset) = ptr.into_parts();
            let global_alloc = tyctx.global_alloc(alloc_id);
            match global_alloc {
                GlobalAlloc::Static(def_id) => {
                    let instance = Instance::mono(tyctx, def_id).polymorphize(tyctx);
                    let symbol_name = tyctx.symbol_name(instance).to_string().into();
                    let ty = tyctx.type_of(def_id).instantiate_identity();
                    let tpe = Type::from_ty(ty, tyctx, &method_instance);
                    return vec![CILOp::LDStaticField(
                        crate::cil_op::StaticFieldDescriptor::boxed(None, tpe, symbol_name),
                    )];
                }
                _ => todo!("Unhandled global alloc {global_alloc:?}"),
            }
            //panic!("alloc_id:{alloc_id:?}")
        }
    };
    let tpe = Type::from_ty(scalar_type, tyctx, &method_instance);
    match scalar_type.kind() {
        TyKind::Int(int_type) => load_const_int(scalar_u128, int_type),
        TyKind::Uint(uint_type) => load_const_uint(scalar_u128, uint_type),
        TyKind::Float(ftype) => load_const_float(scalar_u128, ftype, tyctx),
        TyKind::Bool => vec![CILOp::LdcI32(scalar_u128 as i32)],
        TyKind::RawPtr(_) => {
            let value = i64::from_ne_bytes((scalar_u128 as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value)]
        }
        TyKind::Adt(adt_def, _subst) => match adt_def.adt_kind() {
            AdtKind::Enum => {
                let field_type = Type::U8;
                let enum_dotnet = tpe.as_dotnet().expect("Enum scalar not an ADT!");
                vec![
                    CILOp::SizeOf(Box::new(tpe)),
                    CILOp::LocAlloc,
                    CILOp::Dup,
                    CILOp::LDIndRef,
                    CILOp::LdcI64(scalar_u128 as i64),
                    CILOp::STField(Box::new(crate::cil_op::FieldDescriptor::new(
                        enum_dotnet.clone(),
                        field_type,
                        "_tag".into(),
                    ))),
                    CILOp::LdObj(Box::new(enum_dotnet.into())),
                ]
            }
            _ => todo!("Can't load const ADT scalars of type {scalar_type:?}"),
        },
        TyKind::Char => {
            let value = i64::from_ne_bytes((scalar_u128 as u64).to_ne_bytes());
            vec![CILOp::LdcI64(value), CILOp::ConvU64(false)]
        }
        _ => todo!("Can't load scalar constants of type {scalar_type:?}!"),
    }
}
fn load_const_float(value: u128, int_type: &FloatTy, _tyctx: TyCtxt) -> Vec<CILOp> {
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
pub fn load_const_int(value: u128, int_type: &IntTy) -> Vec<CILOp> {
    match int_type {
        IntTy::I8 => {
            let value = i8::from_ne_bytes([value as u8]);
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvI8(false)]
        }
        IntTy::I16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvI16(false)]
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
            let low = (value & u128::from(u64::MAX)) as u64;
            let high = (value << 64) as u64;
            let low = i64::from_ne_bytes(low.to_ne_bytes());
            let high = i64::from_ne_bytes(high.to_ne_bytes());
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
pub fn load_const_uint(value: u128, int_type: &UintTy) -> Vec<CILOp> {
    match int_type {
        UintTy::U8 => {
            let value = i8::from_ne_bytes([value as u8]);
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvU8(false)]
        }
        UintTy::U16 => {
            let value = i16::from_ne_bytes((value as u16).to_ne_bytes());
            vec![CILOp::LdcI32(i32::from(value)), CILOp::ConvU16(false)]
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
            let low = (value & u128::from(u64::MAX)) as u64;
            let high = (value << 64) as u64;
            let low = i64::from_ne_bytes(low.to_ne_bytes());
            let high = i64::from_ne_bytes(high.to_ne_bytes());
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
