use cilly::{
    call,
    call_site::CallSite,
    cil_node::CILNode,
    eq,
    fn_sig::FnSig,
    gt, gt_un, lt, lt_un,
    v2::{Assembly, ClassRef, Float, Int},
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
        CILNode::LdFalse
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
            UintTy::U128 => call!(
                CallSite::new_extern(
                    ClassRef::uint_128(asm),
                    "op_Equality".into(),
                    FnSig::new([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_Equality".into(),
                    FnSig::new([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Bool
        | TyKind::Char
        | TyKind::Float(FloatTy::F32 | FloatTy::F64)
        | TyKind::RawPtr(_, _) => {
            eq!(operand_a, operand_b)
        }
        TyKind::Float(FloatTy::F128) => call!(
            CallSite::builtin(
                "__eqtf2".into(),
                FnSig::new(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool
                ),
                true
            ),
            [operand_a, operand_b]
        ),

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
            UintTy::U128 => call!(
                CallSite::new_extern(
                    ClassRef::uint_128(asm),
                    "op_LessThan".into(),
                    FnSig::new([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => lt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_LessThan".into(),
                    FnSig::new([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => lt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            lt!(operand_a, operand_b)
        }
        TyKind::RawPtr(_, _) | TyKind::FnPtr(_, _) => lt_un!(operand_a, operand_b),
        TyKind::Float(FloatTy::F128) => call!(
            CallSite::builtin(
                "__lttf2".into(),
                FnSig::new(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool
                ),
                true
            ),
            [operand_a, operand_b]
        ),
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
            UintTy::U128 => call!(
                CallSite::new_extern(
                    ClassRef::uint_128(asm),
                    "op_GreaterThan".into(),
                    FnSig::new([Type::Int(Int::U128), Type::Int(Int::U128)], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => gt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => call!(
                CallSite::new_extern(
                    ClassRef::int_128(asm),
                    "op_GreaterThan".into(),
                    FnSig::new([Type::Int(Int::I128), Type::Int(Int::I128)], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => gt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(FloatTy::F32 | FloatTy::F64) => {
            gt!(operand_a, operand_b)
        }
        TyKind::Float(FloatTy::F128) => call!(
            CallSite::builtin(
                "__gttf2".into(),
                FnSig::new(
                    [Type::Float(Float::F128), Type::Float(Float::F128)],
                    Type::Bool
                ),
                true
            ),
            [operand_a, operand_b]
        ),
        TyKind::RawPtr(_, _) => gt_un!(operand_a, operand_b),
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
