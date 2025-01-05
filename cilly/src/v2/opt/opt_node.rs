use crate::{
    v2::{cilnode::ExtendKind, Assembly, CILNode, Const, Int, NodeIdx, Type},
    BinOp,
};

use super::{opt_if_fuel, OptFuel, SideEffectInfoCache};
/// Optimizes an intiger cast.
fn opt_int_cast(
    original: CILNode,
    asm: &mut Assembly,
    fuel: &mut OptFuel,
    input: NodeIdx,
    target: Int,
    extend: ExtendKind,
) -> CILNode {
    match asm.get_node(input) {
        CILNode::LdField { addr, field } if asm[*field].tpe() == Type::Int(target) => {
            asm.get_node(input).clone()
        }
        CILNode::Const(cst) => match (cst.as_ref(), target) {
            (Const::U64(val), Int::USize) => opt_if_fuel(Const::USize(*val).into(), original, fuel),
            (Const::I64(val), Int::ISize) => opt_if_fuel(Const::ISize(*val).into(), original, fuel),
            (Const::U64(val), Int::U64) => opt_if_fuel(Const::U64(*val).into(), original, fuel),
            (Const::I64(val), Int::I64) => opt_if_fuel(Const::I64(*val).into(), original, fuel),
            (Const::U32(val), Int::U32) => opt_if_fuel(Const::U32(*val).into(), original, fuel),
            (Const::I32(val), Int::I32) => opt_if_fuel(Const::I32(*val).into(), original, fuel),
            (Const::I32(val), Int::U32) => {
                opt_if_fuel(Const::U32(*val as u32).into(), original, fuel)
            }
            (Const::U64(val), Int::U8) => opt_if_fuel(Const::U8(*val as u8).into(), original, fuel),
            (Const::I32(val), Int::USize) => match extend {
                ExtendKind::SignExtend => {
                    opt_if_fuel(Const::USize(*val as i64 as u64).into(), original, fuel)
                }
                ExtendKind::ZeroExtend => {
                    opt_if_fuel(Const::USize(*val as u32 as u64).into(), original, fuel)
                }
            },
            _ => original,
        },
        CILNode::IntCast {
            input: input2,
            target: target2,
            extend: extend2,
        } => {
            if target == *target2 && extend == *extend2 {
                return opt_if_fuel(asm.get_node(input).clone(), original, fuel);
            }
            match (target, target2) {
                (Int::USize | Int::ISize, Int::USize | Int::ISize) => {
                    // A usize to isize cast does nothing, except change the type on the evaulation stack(the bits are unchanged).
                    // So, we can just create a cast like it.
                    opt_if_fuel(
                        CILNode::IntCast {
                            input: *input2,
                            target,
                            extend: *extend2,
                        },
                        original,
                        fuel,
                    )
                }
                (Int::U64 | Int::I64, Int::U64 | Int::I64) => {
                    // A u64 to i64 cast does nothing, except change the type on the evaulation stack(the bits are unchanged).
                    // So, we can just create a cast like it.
                    opt_if_fuel(
                        CILNode::IntCast {
                            input: *input2,
                            target,
                            extend: *extend2,
                        },
                        original,
                        fuel,
                    )
                }
                (Int::U32 | Int::I32, Int::U32 | Int::I32) => {
                    // A u64 to i64 cast does nothing, except change the type on the evaulation stack(the bits are unchanged).
                    // So, we can just create a cast like it.
                    opt_if_fuel(
                        CILNode::IntCast {
                            input: *input2,
                            target,
                            extend: *extend2,
                        },
                        original,
                        fuel,
                    )
                }
                _ => original,
            }
        }
        _ => original,
    }
}
pub fn opt_node(
    original: CILNode,
    asm: &mut Assembly,
    fuel: &mut OptFuel,
    cache: &mut SideEffectInfoCache,
) -> CILNode {
    match original {
        CILNode::SizeOf(tpe) => match asm[tpe] {
            Type::Int(
                int @ (Int::I128
                | Int::I64
                | Int::I32
                | Int::I16
                | Int::I8
                | Int::U128
                | Int::U64
                | Int::U32
                | Int::U16
                | Int::U8),
            ) => opt_if_fuel(
                Const::I32(int.size().unwrap() as i32).into(),
                original,
                fuel,
            ),
            _ => original,
        },
        CILNode::IntCast {
            input,
            target,
            extend,
        } => opt_int_cast(original, asm, fuel, input, target, extend),
        CILNode::Call(info) => super::inline::trivial_inline_call(info.0, &info.1, fuel, asm),
        CILNode::LdInd {
            addr,
            tpe,
            volatile: volitale,
        } => match asm.get_node(addr) {
            CILNode::RefToPtr(inner) => opt_if_fuel(
                CILNode::LdInd {
                    addr: *inner,
                    tpe,
                    volatile: volitale,
                },
                original,
                fuel,
            ),
            CILNode::LdLocA(loc) => opt_if_fuel(CILNode::LdLoc(*loc), original, fuel),
            CILNode::LdArgA(loc) => opt_if_fuel(CILNode::LdArg(*loc), original, fuel),
            CILNode::LdFieldAdress { addr, field } => {
                let field_desc = asm.get_field(*field);
                if field_desc.tpe() == asm[tpe] {
                    opt_if_fuel(
                        CILNode::LdField {
                            addr: *addr,
                            field: *field,
                        },
                        original,
                        fuel,
                    )
                } else {
                    original
                }
            }
            _ => original,
        },
        CILNode::LdFieldAdress { addr, field } => match asm.get_node(addr) {
            CILNode::RefToPtr(inner) => CILNode::RefToPtr(asm.alloc_node(CILNode::LdFieldAdress {
                addr: *inner,
                field,
            })),
            _ => original,
        },
        CILNode::BinOp(lhs, rhs, op @ (BinOp::Add | BinOp::Sub)) => {
            match (asm.get_node(lhs), asm.get_node(rhs)) {
                (CILNode::Const(cst), rhs) if cst.as_ref().is_zero() && op != BinOp::Sub => {
                    rhs.clone()
                }
                (lhs, CILNode::Const(cst)) if cst.as_ref().is_zero() => lhs.clone(),
                _ => original,
            }
        }
        //CILNode::BinOp(lhs,rhs ,BinOp::And) if lhs == rhs && cache.has_side_effects(lhs, asm)=> asm[lhs].clone(),
        CILNode::LdField { addr, field } => match asm.get_node(addr) {
            CILNode::RefToPtr(addr) => {
                opt_if_fuel(CILNode::LdField { addr: *addr, field }, original, fuel)
            }
            CILNode::LdLocA(loc) => opt_if_fuel(
                CILNode::LdField {
                    addr: asm.alloc_node(CILNode::LdLoc(*loc)),
                    field,
                },
                original,
                fuel,
            ),
            CILNode::LdArgA(loc) => opt_if_fuel(
                CILNode::LdField {
                    addr: asm.alloc_node(CILNode::LdArg(*loc)),
                    field,
                },
                original,
                fuel,
            ),
            CILNode::LdFieldAdress {
                addr: addr2,
                field: field2,
            } => opt_if_fuel(
                CILNode::LdField {
                    addr: asm.alloc_node(CILNode::LdField {
                        addr: *addr2,
                        field: *field2,
                    }),
                    field,
                },
                original,
                fuel,
            ),
            CILNode::LdInd {
                addr,
                tpe,
                volatile: _,
            } => {
                if let Type::ClassRef(tpe) = asm[*tpe] {
                    if tpe == asm.get_field(field).owner() {
                        opt_if_fuel(CILNode::LdField { addr: *addr, field }, original, fuel)
                    } else {
                        original
                    }
                } else {
                    original
                }
            }
            _ => original,
        },
        _ => original,
    }
}
