use rustc_middle::mir::{BinOp, Operand};
use rustc_middle::ty::{Instance, TyCtxt};

use crate::{
    cil::{CILOp, CallSite, FieldDescriptor},
    function_sig::FnSig,
    r#type::{DotnetTypeRef, TyCache, Type},
};
/// Preforms an checked binary operation.
pub(crate) fn binop_checked<'tyctx>(
    binop: BinOp,
    operand_a: &Operand<'tyctx>,
    operand_b: &Operand<'tyctx>,
    tyctx: TyCtxt<'tyctx>,
    method: &rustc_middle::mir::Body<'tyctx>,
    method_instance: Instance<'tyctx>,
    cache: &mut TyCache,
) -> Vec<CILOp> {
    let ops_a = crate::operand::handle_operand(operand_a, tyctx, method, method_instance, cache);
    let ops_b = crate::operand::handle_operand(operand_b, tyctx, method, method_instance, cache);
    let ty_a = operand_a.ty(&method.local_decls, tyctx);
    let ty_a = crate::utilis::monomorphize(&method_instance, ty_a, tyctx);
    let ty_b = operand_b.ty(&method.local_decls, tyctx);
    let ty_b = crate::utilis::monomorphize(&method_instance, ty_b, tyctx);
    assert_eq!(ty_a, ty_b);
    let ty = cache.type_from_cache(ty_a, tyctx, Some(method_instance));
    match binop {
        BinOp::Mul | BinOp::MulUnchecked => [ops_a, ops_b, mul(ty)].into_iter().flatten().collect(),
        BinOp::Add => [ops_a, ops_b, add(ty).to_vec()]
            .into_iter()
            .flatten()
            .collect(),
        BinOp::Sub => [ops_a, ops_b, sub(ty).to_vec()]
            .into_iter()
            .flatten()
            .collect(),
        _ => todo!("Can't preform checked op {binop:?}"),
    }
}
fn mul(tpe: Type) -> Vec<CILOp> {
    match tpe {
        Type::U8 => promoted_ubinop(
            Type::U8,
            Type::U16,
            CILOp::ConvU16(false),
            CILOp::ConvU8(false),
            CILOp::LdcI32(u8::MAX as i32),
            CILOp::Mul,
        )
        .into(),
        Type::I8 => promoted_sbinop(
            Type::I8,
            Type::I16,
            CILOp::ConvI16(false),
            CILOp::ConvI8(false),
            CILOp::LdcI32(i8::MAX as i32),
            CILOp::LdcI32(i8::MIN as i32),
            CILOp::Mul,
        )
        .into(),
        Type::U16 => promoted_ubinop(
            Type::U16,
            Type::U32,
            CILOp::ConvU32(false),
            CILOp::ConvU16(false),
            CILOp::LdcI32(u16::MAX as i32),
            CILOp::Mul,
        )
        .into(),
        Type::I16 => promoted_sbinop(
            Type::I16,
            Type::I32,
            CILOp::ConvI32(false),
            CILOp::ConvI16(false),
            CILOp::LdcI32(i16::MAX as i32),
            CILOp::LdcI32(i16::MIN as i32),
            CILOp::Mul,
        )
        .into(),
        Type::U32 => promoted_ubinop(
            Type::U32,
            Type::U64,
            CILOp::ConvU64(false),
            CILOp::ConvU32(false),
            CILOp::LdcI64(u32::MAX as i64),
            CILOp::Mul,
        )
        .into(),
        Type::I32 => promoted_sbinop(
            Type::I32,
            Type::I64,
            CILOp::ConvI64(false),
            CILOp::ConvI32(false),
            CILOp::LdcI32(i32::MAX),
            CILOp::LdcI32(i32::MIN),
            CILOp::Mul,
        )
        .into(),
        Type::U64 => promoted_ubinop(
            Type::U64,
            Type::U128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::U64], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::U128], &Type::U64),
                true,
            )),
            CILOp::LdcI64(u64::MAX as i64),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Multiplication".into(),
                crate::function_sig::FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            )),
        )
        .into(),
        Type::I64 => promoted_sbinop(
            Type::I64,
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::I64], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::I128], &Type::I64),
                true,
            )),
            CILOp::LdcI64(i64::MAX),
            CILOp::LdcI64(i64::MIN),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Multiplication".into(),
                crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
        )
        .into(),
        Type::I128 => {
            eprintln!("WARING: checked 128 bit arthmetics ARE NOT SUPPOPRTED YET. Using unchecked variants, bugs may occur.");
            let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
            let tuple_ty: Type = tuple.clone().into();
            vec![
                CILOp::Call(
                    CallSite::new(
                        Some(DotnetTypeRef::int_128()),
                        "op_Multiplication".into(),
                        crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                        true,
                    )
                    .into(),
                ),
                CILOp::NewTMPLocal(tpe.clone().into()),
                CILOp::SetTMPLocal,
                CILOp::NewTMPLocal(tuple_ty.into()),
                CILOp::LoadAddresOfTMPLocal,
                CILOp::LoadUnderTMPLocal(1),
                CILOp::STField(FieldDescriptor::boxed(
                    tuple.clone(),
                    Type::I128,
                    "Item1".into(),
                )),
                CILOp::LoadAddresOfTMPLocal,
                CILOp::LdcI32(0),
                CILOp::STField(FieldDescriptor::boxed(
                    tuple.clone(),
                    Type::Bool,
                    "Item2".into(),
                )),
                CILOp::LoadTMPLocal,
                CILOp::FreeTMPLocal,
            ]
        }
        Type::U128 => {
            eprintln!("WARING: checked 128 bit arthmetics ARE NOT SUPPOPRTED YET. Using unchecked variants, bugs may occur.");
            let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
            let tuple_ty: Type = tuple.clone().into();
            vec![
                CILOp::Call(
                    CallSite::new(
                        Some(DotnetTypeRef::uint_128()),
                        "op_Multiplication".into(),
                        crate::function_sig::FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                        true,
                    )
                    .into(),
                ),
                CILOp::NewTMPLocal(tpe.clone().into()),
                CILOp::SetTMPLocal,
                CILOp::NewTMPLocal(tuple_ty.into()),
                CILOp::LoadAddresOfTMPLocal,
                CILOp::LoadUnderTMPLocal(1),
                CILOp::STField(FieldDescriptor::boxed(
                    tuple.clone(),
                    Type::U128,
                    "Item1".into(),
                )),
                CILOp::LoadAddresOfTMPLocal,
                CILOp::LdcI32(0),
                CILOp::STField(FieldDescriptor::boxed(
                    tuple.clone(),
                    Type::Bool,
                    "Item2".into(),
                )),
                CILOp::LoadTMPLocal,
                CILOp::FreeTMPLocal,
            ]
        }
        Type::USize => promoted_ubinop(
            Type::USize,
            Type::U128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::USize], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::U128], &Type::USize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::usize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Multiplication".into(),
                crate::function_sig::FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            )),
        )
        .into(),

        Type::ISize => promoted_sbinop(
            Type::ISize,
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::ISize], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::I128], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::isize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::isize_type()),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Multiplication".into(),
                crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
        )
        .into(),
        _ => todo!("Can't preform checked mul on type {tpe:?} yet!"),
    }
}
fn add(tpe: Type) -> Box<[CILOp]> {
    match tpe {
        Type::I8 => promoted_sbinop(
            Type::I8,
            Type::I16,
            CILOp::ConvI16(false),
            CILOp::ConvI8(false),
            CILOp::LdcI32(i8::MAX as i32),
            CILOp::LdcI32(i8::MIN as i32),
            CILOp::Add,
        )
        .into(),
        Type::U8 => checked_uadd_type(Type::U8, CILOp::ConvU8(false), CILOp::Add).into(),
        Type::I16 => promoted_sbinop(
            Type::I16,
            Type::I32,
            CILOp::ConvI32(false),
            CILOp::ConvI16(false),
            CILOp::LdcI32(i16::MAX as i32),
            CILOp::LdcI32(i16::MIN as i32),
            CILOp::Add,
        )
        .into(),
        Type::U16 => checked_uadd_type(Type::U16, CILOp::ConvU16(false), CILOp::Add).into(),
        Type::I32 => promoted_sbinop(
            Type::I32,
            Type::I64,
            CILOp::ConvI64(false),
            CILOp::ConvI32(false),
            CILOp::LdcI32(i32::MAX),
            CILOp::LdcI32(i32::MIN),
            CILOp::Add,
        )
        .into(),
        Type::U32 => checked_uadd_type(Type::U32, CILOp::Nop, CILOp::Add).into(),
        //This works ONLY in dotnet.
        Type::I64 => promoted_sbinop(
            Type::I64,
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::I64], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::I128], &Type::I64),
                true,
            )),
            CILOp::LdcI64(i64::MAX),
            CILOp::LdcI64(i64::MIN),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Addition".into(),
                crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
        )
        .into(),
        Type::U64 => checked_uadd_type(Type::U64, CILOp::Nop, CILOp::Add).into(),
        Type::U128 => checked_uadd_type(
            Type::U128,
            CILOp::Nop,
            CILOp::Call(
                CallSite::new(
                    Some(DotnetTypeRef::uint_128()),
                    "op_Addition".into(),
                    crate::function_sig::FnSig::new(
                        &[
                            DotnetTypeRef::uint_128().into(),
                            DotnetTypeRef::uint_128().into(),
                        ],
                        &DotnetTypeRef::uint_128().into(),
                    ),
                    true,
                )
                .into(),
            ),
        )
        .into(),

        Type::USize => promoted_ubinop(
            Type::USize,
            Type::U128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::USize], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::U128], &Type::USize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::usize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Addition".into(),
                crate::function_sig::FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            )),
        )
        .into(),
        Type::ISize => promoted_sbinop(
            Type::ISize,
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::ISize], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::I128], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::isize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::isize_type()),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Addition".into(),
                crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
        )
        .into(),
        _ => todo!("Can't preform checked add on type {tpe:?} yet!"),
    }
}
fn checked_uadd_type(tpe: Type, truncate: CILOp, add: CILOp) -> Vec<CILOp> {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    let tuple_ty = tuple.clone().into();
    vec![
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        add,
        truncate,
        CILOp::Dup,
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::LoadUnderTMPLocal(2),
        CILOp::Or,
        CILOp::Lt,
        CILOp::NewTMPLocal(Type::Bool.into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(Box::new(tuple_ty)),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::Bool,
            "Item2".into(),
        )),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(2),
        CILOp::STField(FieldDescriptor::boxed(tuple.clone(), tpe, "Item1".into())),
        CILOp::LoadTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]
}
/*
/// Checked signed add type
fn checked_sadd_type(tpe: Type, truncate: CILOp, mask: CILOp) -> Vec<CILOp> {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    let tuple_ty: Type = tuple.clone().into();
    vec![
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::Add,
        truncate.clone(),
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        mask.clone(),
        CILOp::And,
        CILOp::Dup,
        CILOp::LoadUnderTMPLocal(2),
        mask.clone(),
        CILOp::And,
        CILOp::Eq,
        truncate.clone(),
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        mask.clone(),
        CILOp::And,
        CILOp::Eq,
        CILOp::Not,
        CILOp::LoadTMPLocal,
        CILOp::And,
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(tuple_ty.into()),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::GenericArg(1),
            "Item2".into(),
        )),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(3),
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::GenericArg(0),
            "Item1".into(),
        )),
        CILOp::LoadTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]
}*/
fn sub(tpe: Type) -> Box<[CILOp]> {
    match tpe {
        Type::I8 => promoted_sbinop(
            Type::I8,
            Type::I16,
            CILOp::ConvI16(false),
            CILOp::ConvI8(false),
            CILOp::LdcI32(i8::MAX as i32),
            CILOp::LdcI32(i8::MIN as i32),
            CILOp::Sub,
        )
        .into(),
        Type::U8 => promoted_ubinop(
            Type::U8,
            Type::U16,
            CILOp::ConvU16(false),
            CILOp::ConvU8(false),
            CILOp::LdcI32(u8::MAX as i32),
            CILOp::Sub,
        )
        .into(),
        Type::I16 => promoted_sbinop(
            Type::I16,
            Type::I32,
            CILOp::ConvI32(false),
            CILOp::ConvI16(false),
            CILOp::LdcI32(i16::MAX as i32),
            CILOp::LdcI32(i16::MIN as i32),
            CILOp::Sub,
        )
        .into(),
        Type::U16 => promoted_ubinop(
            Type::U16,
            Type::U32,
            CILOp::ConvU32(false),
            CILOp::ConvU16(false),
            CILOp::LdcI32(u16::MAX as i32),
            CILOp::Sub,
        )
        .into(),
        Type::I32 => promoted_sbinop(
            Type::I32,
            Type::I64,
            CILOp::ConvI64(false),
            CILOp::ConvI32(false),
            CILOp::LdcI32(i32::MAX),
            CILOp::LdcI32(i32::MIN),
            CILOp::Sub,
        )
        .into(),
        Type::U32 => promoted_ubinop(
            Type::U32,
            Type::U64,
            CILOp::ConvU64(false),
            CILOp::ConvU32(false),
            CILOp::LdcI64(u64::MAX as i64),
            CILOp::Sub,
        )
        .into(),
        //This works ONLY in dotnet.
        Type::I64 => promoted_sbinop(
            Type::I64,
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::I64], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::I128], &Type::I64),
                true,
            )),
            CILOp::LdcI64(i64::MAX),
            CILOp::LdcI64(i64::MIN),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Subtraction".into(),
                crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
        )
        .into(),
        Type::U64 => promoted_ubinop(
            Type::U64,
            Type::U128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::U64], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::U128], &Type::U64),
                true,
            )),
            CILOp::LdcI64(i64::MIN),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Subtraction".into(),
                crate::function_sig::FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            )),
        )
        .into(),
        Type::USize => promoted_ubinop(
            Type::USize,
            Type::U128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::USize], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::U128], &Type::USize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::usize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::USize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Subtraction".into(),
                crate::function_sig::FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            )),
        )
        .into(),
        Type::ISize => promoted_sbinop(
            Type::ISize,
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                crate::function_sig::FnSig::new(&[Type::ISize], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Explicit".into(),
                crate::function_sig::FnSig::new(&[Type::I128], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::isize_type()),
                "get_MaxValue".into(),
                FnSig::new(&[], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::isize_type()),
                "get_MinValue".into(),
                FnSig::new(&[], &Type::ISize),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Subtraction".into(),
                crate::function_sig::FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
        )
        .into(),
        Type::I128 => checked_sub_128bit(
            Type::I128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_Subtraction".into(),
                FnSig::new(&[Type::I128, Type::I128], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
            Some(DotnetTypeRef::int_128()),
                "op_Implicit".into(),
                FnSig::new(&[Type::I32], &Type::I128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_GreaterThan".into(),
                FnSig::new(&[Type::I128, Type::I128], &Type::Bool),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::int_128()),
                "op_LessThan".into(),
                FnSig::new(&[Type::I128, Type::I128], &Type::Bool),
                true,
            )),
        ).into(),
        Type::U128 => checked_sub_128bit(
            Type::U128,
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_Subtraction".into(),
                FnSig::new(&[Type::U128, Type::U128], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
            Some(DotnetTypeRef::uint_128()),
                "op_Implicit".into(),
                FnSig::new(&[Type::I32], &Type::U128),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_GreaterThan".into(),
                FnSig::new(&[Type::U128, Type::U128], &Type::Bool),
                true,
            )),
            CILOp::Call(CallSite::boxed(
                Some(DotnetTypeRef::uint_128()),
                "op_LessThan".into(),
                FnSig::new(&[Type::U128, Type::U128], &Type::Bool),
                true,
            )),
        ).into(),
        _ => todo!("Can't preform checked sub on type {tpe:?} yet!"),
    }
}
#[test]
fn unsigned_add() {
    //u8
    for a in 0..u8::MAX {
        for b in 0..u8::MAX {
            let added = u8::checked_add(a, b);
            let alg_added = {
                let sum = u8::wrapping_add(a, b);
                if sum < a | b {
                    None
                } else {
                    Some(sum)
                }
            };
            assert_eq!(added, alg_added, "checked {a} + {b}");
        }
    }
}
#[test]
fn signed_add() {
    //u8
    for a in i8::MIN..i8::MAX {
        for b in i8::MIN..i8::MAX {
            let added = i8::checked_add(a, b);
            let alg_added = {
                let sum = i8::wrapping_add(a, b);
                let sign_a = a & (0b1000_0000_u8 as i8);
                let sign_b = b & (0b1000_0000_u8 as i8);
                let sum_sign = sum & (0b1000_0000_u8 as i8);
                let signs_equal = sign_a == sign_b;
                if signs_equal && sum_sign != sign_a {
                    None
                } else {
                    Some(sum)
                }
            };
            assert_eq!(added, alg_added, "checked {a} + {b}");
        }
    }
}
pub fn promoted_ubinop(
    tpe: Type,
    promoted_type: Type,
    promote: CILOp,
    truncate: CILOp,
    omask: CILOp,
    binop: CILOp,
) -> [CILOp; 27] {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    let tuple_ty: Type = tuple.clone().into();
    [
        // Promote arguments
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        promote.clone(),
        CILOp::LoadTMPLocal,
        promote.clone(),
        // Preform binop
        binop.clone(),
        // Save the promoted result of binop
        CILOp::NewTMPLocal(promoted_type.clone().into()),
        CILOp::SetTMPLocal,
        // Compare the result to the overflow mask
        CILOp::LoadTMPLocal,
        omask.clone(),
        promote.clone(),
        CILOp::Gt,
        // Save the bollean indicating overflow
        CILOp::NewTMPLocal(Type::Bool.into()),
        CILOp::SetTMPLocal,
        // Create result tuple type
        CILOp::NewTMPLocal(tuple_ty.into()),
        // Set the tuples second field to overflow flag
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1), // ov
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::Bool,
            "Item2".into(),
        )),
        // Set the tuples first field to promotion result
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(2),
        truncate,
        CILOp::STField(FieldDescriptor::boxed(tuple.clone(), tpe, "Item1".into())),
        // Load results
        CILOp::LoadTMPLocal,
        // Reset temporary local statck.
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]
}
pub fn promoted_sbinop(
    tpe: Type,
    promoted_type: Type,
    promote: CILOp,
    truncate: CILOp,
    overflow_val: CILOp,
    underflow_val: CILOp,
    binop: CILOp,
) -> [CILOp; 32] {
    let tuple = crate::r#type::simple_tuple(&[tpe.clone(), Type::Bool]);
    let tuple_ty: Type = tuple.clone().into();
    [
        // Promote arguments
        CILOp::NewTMPLocal(tpe.clone().into()),
        CILOp::SetTMPLocal,
        promote.clone(),
        CILOp::LoadTMPLocal,
        promote.clone(),
        // Preform binop
        binop.clone(),
        // Save the promoted result of binop
        CILOp::NewTMPLocal(promoted_type.clone().into()),
        CILOp::SetTMPLocal,
        // Compare the result to the overflow mask
        CILOp::LoadTMPLocal,
        overflow_val.clone(),
        promote.clone(),
        CILOp::Gt,
        CILOp::LoadTMPLocal,
        // Compare the result to the undeflow mask
        underflow_val.clone(),
        promote.clone(),
        CILOp::Lt,
        CILOp::Or,
        // Save the bollean indicating overflow
        CILOp::NewTMPLocal(Type::Bool.into()),
        CILOp::SetTMPLocal,
        // Create result tuple type
        CILOp::NewTMPLocal(tuple_ty.into()),
        // Set the tuples second field to overflow flag
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1), // ov
        CILOp::STField(FieldDescriptor::boxed(
            tuple.clone(),
            Type::Bool,
            "Item2".into(),
        )),
        // Set the tuples first field to promotion result
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(2),
        truncate,
        CILOp::STField(FieldDescriptor::boxed(tuple.clone(), tpe, "Item1".into())),
        // Load results
        CILOp::LoadTMPLocal,
        // Reset temporary local statck.
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]
}
fn checked_sub_128bit(
    int_type: Type,
    subtraction: CILOp,
    promote_i32: CILOp,
    greater_than: CILOp,
    less_than: CILOp,
) -> [CILOp; 56] {
    let tuple_type = crate::r#type::simple_tuple(&[int_type.clone(), Type::Bool]);

    //call bool [System.Runtime]System.Int128::op_GreaterThan(valuetype [System.Runtime]System.Int128, valuetype [System.Runtime]System.Int128)
    //call bool [System.Runtime]System.Int128::op_LessThan(valuetype [System.Runtime]System.Int128, valuetype [System.Runtime]System.Int128)
    let bool_field = FieldDescriptor::new(tuple_type.clone(), Type::Bool, "Item2".into());
    let int_field = FieldDescriptor::new(tuple_type.clone(), int_type.clone(), "Item1".into());
    [
        CILOp::NewTMPLocal(int_type.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::NewTMPLocal(int_type.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::LoadTMPLocal,
        subtraction,
        CILOp::NewTMPLocal(int_type.clone().into()),
        CILOp::SetTMPLocal,
        CILOp::LoadUnderTMPLocal(2),
        CILOp::LdcI32(0),
        promote_i32.clone(),
        greater_than.clone(),
        CILOp::LoadUnderTMPLocal(1),
        CILOp::LdcI32(0),
        promote_i32.clone(),
        less_than.clone(),
        CILOp::And,
        CILOp::LoadTMPLocal,
        CILOp::LdcI32(0),
        promote_i32.clone(),
        less_than.clone(),
        CILOp::And,
        CILOp::LoadUnderTMPLocal(2),
        CILOp::LdcI32(0),
        promote_i32.clone(),
        less_than,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::LdcI32(0),
        promote_i32.clone(),
        greater_than.clone(),
        CILOp::And,
        CILOp::LoadTMPLocal,
        CILOp::LdcI32(0),
        promote_i32,
        greater_than,
        CILOp::And,
        CILOp::Or,
        CILOp::NewTMPLocal(Type::Bool.into()),
        CILOp::SetTMPLocal,
        // Set the bool part
        CILOp::NewTMPLocal(Box::new(tuple_type.clone().into())),
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::STField(bool_field.clone().into()),
        // Set the bool part
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(1),
        CILOp::STField(bool_field.clone().into()),
        // Set the bool part
        CILOp::LoadAddresOfTMPLocal,
        CILOp::LoadUnderTMPLocal(2),
        CILOp::STField(int_field.clone().into()),
        CILOp::LoadTMPLocal,
        //Tuple
        CILOp::FreeTMPLocal,
        // Result and flag
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
        // Arguments
        CILOp::FreeTMPLocal,
        CILOp::FreeTMPLocal,
    ]
}
