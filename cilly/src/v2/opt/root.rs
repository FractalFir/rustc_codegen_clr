use super::inline::inline_trivial_call_root;

use super::super::{cilroot::BranchCond, method::LocalDef, BinOp, CILNode, CILRoot, Const, Type};
pub use super::opt_fuel::OptFuel;
use super::opt_if_fuel;
pub use super::side_effect::*;
use crate::bimap::Interned;
use crate::cilroot::CmpKind;
use crate::Assembly;

pub fn root_opt(
    root: CILRoot,
    asm: &mut Assembly,
    root_fuel: &mut OptFuel,
    cache: &mut SideEffectInfoCache,
    locals: &[LocalDef],
) -> CILRoot {
    match root {
        CILRoot::Pop(pop) => match asm.get_node(pop) {
            CILNode::LdLoc(_) => CILRoot::Nop,
            _ => {
                let has_side_effects = cache.has_side_effects(pop, asm);
                if has_side_effects {
                    root
                } else {
                    CILRoot::Nop
                }
            }
        },
        CILRoot::Call(info) => inline_trivial_call_root(info.0, &info.1, root_fuel, asm),

        CILRoot::StInd(ref info) => match asm.get_node(info.0) {
            CILNode::LdLocA(loc) if asm[locals[*loc as usize].1] == info.2 => {
                CILRoot::StLoc(*loc, info.1)
            }
            _ => root,
        },
        CILRoot::SetField(info) => {
            let (field, mut addr, val) = info.as_ref();
            if let CILNode::RefToPtr(inner) = asm[addr] {
                addr = inner;
            }
            CILRoot::SetField(Box::new((*field, addr, *val)))
        }
        CILRoot::InitObj(addr, tpe) => opt_init_obj(addr, tpe, asm, root_fuel),
        CILRoot::Branch(ref info) => {
            let (target, sub_target, cond) = info.as_ref();
            match cond {
                Some(BranchCond::False(cond)) => {
                    match asm.get_node(*cond) {
                        CILNode::Const(cst) => match cst.as_ref() {
                            Const::Bool(false) => opt_if_fuel(
                                CILRoot::Branch(Box::new((*target, *sub_target, None))),
                                root,
                                root_fuel,
                            ),
                            Const::Bool(true) => opt_if_fuel(CILRoot::Nop, root, root_fuel),
                            _ => root,
                        },
                        // a == b is false <=> a != b
                        CILNode::BinOp(lhs, rhs, BinOp::Eq) => opt_if_fuel(
                            {
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Ne(*lhs, *rhs)),
                                )))
                            },
                            root,
                            root_fuel,
                        ),
                        // a > b is false <=> a <= b
                        CILNode::BinOp(lhs, rhs, BinOp::Gt) => opt_if_fuel(
                            {
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Le(*lhs, *rhs, CmpKind::Ordered)),
                                )))
                            },
                            root,
                            root_fuel,
                        ),
                        CILNode::BinOp(lhs, rhs, BinOp::GtUn) => opt_if_fuel(
                            {
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Le(*lhs, *rhs, CmpKind::Unordered)),
                                )))
                            },
                            root,
                            root_fuel,
                        ),
                        // a < b is false <=> a >= b
                        CILNode::BinOp(lhs, rhs, BinOp::Lt) => opt_if_fuel(
                            {
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Ge(*lhs, *rhs, CmpKind::Ordered)),
                                )))
                            },
                            root,
                            root_fuel,
                        ),
                        CILNode::BinOp(lhs, rhs, BinOp::LtUn) => opt_if_fuel(
                            {
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::Ge(*lhs, *rhs, CmpKind::Unordered)),
                                )))
                            },
                            root,
                            root_fuel,
                        ),
                        //CILNode::IntCast { input, target, extend }
                        _ => root,
                    }
                }
                Some(BranchCond::True(cond)) => match asm.get_node(*cond) {
                    // a == b  is true <=> a == b
                    CILNode::BinOp(lhs, rhs, BinOp::Eq) => opt_if_fuel(
                        CILRoot::Branch(Box::new((
                            *target,
                            *sub_target,
                            Some(BranchCond::Eq(*lhs, *rhs)),
                        ))),
                        root,
                        root_fuel,
                    ),
                    CILNode::BinOp(lhs, rhs, BinOp::GtUn) => opt_if_fuel(
                        {
                            CILRoot::Branch(Box::new((
                                *target,
                                *sub_target,
                                Some(BranchCond::Gt(*lhs, *rhs, CmpKind::Unordered)),
                            )))
                        },
                        root,
                        root_fuel,
                    ),
                    CILNode::BinOp(lhs, rhs, BinOp::Gt) => opt_if_fuel(
                        CILRoot::Branch(Box::new((
                            *target,
                            *sub_target,
                            Some(BranchCond::Gt(*lhs, *rhs, CmpKind::Ordered)),
                        ))),
                        root,
                        root_fuel,
                    ),
                    CILNode::BinOp(lhs, rhs, BinOp::LtUn) => opt_if_fuel(
                        {
                            CILRoot::Branch(Box::new((
                                *target,
                                *sub_target,
                                Some(BranchCond::Lt(*lhs, *rhs, CmpKind::Unordered)),
                            )))
                        },
                        root,
                        root_fuel,
                    ),
                    CILNode::BinOp(lhs, rhs, BinOp::Lt) => opt_if_fuel(
                        CILRoot::Branch(Box::new((
                            *target,
                            *sub_target,
                            Some(BranchCond::Lt(*lhs, *rhs, CmpKind::Ordered)),
                        ))),
                        root,
                        root_fuel,
                    ),
                    _ => root,
                },
                Some(BranchCond::Ne(lhs, rhs)) => {
                    match (asm.get_node(*lhs), asm.get_node(*rhs)) {
                        (_, CILNode::Const(cst)) => match cst.as_ref() {
                            // val != false <=> val is true
                            Const::Bool(false)
                            | Const::ISize(0)
                            | Const::USize(0)
                            | Const::I64(0)
                            | Const::U64(0)
                            | Const::I32(0)
                            | Const::U32(0)
                            | Const::I16(0)
                            | Const::U16(0)
                            | Const::I8(0)
                            | Const::U8(0) => opt_if_fuel(
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::True(*lhs)),
                                ))),
                                root,
                                root_fuel,
                            ),
                            // val != true <=> val is false
                            Const::Bool(true) => opt_if_fuel(
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::False(*lhs)),
                                ))),
                                root,
                                root_fuel,
                            ),
                            _ => root,
                        },
                        (CILNode::Const(cst), _) => match cst.as_ref() {
                            // val != false <=> val is true
                            Const::Bool(false)
                            | Const::ISize(0)
                            | Const::USize(0)
                            | Const::I64(0)
                            | Const::U64(0)
                            | Const::I32(0)
                            | Const::U32(0)
                            | Const::I16(0)
                            | Const::U16(0)
                            | Const::I8(0)
                            | Const::U8(0) => opt_if_fuel(
                                CILRoot::Branch(Box::new((
                                    *target,
                                    *sub_target,
                                    Some(BranchCond::True(*rhs)),
                                ))),
                                root,
                                root_fuel,
                            ),
                            _ => root,
                        },
                        _ => root,
                    }
                }
                Some(BranchCond::Eq(lhs, rhs)) => match (asm.get_node(*lhs), asm.get_node(*rhs)) {
                    (_, CILNode::Const(cst)) => match cst.as_ref() {
                        Const::Bool(false)
                        | Const::ISize(0)
                        | Const::USize(0)
                        | Const::I64(0)
                        | Const::U64(0)
                        | Const::I32(0)
                        | Const::U32(0)
                        | Const::I16(0)
                        | Const::U16(0)
                        | Const::I8(0)
                        | Const::U8(0) => opt_if_fuel(
                            CILRoot::Branch(Box::new((
                                *target,
                                *sub_target,
                                Some(BranchCond::False(*lhs)),
                            ))),
                            root,
                            root_fuel,
                        ),
                        _ => root,
                    },
                    (CILNode::Const(cst), _) => match cst.as_ref() {
                        Const::Bool(false)
                        | Const::ISize(0)
                        | Const::USize(0)
                        | Const::I64(0)
                        | Const::U64(0)
                        | Const::I32(0)
                        | Const::U32(0)
                        | Const::I16(0)
                        | Const::U16(0)
                        | Const::I8(0)
                        | Const::U8(0) => opt_if_fuel(
                            CILRoot::Branch(Box::new((
                                *target,
                                *sub_target,
                                Some(BranchCond::False(*rhs)),
                            ))),
                            root,
                            root_fuel,
                        ),
                        _ => root,
                    },
                    _ => root,
                },
                Some(_) | None => root,
            }
        }
        CILRoot::StLoc(loc, val) if asm[val] == CILNode::LdLoc(loc) => CILRoot::Nop,
        CILRoot::StArg(loc, val) if asm[val] == CILNode::LdArg(loc) => CILRoot::Nop,
        _ => root,
    }
}
fn opt_init_obj(
    mut addr: Interned<CILNode>,
    tpe: Interned<Type>,
    asm: &mut Assembly,
    fuel: &mut OptFuel,
) -> CILRoot {
    // 1. Check if the addr is RefToPtr. If so, remove that.
    if let CILNode::RefToPtr(inner) = asm[addr] {
        if fuel.consume(1) {
            addr = inner;
        }
    }
    // 2. Check if the type is a small primitive - if so, replace this with StObj to allow for more optimizations.
    match asm[tpe] {
        Type::Int(int) if int.size().unwrap_or(8) <= 8 && fuel.consume(1) => {
            return CILRoot::StInd(Box::new((
                addr,
                asm.alloc_node(int.zero()),
                Type::Int(int),
                false,
            )));
        }
        Type::Float(float) if fuel.consume(1) && matches!(float.size(), 32 | 64) => {
            return CILRoot::StInd(Box::new((
                addr,
                asm.alloc_node(float.zero()),
                Type::Float(float),
                false,
            )));
        }
        Type::Bool if fuel.consume(1) => {
            return CILRoot::StInd(Box::new((addr, asm.alloc_node(false), Type::Bool, false)));
        }
        _ => (),
    }
    CILRoot::InitObj(addr, tpe)
}
