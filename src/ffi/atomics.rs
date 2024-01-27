use crate::{
    assembly::Assembly,
    cil::{CILOp, CallSite, FieldDescriptor},
    function_sig::FnSig,
    method::Method,
    r#type::{DotnetTypeRef, Type},
};
/* 
//TODO: propely mimic the semantics used in atomics!
pub fn add_atomics(asm: &mut Assembly) {
    add_atomic_load_acquire(asm);
    add_atomic_cxchgweak_acquire_acquire(asm);
}
fn add_atomic_load_acquire(asm: &mut Assembly) {
    let atomic_load_acquire_calls: Box<[_]> = asm
        .call_sites()
        .filter(|call_site| {
            call_site.signature().inputs().len() == 1
                && matches!(call_site.signature().inputs()[0], Type::Ptr(_))
                && call_site.name().contains("atomic_load_acquire")
        })
        .cloned()
        .collect();
    for call in atomic_load_acquire_calls.iter() {
        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Private,
            true,
            call.signature().clone(),
            call.name(),
            vec![],
        );
        match call.signature().output() {
            //Those can't be implemented using System.Threading.Interlocked, since there is no Read(Uintptr).
            Type::Ptr(_) => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndISize, CILOp::Ret]),
            Type::ISize => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndISize, CILOp::Ret]),
            Type::USize => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndISize, CILOp::Ret]),
            //Those can be implemented using System.Threading.Interlocked.
            Type::I64 => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI64, CILOp::Ret]),
            Type::U64 => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI64, CILOp::Ret]),
            //Those can't be implemented using System.Threading.Interlocked.
            Type::U8 => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI8, CILOp::Ret]),
            Type::I8 => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI8, CILOp::Ret]),
            Type::U32 => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI32, CILOp::Ret]),
            Type::I32 => method.set_ops(vec![CILOp::LDArg(0), CILOp::LDIndI32, CILOp::Ret]),
            _ => {
                eprintln!(
                    "Warning: `atomic_load_acquire` used with unsuported type({:?})!",
                    call.signature().output()
                );
                continue;
            }
        }
        asm.add_method(method);
    }
}
fn interlocked() -> DotnetTypeRef {
    DotnetTypeRef::new(Some("System.Threading"), "System.Threading.Interlocked")
        .with_valuetype(false)
}

fn add_atomic_cxchgweak_acquire_acquire(asm: &mut Assembly) {
    let atomic_cxchgweak_acquire_acquire_calls: Box<[_]> = asm
        .call_sites()
        .filter(|call_site| {
            call_site.signature().inputs().len() == 3
                && call_site
                    .name()
                    .contains("atomic_cxchgweak_acquire_acquire")
        })
        .cloned()
        .collect();
    for call in atomic_cxchgweak_acquire_acquire_calls.iter() {
        let ret_tuple = call.signature().output();
        let ret_tuple_dotnet = ret_tuple
            .as_dotnet()
            .expect("atomic_cxchgweak_acquire_acquire: return tuple invalid!");
        let mut method = Method::new(
            crate::access_modifier::AccessModifer::Private,
            true,
            call.signature().clone(),
            call.name(),
            vec![
                (None, call.signature().inputs()[1].clone()),
                (Some("return_tuple".into()), ret_tuple.clone()),
            ],
        );
        match call.signature().inputs()[1] {
            //Those can't be implemented using System.Threading.Interlocked, since there is no Read(Uintptr).
            Type::USize | Type::ISize | Type::Ptr(_) => {
                let call_sig = FnSig::new(
                    &[
                        Type::ManagedReference(Type::USize.into()),
                        Type::USize,
                        Type::USize,
                    ],
                    &Type::USize,
                );
                method.set_ops(vec![
                    CILOp::LDArg(0),
                    CILOp::LDArg(1),
                    CILOp::LDArg(2),
                    CILOp::Call(
                        CallSite::new(
                            Some(interlocked()),
                            "CompareExchange".into(),
                            call_sig,
                            true,
                        )
                        .into(),
                    ),
                    CILOp::STLoc(0),
                    CILOp::LDLocA(1),
                    CILOp::LDLoc(0),
                    CILOp::STField(Box::new(FieldDescriptor::new(
                        ret_tuple_dotnet.clone(),
                        call.signature().inputs()[1].clone(),
                        "Item1".into(),
                    ))),
                    CILOp::LDLocA(1),
                    CILOp::LDLoc(0),
                    CILOp::LDArg(1),
                    CILOp::Eq,
                    CILOp::Not,
                    CILOp::STField(Box::new(FieldDescriptor::new(
                        ret_tuple_dotnet,
                        Type::Bool,
                        "Item2".into(),
                    ))),
                    CILOp::LDLoc(1),
                    CILOp::Ret,
                ]);
            }
            Type::U32 | Type::I32 => {
                let call_sig = FnSig::new(
                    &[
                        Type::ManagedReference(Type::I32.into()),
                        Type::I32,
                        Type::I32,
                    ],
                    &Type::I32,
                );
                method.set_ops(vec![
                    CILOp::LDArg(0),
                    CILOp::LDArg(1),
                    CILOp::LDArg(2),
                    CILOp::Call(
                        CallSite::new(
                            Some(interlocked()),
                            "CompareExchange".into(),
                            call_sig,
                            true,
                        )
                        .into(),
                    ),
                    CILOp::STLoc(0),
                    CILOp::LDLocA(1),
                    CILOp::LDLoc(0),
                    CILOp::STField(Box::new(FieldDescriptor::new(
                        ret_tuple_dotnet.clone(),
                        call.signature().inputs()[1].clone(),
                        "Item1".into(),
                    ))),
                    CILOp::LDLocA(1),
                    CILOp::LDLoc(0),
                    CILOp::LDArg(1),
                    CILOp::Eq,
                    CILOp::Not,
                    CILOp::STField(Box::new(FieldDescriptor::new(
                        ret_tuple_dotnet,
                        Type::Bool,
                        "Item2".into(),
                    ))),
                    CILOp::LDLoc(1),
                    CILOp::Ret,
                ]);
            }
            Type::U64 | Type::I64 => {
                let call_sig = FnSig::new(
                    &[
                        Type::ManagedReference(Type::U64.into()),
                        Type::U64,
                        Type::U64,
                    ],
                    &Type::U64,
                );
                method.set_ops(vec![
                    CILOp::LDArg(0),
                    CILOp::LDArg(1),
                    CILOp::LDArg(2),
                    CILOp::Call(
                        CallSite::new(
                            Some(interlocked()),
                            "CompareExchange".into(),
                            call_sig,
                            true,
                        )
                        .into(),
                    ),
                    CILOp::STLoc(0),
                    CILOp::LDLocA(1),
                    CILOp::LDLoc(0),
                    CILOp::STField(Box::new(FieldDescriptor::new(
                        ret_tuple_dotnet.clone(),
                        call.signature().inputs()[1].clone(),
                        "Item1".into(),
                    ))),
                    CILOp::LDLocA(1),
                    CILOp::LDLoc(0),
                    CILOp::LDArg(1),
                    CILOp::Eq,
                    CILOp::Not,
                    CILOp::STField(Box::new(FieldDescriptor::new(
                        ret_tuple_dotnet,
                        Type::Bool,
                        "Item2".into(),
                    ))),
                    CILOp::LDLoc(1),
                    CILOp::Ret,
                ]);
            }
            //Type::ISize=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndISize,CILOp::Ret]),
            // Type::USize=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndISize,CILOp::Ret]),
            //Those can be implemented using System.Threading.Interlocked.
            //Type::I64=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndI64,CILOp::Ret]),
            // Type::U64=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndI64,CILOp::Ret]),
            //Those can't be implemented using System.Threading.Interlocked.
            //Type::U8=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndI8,CILOp::Ret]),
            // Type::I8=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndI8,CILOp::Ret]),
            //Type::U32=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndI32,CILOp::Ret]),
            //Type::I32=> method.set_ops(vec![CILOp::LDArg(0),CILOp::LDIndI32,CILOp::Ret]),
            _ => {
                eprintln!(
                    "Warning: `atomic_cxchgweak_acquire_acquire` used with unsuported type({:?})!",
                    call.signature().inputs()[1]
                );
                continue;
            }
        }
        asm.add_method(method);
    }
}
//TODO: implement cmp by using System.Threading.Interlocked.CompareExchange<T>(T, T, T)
*/