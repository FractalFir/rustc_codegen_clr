use cilly::{
    call, call_site::CallSite, cil_node::CILNode, eq, fn_sig::FnSig, gt, gt_un, lt, lt_un,
    DotnetTypeRef, Type,
};
use rustc_middle::ty::{IntTy, Ty, TyKind, UintTy};

pub fn ne_unchecked(ty_a: Ty<'_>, operand_a: CILNode, operand_b: CILNode) -> CILNode {
    //vec![eq_unchecked(ty_a), CILOp::LdcI32(0), CILOp::Eq]
    eq!(eq_unchecked(ty_a, operand_a, operand_b), CILNode::LdFalse)
}
pub fn eq_unchecked(ty_a: Ty<'_>, operand_a: CILNode, operand_b: CILNode) -> CILNode {
    //vec![CILOp::Eq]
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_Equality".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_Equality".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => eq!(operand_a, operand_b),
        },
        TyKind::Bool | TyKind::Char | TyKind::Float(_) | TyKind::RawPtr(_, _) => {
            eq!(operand_a, operand_b)
        }
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
pub fn lt_unchecked(ty_a: Ty<'_>, operand_a: CILNode, operand_b: CILNode) -> CILNode {
    //return CILOp::Lt;
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_LessThan".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => lt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_LessThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => lt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(_) => lt!(operand_a, operand_b),
        TyKind::RawPtr(_, _) | TyKind::FnPtr(_, _) => lt_un!(operand_a, operand_b),
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
pub fn gt_unchecked(ty_a: Ty<'_>, operand_a: CILNode, operand_b: CILNode) -> CILNode {
    match ty_a.kind() {
        TyKind::Uint(uint) => match uint {
            UintTy::U128 => call!(
                CallSite::new_extern(
                    DotnetTypeRef::uint_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::U128, Type::U128], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => gt_un!(operand_a, operand_b),
        },
        TyKind::Int(int) => match int {
            IntTy::I128 => call!(
                CallSite::new_extern(
                    DotnetTypeRef::int_128(),
                    "op_GreaterThan".into(),
                    FnSig::new(&[Type::I128, Type::I128], Type::Bool),
                    true,
                ),
                [operand_a, operand_b]
            ),
            _ => gt!(operand_a, operand_b),
        },
        // TODO: are chars considered signed or unsigned?
        TyKind::Bool | TyKind::Char | TyKind::Float(_) => gt!(operand_a, operand_b),
        TyKind::RawPtr(_, _) => gt_un!(operand_a, operand_b),
        _ => panic!("Can't eq type  {ty_a:?}"),
    }
}
