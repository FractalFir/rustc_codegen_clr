use cilly::{
    call,
    call_site::CallSite,
    cil_node::CILNode,
    sub,
    v2::{Assembly, ClassRef, Int},
    FnSig, Type,
};

pub fn atomic_add(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    sub!(
        match tpe {
            Type::Int(Int::U64 | Int::I64) => {
                call!(
                    CallSite::new(
                        Some(ClassRef::interlocked(asm)),
                        "Add".into(),
                        FnSig::new(
                            [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                            Type::Int(Int::U64)
                        ),
                        true
                    ),
                    [addr, addend.clone()]
                )
            }
            Type::Int(Int::U32 | Int::I32) => {
                call!(
                    CallSite::new(
                        Some(ClassRef::interlocked(asm)),
                        "Add".into(),
                        FnSig::new(
                            [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                            Type::Int(Int::U32)
                        ),
                        true
                    ),
                    [addr, addend.clone()]
                )
            }
            Type::Int(Int::USize | Int::ISize) | Type::Ptr(_) => call!(
                CallSite::builtin(
                    "atomic_add_usize".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                        Type::Int(Int::USize)
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
pub fn atomic_or(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::U64 | Int::I64) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked(asm)),
                    "Or".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32 | Int::I32) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked(asm)),
                    "Or".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) => call!(
            CallSite::builtin(
                "atomic_or_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "atomic_or_isize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::ISize)), Type::Int(Int::ISize)],
                    Type::Int(Int::ISize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_or_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(Type::Ptr(inner)),
        _ => todo!(),
    }
}
pub fn atomic_xor(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_xor_i32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I32)), Type::Int(Int::I32)],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::I64) => {
            call!(
                CallSite::builtin(
                    "atomic_xor_i64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I64)), Type::Int(Int::I64)],
                        Type::Int(Int::I64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32) => {
            call!(
                CallSite::builtin(
                    "atomic_xor_u32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U64) => {
            call!(
                CallSite::builtin(
                    "atomic_xor_u64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) => call!(
            CallSite::builtin(
                "atomic_xor_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "atomic_xor_isize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::ISize)), Type::Int(Int::ISize)],
                    Type::Int(Int::ISize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_xor_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(Type::Ptr(inner)),
        _ => todo!(),
    }
}
pub fn atomic_and(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::U64 | Int::I64) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked(asm)),
                    "And".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32 | Int::I32) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked(asm)),
                    "And".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize | Int::ISize) => call!(
            CallSite::builtin(
                "atomic_and_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_and_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(Type::Ptr(inner)),
        _ => todo!(),
    }
}
pub fn compare_bytes(a: CILNode, b: CILNode, len: CILNode, asm: &mut Assembly) -> CILNode {
    call!(
        CallSite::builtin(
            "memcmp".into(),
            FnSig::new(
                [
                    asm.nptr(Type::Int(Int::U8)),
                    asm.nptr(Type::Int(Int::U8)),
                    Type::Int(Int::USize)
                ],
                Type::Int(Int::I32)
            ),
            true
        ),
        [a, b, len]
    )
}
pub fn atomic_nand(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_nand_i32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I32)), Type::Int(Int::I32)],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::I64) => {
            call!(
                CallSite::builtin(
                    "atomic_nand_i64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I64)), Type::Int(Int::I64)],
                        Type::Int(Int::I64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32) => {
            call!(
                CallSite::builtin(
                    "atomic_nand_u32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U64) => {
            call!(
                CallSite::builtin(
                    "atomic_nand_u64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) => call!(
            CallSite::builtin(
                "atomic_nand_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "atomic_nand_isize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::ISize)), Type::Int(Int::ISize)],
                    Type::Int(Int::ISize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_nand_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(Type::Ptr(inner)),
        _ => todo!(),
    }
}
pub fn atomic_min(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_min_i32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I32)), Type::Int(Int::I32)],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::I64) => {
            call!(
                CallSite::builtin(
                    "atomic_min_i64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I64)), Type::Int(Int::I64)],
                        Type::Int(Int::I64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32) => {
            call!(
                CallSite::builtin(
                    "atomic_min_u32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U64) => {
            call!(
                CallSite::builtin(
                    "atomic_min_u64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) => call!(
            CallSite::builtin(
                "atomic_min_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "atomic_min_isize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::ISize)), Type::Int(Int::ISize)],
                    Type::Int(Int::ISize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_min_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(Type::Ptr(inner)),
        _ => todo!(),
    }
}
pub fn atomic_max(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_max_i32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I32)), Type::Int(Int::I32)],
                        Type::Int(Int::I32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::I64) => {
            call!(
                CallSite::builtin(
                    "atomic_max_i64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::I64)), Type::Int(Int::I64)],
                        Type::Int(Int::I64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32) => {
            call!(
                CallSite::builtin(
                    "atomic_max_u32".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U32)), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U64) => {
            call!(
                CallSite::builtin(
                    "atomic_max_u64".into(),
                    FnSig::new(
                        [asm.nref(Type::Int(Int::U64)), Type::Int(Int::U64)],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) => call!(
            CallSite::builtin(
                "atomic_max_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "atomic_max_isize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::ISize)), Type::Int(Int::ISize)],
                    Type::Int(Int::ISize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "atomic_max_usize".into(),
                FnSig::new(
                    [asm.nref(Type::Int(Int::USize)), Type::Int(Int::USize)],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(Type::Ptr(inner)),
        _ => todo!(),
    }
}
