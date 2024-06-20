use cilly::{call, call_site::CallSite, cil_node::CILNode, DotnetTypeRef, FnSig, Type};

pub fn interlocked_add(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::U64 | Type::I64 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "Add".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 | Type::I32 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "Add".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize | Type::ISize => call!(
            CallSite::builtin(
                "interlocked_add_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        _ => todo!(),
    }
}
pub fn interlocked_or(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::U64 | Type::I64 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "Or".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 | Type::I32 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "Or".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize | Type::ISize => call!(
            CallSite::builtin(
                "interlocked_or_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        _ => todo!(),
    }
}
pub fn compare_bytes(a: CILNode, b: CILNode, len: CILNode) -> CILNode {
    call!(
        CallSite::builtin(
            "memcmp".into(),
            FnSig::new(
                &[
                    Type::Ptr(Type::U8.into()),
                    Type::Ptr(Type::U8.into()),
                    Type::USize
                ],
                Type::I32
            ),
            true
        ),
        [a, b, len]
    )
}
