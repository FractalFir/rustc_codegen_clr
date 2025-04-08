use cilly::{
    call,
    cil_node::CILNode,
    cilnode::MethodKind,
    MethodRef, Type, {Assembly, ClassRef, Int},
};

pub fn atomic_add(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(int) => {
            let u64_ref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_add_{int}", int = int.name())),
                asm.sig([u64_ref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }

        Type::Ptr(_) => {
            let usize_ref = asm.nref(Type::Int(Int::USize));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("atomic_add_usize"),
                asm.sig([usize_ref, Type::Int(Int::USize)], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(usize_ref),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            ).cast_ptr(tpe)
        }

        _ => todo!(),
    }
}
pub fn atomic_or(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::U64 | Int::I64) => {
            let u64_ref = asm.nref(Type::Int(Int::U64));
            let mref = MethodRef::new(
                ClassRef::interlocked(asm),
                asm.alloc_string("Or"),
                asm.sig([u64_ref, Type::Int(Int::U64)], Type::Int(Int::U64)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [addr, addend])
        }
        Type::Int(Int::U32 | Int::I32) => {
            let u32_ref = asm.nref(Type::Int(Int::U32));
            let mref = MethodRef::new(
                ClassRef::interlocked(asm),
                asm.alloc_string("Or"),
                asm.sig([u32_ref, Type::Int(Int::U32)], Type::Int(Int::U32)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [addr, addend])
        }
        Type::Int(Int::ISize | Int::USize | Int::U8 | Int::I8) | Type::Bool => {
            let int_ref = asm.nref(tpe);
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_or_{}", tpe.mangle(asm))),
                asm.sig([int_ref, tpe], tpe),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }

        Type::Ptr(inner) => {
            let int = Int::USize;
            let int_ref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_or_{}", int.name())),
                asm.sig([int_ref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            );
            cilnode.cast_ptr(Type::Ptr(inner)).cast_ptr(tpe)
        }
        _ => todo!("Can't atomic or {tpe:?}"),
    }
}
pub fn atomic_xor(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Bool
        | Type::Int(
            Int::U8 | Int::I8 | Int::U32 | Int::I32 | Int::U64 | Int::I64 | Int::USize | Int::ISize,
        ) => {
            let iref = asm.nref(tpe);
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_xor_{}", tpe.mangle(asm))),
                asm.sig([iref, tpe], tpe),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }

        Type::Ptr(inner) => {
            let int = Int::USize;
            let iref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_xor_{}", int.name())),
                asm.sig([iref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            )
            .cast_ptr(Type::Ptr(inner))
        }
        _ => todo!("Can't atomic xor {tpe:?}"),
    }
}
pub fn atomic_and(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(Int::U64 | Int::I64) => {
            let u64_ref = asm.nref(Type::Int(Int::U64));
            let mref = MethodRef::new(
                ClassRef::interlocked(asm),
                asm.alloc_string("And"),
                asm.sig([u64_ref, Type::Int(Int::U64)], Type::Int(Int::U64)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        Type::Int(Int::U32 | Int::I32) => {
            let u32_ref = asm.nref(Type::Int(Int::U32));
            let mref = MethodRef::new(
                ClassRef::interlocked(asm),
                asm.alloc_string("And"),
                asm.sig([u32_ref, Type::Int(Int::U32)], Type::Int(Int::U32)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        Type::Int(Int::USize | Int::ISize) => {
            let usize_ref = asm.nref(Type::Int(Int::USize));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("atomic_and_usize"),
                asm.sig([usize_ref, Type::Int(Int::USize)], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        Type::Ptr(inner) => {
            let usize_ref = asm.nref(Type::Int(Int::USize));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("atomic_and_usize"),
                asm.sig([usize_ref, Type::Int(Int::USize)], Type::Int(Int::USize)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            );
            cilnode.cast_ptr(Type::Ptr(inner))
        }
        Type::Bool | Type::Int(Int::U8 | Int::I8) => {
            let iref = asm.nref(tpe);
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_and_{}", tpe.mangle(asm))),
                asm.sig([iref, tpe], tpe),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        _ => todo!("Can't atomic and {tpe:?}"),
    }
}
pub fn compare_bytes(a: CILNode, b: CILNode, len: CILNode, asm: &mut Assembly) -> CILNode {
    let u8_ref = asm.nptr(Type::Int(Int::U8));
    let mref = MethodRef::new(
        *asm.main_module(),
        asm.alloc_string("memcmp"),
        asm.sig([u8_ref, u8_ref, Type::Int(Int::USize)], Type::Int(Int::I32)),
        MethodKind::Static,
        vec![].into(),
    );
    call!(asm.alloc_methodref(mref), [a, b, len])
}
pub fn atomic_nand(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Int(int @ (Int::U32 | Int::I32 | Int::U64 | Int::I64 | Int::USize | Int::ISize)) => {
            let iref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_nand_{}", int.name())),
                asm.sig([iref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        Type::Ptr(inner) => {
            let int = Int::USize;
            let iref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_nand_{}", int.name())),
                asm.sig([iref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            )
            .cast_ptr(Type::Ptr(inner))
        }
        Type::Bool | Type::Int(Int::U8 | Int::I8) => {
            let iref = asm.nref(tpe);
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_nand_{}", tpe.mangle(asm))),
                asm.sig([iref, tpe], tpe),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        _ => todo!("Can't atomic nand {tpe:?}"),
    }
}
pub fn atomic_min(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Bool
        | Type::Int(
            Int::U8 | Int::I8 | Int::U32 | Int::I32 | Int::U64 | Int::I64 | Int::USize | Int::ISize,
        ) => {
            let iref = asm.nref(tpe);
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_min_{}", tpe.mangle(asm))),
                asm.sig([iref, tpe], tpe),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        Type::Ptr(inner) => {
            let int = Int::USize;
            let iref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_min_{}", int.name())),
                asm.sig([iref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            )
            .cast_ptr(Type::Ptr(inner))
        }
        _ => todo!("Can't atomic min {tpe:?}"),
    }
}
pub fn atomic_max(addr: CILNode, addend: CILNode, tpe: Type, asm: &mut Assembly) -> CILNode {
    match tpe {
        Type::Bool
        | Type::Int(
            Int::U8 | Int::I8 | Int::U32 | Int::I32 | Int::U64 | Int::I64 | Int::USize | Int::ISize,
        ) => {
            let iref = asm.nref(tpe);
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_max_{}", tpe.mangle(asm))),
                asm.sig([iref, tpe], tpe),
                MethodKind::Static,
                vec![].into(),
            );
            let cilnode = call!(asm.alloc_methodref(mref), [addr, addend]);
            cilnode
        }
        Type::Ptr(inner) => {
            let int = Int::USize;
            let iref = asm.nref(Type::Int(int));
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string(format!("atomic_max_{}", int.name())),
                asm.sig([iref, Type::Int(int)], Type::Int(int)),
                MethodKind::Static,
                vec![].into(),
            );
            call!(
                asm.alloc_methodref(mref),
                [
                    addr.cast_ptr(asm.nref(Type::Int(Int::USize))),
                    addend.cast_ptr(Type::Int(Int::USize))
                ]
            )
            .cast_ptr(Type::Ptr(inner))
        }
        _ => todo!(),
    }
}
