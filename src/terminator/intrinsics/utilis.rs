use cilly::{call, call_site::CallSite, cil_node::CILNode, ptr, sub, ClassRef, FnSig, Type};

pub fn interlocked_add(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    sub!(
        match tpe {
            Type::Int(Int::U64) | Type::Int(Int::I64) => {
                call!(
                    CallSite::new(
                        Some(ClassRef::interlocked()),
                        "Add".into(),
                        FnSig::new(
                            &[
                                Type::Ref(Box::new(Type::Int(Int::U64))),
                                Type::Int(Int::U64)
                            ],
                            Type::Int(Int::U64)
                        ),
                        true
                    ),
                    [addr, addend.clone()]
                )
            }
            Type::Int(Int::U32) | Type::Int(Int::I32) => {
                call!(
                    CallSite::new(
                        Some(ClassRef::interlocked()),
                        "Add".into(),
                        FnSig::new(
                            &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
                            Type::Int(Int::U32)
                        ),
                        true
                    ),
                    [addr, addend.clone()]
                )
            }
            Type::Int(Int::USize) | Type::Int(Int::ISize) | Type::Ptr(_) => call!(
                CallSite::builtin(
                    "interlocked_add_usize".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::USize))),
                            Type::Int(Int::USize)
                        ],
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
pub fn interlocked_or(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::Int(Int::U64) | Type::Int(Int::I64) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked()),
                    "Or".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::U64))),
                            Type::Int(Int::U64)
                        ],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32) | Type::Int(Int::I32) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked()),
                    "Or".into(),
                    FnSig::new(
                        &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) | Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "interlocked_or_usize".into(),
                FnSig::new(
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "interlocked_or_usize".into(),
                FnSig::new(
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(Type::Ref(Box::new(Type::Int(Int::USize)))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_xor(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_xor_i32".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I32))),
                            Type::Int(Int::I32)
                        ],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I64))),
                            Type::Int(Int::I64)
                        ],
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
                        &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::U64))),
                            Type::Int(Int::U64)
                        ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::ISize))),
                        Type::Int(Int::ISize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(Type::Ref(Box::new(Type::Int(Int::USize)))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_and(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::Int(Int::U64) | Type::Int(Int::I64) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked()),
                    "And".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::U64))),
                            Type::Int(Int::U64)
                        ],
                        Type::Int(Int::U64)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::U32) | Type::Int(Int::I32) => {
            call!(
                CallSite::new(
                    Some(ClassRef::interlocked()),
                    "And".into(),
                    FnSig::new(
                        &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
                        Type::Int(Int::U32)
                    ),
                    true
                ),
                [addr, addend]
            )
        }
        Type::Int(Int::USize) | Type::Int(Int::ISize) => call!(
            CallSite::builtin(
                "interlocked_and_usize".into(),
                FnSig::new(
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [addr, addend]
        ),
        Type::Ptr(inner) => call!(
            CallSite::builtin(
                "interlocked_and_usize".into(),
                FnSig::new(
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(Type::Ref(Box::new(Type::Int(Int::USize)))),
                addend.cast_ptr(Type::Int(Int::USize))
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
                    Type::Ptr(Type::Int(Int::U8).into()),
                    Type::Ptr(Type::Int(Int::U8).into()),
                    Type::Int(Int::USize)
                ],
                Type::Int(Int::I32)
            ),
            true
        ),
        [a, b, len]
    )
}
pub fn interlocked_nand(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_nand_i32".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I32))),
                            Type::Int(Int::I32)
                        ],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I64))),
                            Type::Int(Int::I64)
                        ],
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
                        &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::U64))),
                            Type::Int(Int::U64)
                        ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::ISize))),
                        Type::Int(Int::ISize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(Type::Ref(Box::new(Type::Int(Int::USize)))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_min(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_min_i32".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I32))),
                            Type::Int(Int::I32)
                        ],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I64))),
                            Type::Int(Int::I64)
                        ],
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
                        &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::U64))),
                            Type::Int(Int::U64)
                        ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::ISize))),
                        Type::Int(Int::ISize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(Type::Ref(Box::new(Type::Int(Int::USize)))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
pub fn interlocked_max(addr: CILNode, addend: CILNode, tpe: Type) -> CILNode {
    match tpe {
        Type::Int(Int::I32) => {
            call!(
                CallSite::builtin(
                    "atomic_max_i32".into(),
                    FnSig::new(
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I32))),
                            Type::Int(Int::I32)
                        ],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::I64))),
                            Type::Int(Int::I64)
                        ],
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
                        &[Type::Ref(Box::new(Type::Int(Int::U32))), Type::Int(Int::U32)],
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
                        &[
                            Type::Ref(Box::new(Type::Int(Int::U64))),
                            Type::Int(Int::U64)
                        ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::ISize))),
                        Type::Int(Int::ISize)
                    ],
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
                    &[
                        Type::Ref(Box::new(Type::Int(Int::USize))),
                        Type::Int(Int::USize)
                    ],
                    Type::Int(Int::USize)
                ),
                true
            ),
            [
                addr.cast_ptr(Type::Ref(Box::new(Type::Int(Int::USize)))),
                addend.cast_ptr(Type::Int(Int::USize))
            ]
        )
        .cast_ptr(ptr!(*inner)),
        _ => todo!(),
    }
}
