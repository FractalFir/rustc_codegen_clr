use crate::cil::CILOp;

pub fn optimize_combos(ops: &mut Vec<CILOp>) {
    if ops.is_empty() {
        return;
    }
    for idx in 0..(ops.len() - 1) {
        let (op1, op2) = (&ops[idx], &ops[idx + 1]);
        match (op1, op2) {
            // BB_SOURCE:goto target makes it so all goto SOURCE can be replaced with goto TARGET
            (CILOp::Label(source), CILOp::GoTo(target)) => {
                let source = *source;
                let target = *target;
                ops.iter_mut()
                    .for_each(|cilop| cilop.replace_target(source, target));
            }

            (CILOp::LDLoc(a), CILOp::STLoc(b)) => {
                if a == b {
                    ops[idx] = CILOp::Nop;
                    ops[idx + 1] = CILOp::Nop;
                }
            }
            (CILOp::STLoc(a), CILOp::LDLoc(b)) => {
                if a == b {
                    ops[idx + 1] = CILOp::STLoc(*a);
                    ops[idx] = CILOp::Dup;
                }
            }
            (CILOp::Dup | CILOp::LDLoc(_) | CILOp::LDLocA(_), CILOp::Pop) => {
                ops[idx] = CILOp::Nop;
                ops[idx + 1] = CILOp::Nop;
            }
            (CILOp::GoTo(target), CILOp::Label(label)) => {
                if target == label {
                    ops[idx] = CILOp::Nop;
                }
            }
            (CILOp::GoTo(_), op2) => {
                //TODO: Handle exception handling constructs!
                if let CILOp::Label(_) = op2 {
                }
                // Any op after GOTO and not preceded by a label is unreachable.
                else {
                    ops[idx + 1] = CILOp::Nop;
                }
            }
            (CILOp::Label(source), CILOp::Label(target)) => {
                let source = *source;
                let target = *target;
                ops.iter_mut()
                    .for_each(|cilop| cilop.replace_target(source, target));
            }
            (CILOp::Not, CILOp::BZero(target)) => {
                ops[idx + 1] = CILOp::BTrue(*target);
                ops[idx] = CILOp::Nop;
            }
            (CILOp::Eq, CILOp::BZero(target)) => {
                ops[idx + 1] = CILOp::BNe(*target);
                ops[idx] = CILOp::Nop;
            }
            (CILOp::Eq, CILOp::BTrue(target)) => {
                ops[idx + 1] = CILOp::BEq(*target);
                ops[idx] = CILOp::Nop;
            }
            (CILOp::LdcI32(0) | CILOp::LdcI64(0), CILOp::BEq(target)) => {
                ops[idx + 1] = CILOp::BZero(*target);
                ops[idx] = CILOp::Nop;
            }
            (CILOp::LdcI32(0) | CILOp::LdcI64(0), CILOp::BNe(target)) => {
                ops[idx + 1] = CILOp::BTrue(*target);
                ops[idx] = CILOp::Nop;
            }
            (CILOp::Lt, CILOp::BZero(target)) => {
                ops[idx + 1] = CILOp::BGe(*target);
                ops[idx] = CILOp::Nop;
            }
            (
                CILOp::LdcI32(_) | CILOp::LdcI64(_) | CILOp::LdcF32(_) | CILOp::LdcF64(_),
                CILOp::STLoc(loc),
            ) => {
                let loc = *loc;
                let set_count = ops.iter().filter(|op| CILOp::STLoc(loc) == **op).count();
                // If this is not the only set, this optimization won't work.
                if set_count != 1 {
                    continue;
                }
                // If a pointer to this local is taken, this optmization won't work.
                if ops.iter().any(|op| CILOp::LDLocA(loc) == *op) {
                    continue;
                }
                let load = op1.clone();
                ops.iter_mut()
                    .filter(|op| CILOp::LDLoc(loc) == **op)
                    .for_each(|op| *op = load.clone());
            }
            (
                CILOp::LdcI32(_) | CILOp::LdcI64(_) | CILOp::LdcF32(_) | CILOp::LdcF64(_),
                CILOp::Pop,
            ) => {
                ops[idx] = CILOp::Nop;
                ops[idx + 1] = CILOp::Nop;
            }
            (CILOp::ConvUSize(false) | CILOp::ConvF64(false), CILOp::Pop) => {
                ops[idx] = CILOp::Pop;
                ops[idx + 1] = CILOp::Nop;
            }
            _ => (),
        }
    }
}
