use cilly::{
    call,
    cil_node::CILNode,
    eq, gt, gt_un, lt, lt_un,
    v2::{cilnode::MethodKind, Assembly, ClassRef, Float, Int, MethodRef},
    Type,
};
use rustc_middle::ty::{FloatTy, IntTy, Ty, TyKind, UintTy};

pub fn ne_unchecked(
    ty_a: Ty<'_>,
    operand_a: CILNode,
    operand_b: CILNode,
    asm: &mut Assembly,
) -> CILNode {
    //vec![eq_unchecked(ty_a), CILOp::LdcI32(0), CILOp::Eq]
    eq!(
        eq_unchecked(ty_a, operand_a, operand_b, asm),
        CILNode::V2(asm.alloc_node(false))
    )
}
pub fn eq_unchecked(
    ty_a: Ty<'_>,
    operand_a: CILNode,
    operand_b: CILNode,
    asm: &mut Assembly,
) -> CILNode {
    //vec![CILOp::Eq]
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => {
                let mref = MethodRef::new(
                    ClassRef::uint_128(asm),
                    asm.alloc_string("op_Equality"),
                    asm.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => {
                let mref = MethodRef::new(
                    ClassRef::int_128(asm),
                    asm.alloc_string("op_Equality"),
                    asm.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Bool
        | TyKind::Char
        | TyKind::Float(FloatTy::F32 | FloatTy::F64)
        | TyKind::RawPtr(_, _) => {
            eq!(operand_a, operand_b)
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("__eqtf2"),
                asm.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("eq_f16"),
                asm.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
pub fn lt_unchecked(
    ty_a: Ty<'_>,
    operand_a: CILNode,
    operand_b: CILNode,
    asm: &mut Assembly,
) -> CILNode {
    //return CILOp::Lt;
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => {
                let mref = MethodRef::new(
                    ClassRef::uint_128(asm),
                    asm.alloc_string("op_LessThan"),
                    asm.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => lt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => {
                let mref = MethodRef::new(
                    ClassRef::int_128(asm),
                    asm.alloc_string("op_LessThan"),
                    asm.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => lt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            lt!(operand_a, operand_b)
        }
        TyKind::RawPtr(_, _) | TyKind::FnPtr(_, _) => lt_un!(operand_a, operand_b),
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("__lttf2"),
                asm.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("lt_f16"),
                asm.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
pub fn gt_unchecked(
    ty_a: Ty<'_>,
    operand_a: CILNode,
    operand_b: CILNode,
    asm: &mut Assembly,
) -> CILNode {
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => {
                let mref = MethodRef::new(
                    ClassRef::uint_128(asm),
                    asm.alloc_string("op_GreaterThan"),
                    asm.sig([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => gt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => {
                let mref = MethodRef::new(
                    ClassRef::int_128(asm),
                    asm.alloc_string("op_GreaterThan"),
                    asm.sig([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    MethodKind::Static,
                    vec![].into(),
                );
                call!(asm.alloc_methodref(mref), [operand_a, operand_b])
            }
            _ => gt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            gt!(operand_a, operand_b)
        }
        TyKind::Float(FloatTy::F128) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("__gttf2"),
                asm.sig(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::Float(FloatTy::F16) => {
            let mref = MethodRef::new(
                *asm.main_module(),
                asm.alloc_string("gt_f16"),
                asm.sig(
                    [Type::Float(Float::F16), Type::Float(Float::F16)],
                    Type::Bool,
                ),
                MethodKind::Static,
                vec![].into(),
            );
            call!(asm.alloc_methodref(mref), [operand_a, operand_b])
        }
        TyKind::RawPtr(_, _) => gt_un!(operand_a, operand_b),
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
