use cilly::{call, call_site::CallSite, cil_node::CILNode, ptr, sub, DotnetTypeRef, FnSig, Type};

pub fn interlocked_add(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    sub!(
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
                    [addr, addend.clone()]
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
                    [addr, addend.clone()]
                )
            }
            Type::USize | Type::ISize | Type::Ptr(_) => call!(
                CallSite::builtin(
                    "interlocked_add_usize".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                        Type::USize
                    ),
                    true
                ),
                [addr, addend.clone()]
            ),

            _ => todo!(),
        },
        addend
    )
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
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "interlocked_or_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [
                addr.cast_ptr(Type::ManagedReference(Box::new(Type::USize))),
                addend.cast_ptr(Type::USize)
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_xor(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::I32 => {
            call!(
                CallSite::builtin(
                    "atomic_xor_i32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I32)), Type::I32],
                        Type::I32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::I64 => {
            call!(
                CallSite::builtin(
                    "atomic_xor_i64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I64)), Type::I64],
                        Type::I64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 => {
            call!(
                CallSite::builtin(
                    "atomic_xor_u32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U64 => {
            call!(
                CallSite::builtin(
                    "atomic_xor_u64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize => call!(
            CallSite::builtin(
                "atomic_xor_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::ISize => call!(
            CallSite::builtin(
                "atomic_xor_isize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::ISize)), Type::ISize],
                    Type::ISize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_xor_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [
                addr.cast_ptr(Type::ManagedReference(Box::new(Type::USize))),
                addend.cast_ptr(Type::USize)
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_and(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::U64 | Type::I64 => {
            call!(
                CallSite::new(
                    Some(DotnetTypeRef::interlocked()),
                    "And".into(),
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
                    "And".into(),
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
                "interlocked_and_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "interlocked_and_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [
                addr.cast_ptr(Type::ManagedReference(Box::new(Type::USize))),
                addend.cast_ptr(Type::USize)
            ]
        )
        .cast_ptr(ptr!(*inner)),
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
pub fn interlocked_nand(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::I32 => {
            call!(
                CallSite::builtin(
                    "atomic_nand_i32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I32)), Type::I32],
                        Type::I32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::I64 => {
            call!(
                CallSite::builtin(
                    "atomic_nand_i64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I64)), Type::I64],
                        Type::I64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 => {
            call!(
                CallSite::builtin(
                    "atomic_nand_u32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U64 => {
            call!(
                CallSite::builtin(
                    "atomic_nand_u64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize => call!(
            CallSite::builtin(
                "atomic_nand_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::ISize => call!(
            CallSite::builtin(
                "atomic_nand_isize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::ISize)), Type::ISize],
                    Type::ISize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_nand_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [
                addr.cast_ptr(Type::ManagedReference(Box::new(Type::USize))),
                addend.cast_ptr(Type::USize)
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_min(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::I32 => {
            call!(
                CallSite::builtin(
                    "atomic_min_i32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I32)), Type::I32],
                        Type::I32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::I64 => {
            call!(
                CallSite::builtin(
                    "atomic_min_i64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I64)), Type::I64],
                        Type::I64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 => {
            call!(
                CallSite::builtin(
                    "atomic_min_u32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U64 => {
            call!(
                CallSite::builtin(
                    "atomic_min_u64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize => call!(
            CallSite::builtin(
                "atomic_min_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::ISize => call!(
            CallSite::builtin(
                "atomic_min_isize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::ISize)), Type::ISize],
                    Type::ISize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_min_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [
                addr.cast_ptr(Type::ManagedReference(Box::new(Type::USize))),
                addend.cast_ptr(Type::USize)
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_max(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::I32 => {
            call!(
                CallSite::builtin(
                    "atomic_max_i32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I32)), Type::I32],
                        Type::I32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::I64 => {
            call!(
                CallSite::builtin(
                    "atomic_max_i64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::I64)), Type::I64],
                        Type::I64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U32 => {
            call!(
                CallSite::builtin(
                    "atomic_max_u32".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U32)), Type::U32],
                        Type::U32
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::U64 => {
            call!(
                CallSite::builtin(
                    "atomic_max_u64".into(),
                    FnSig::new(
                        &[Type::ManagedReference(Box::new(Type::U64)), Type::U64],
                        Type::U64
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::USize => call!(
            CallSite::builtin(
                "atomic_max_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::ISize => call!(
            CallSite::builtin(
                "atomic_max_isize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::ISize)), Type::ISize],
                    Type::ISize
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_max_usize".into(),
                FnSig::new(
                    &[Type::ManagedReference(Box::new(Type::USize)), Type::USize],
                    Type::USize
                ),
                true
            ),
            [
                addr.cast_ptr(Type::ManagedReference(Box::new(Type::USize))),
                addend.cast_ptr(Type::USize)
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
