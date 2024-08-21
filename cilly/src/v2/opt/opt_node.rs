use crate::v2::{Assembly, CILNode, Const, Int, Type};

pub fn opt_node(node: crate::v2::CILNode, asm: &mut Assembly) -> CILNode {
    match node {
        CILNode::IntCast {
            input,
            target,
            extend,
        } => match asm.get_node(input) {
            CILNode::Const(cst) => match (cst.as_ref(), target) {
                (Const::U64(val), Int::USize) => Const::USize(*val).into(),
                (Const::I64(val), Int::ISize) => Const::ISize(*val).into(),
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
                node
            }
            _ => node,
        },
        CILNode::LdInd {
            addr,
            tpe,
            volitale,
        } => {
            if let CILNode::RefToPtr(inner) = asm.get_node(addr) {
                CILNode::LdInd {
                    addr: *inner,
                    tpe,
                    volitale,
                }
            } else {
                node
            }
        }

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
