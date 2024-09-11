use crate::v2::{Assembly, CILNode, Const, Int, Type};

use super::OptFuel;

pub fn opt_node(node: crate::v2::CILNode, asm: &mut Assembly, fuel: &mut OptFuel) -> CILNode {
    match node {
        CILNode::IntCast {
            input,
            target,
            extend,
        } => match asm.get_node(input) {
            CILNode::Const(cst) => match (cst.as_ref(), target) {
                (Const::U64(val), Int::USize) => Const::USize(*val).into(),
                (Const::I64(val), Int::ISize) => Const::ISize(*val).into(),
                (Const::U64(val), Int::U64) => Const::U64(*val).into(),
                (Const::I64(val), Int::I64) => Const::I64(*val).into(),
                (Const::U32(val), Int::U32) => Const::U32(*val).into(),
                (Const::I32(val), Int::I32) => Const::I32(*val).into(),
                (Const::I32(val), Int::U32) => Const::U32(*val as u32).into(),
                (Const::U64(val), Int::U8) => Const::U8(*val as u8).into(),
                _ => node,
            },
            CILNode::IntCast {
                input: input2,
                target: target2,
                extend: extend2,
            } => {
                if target == *target2 && extend == *extend2 {
                    return asm.get_node(input).clone();
                }
                match (target, target2) {
                    (Int::USize | Int::ISize, Int::USize | Int::ISize) => {
                        // A usize to isize cast does nothing, except change the type on the evaulation stack(the bits are unchanged).
                        // So, we can just create a cast like it.
                        CILNode::IntCast {
                            input: *input2,
                            target,
                            extend: *extend2,
                        }
                    }
                    (Int::U64 | Int::I64, Int::U64 | Int::I64) => {
                        // A u64 to i64 cast does nothing, except change the type on the evaulation stack(the bits are unchanged).
                        // So, we can just create a cast like it.
                        CILNode::IntCast {
                            input: *input2,
                            target,
                            extend: *extend2,
                        }
                    }
                    (Int::U32 | Int::I32, Int::U32 | Int::I32) => {
                        // A u64 to i64 cast does nothing, except change the type on the evaulation stack(the bits are unchanged).
                        // So, we can just create a cast like it.
                        CILNode::IntCast {
                            input: *input2,
                            target,
                            extend: *extend2,
                        }
                    }
                    _ => node,
                }
            }
            _ => node,
        },
        CILNode::Call(info) => super::inline::trivial_inline_call(info.0, &info.1, fuel, asm),
        CILNode::LdInd {
            addr,
            tpe,
            volitale,
        } => match asm.get_node(addr) {
            CILNode::RefToPtr(inner) => CILNode::LdInd {
                addr: *inner,
                tpe,
                volitale,
            },
            CILNode::LdLocA(loc) => CILNode::LdLoc(*loc),
            CILNode::LdArgA(loc) => CILNode::LdArg(*loc),
            CILNode::LdFieldAdress { addr, field } => {
                let field_desc = asm.get_field(*field);
                if field_desc.tpe() == *asm.get_type(tpe) {
                    CILNode::LdField {
                        addr: *addr,
                        field: *field,
                    }
                } else {
                    node
                }
            }
            _ => node,
        },

        CILNode::LdField { addr, field } => match asm.get_node(addr) {
            CILNode::RefToPtr(addr) => CILNode::LdField { addr: *addr, field },
            CILNode::LdLocA(loc) => CILNode::LdField {
                addr: asm.alloc_node(CILNode::LdLoc(*loc)),
                field,
            },
            CILNode::LdArgA(loc) => CILNode::LdField {
                addr: asm.alloc_node(CILNode::LdArg(*loc)),
                field,
            },
            CILNode::LdFieldAdress {
                addr: addr2,
                field: field2,
            } => CILNode::LdField {
                addr: asm.alloc_node(CILNode::LdField {
                    addr: *addr2,
                    field: *field2,
                }),
                field,
            },
            CILNode::LdInd {
                addr,
                tpe,
                volitale: _,
            } => {
                if let Type::ClassRef(tpe) = *asm.get_type(*tpe) {
                    if tpe == asm.get_field(field).owner() {
                        CILNode::LdField { addr: *addr, field }
                    } else {
                        node
                    }
                } else {
                    node
                }
            }
            _ => node,
        },
        _ => node,
    }
}
