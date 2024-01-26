use crate::cil::CILOp;
pub fn optimize_combos(ops: &mut Vec<CILOp>) {
    if ops.len() < 3 {
        return;
    }
    'outer: for idx in 0..(ops.len() - 2) {
        let (op1, op2, op3) = (&ops[idx], &ops[idx + 1], &ops[idx + 2]);
        match (op1, op2, op3) {
            (CILOp::LdcI32(1), CILOp::LDLoc(_) | CILOp::LDArg(_), CILOp::Mul) => {
                ops[idx] = op2.clone();
                ops[idx + 1] = CILOp::Nop;
                ops[idx + 2] = CILOp::Nop;
            }
            (_, CILOp::LdcI32(1), CILOp::Mul) => {
                ops[idx] = op1.clone();
                ops[idx + 1] = CILOp::Nop;
                ops[idx + 2] = CILOp::Nop;
            }
            (CILOp::BEq(a1), CILOp::GoTo(b), CILOp::Label(a2)) => {
                if a1 == a2 {
                    let a = *a1;
                    let b = *b;
                    ops[idx] = CILOp::BNe(b);
                    ops[idx + 1] = CILOp::Nop;
                    ops[idx + 2] = CILOp::Label(a);
                }
            }
            (CILOp::LdcI32(0) | CILOp::LdcI64(0), CILOp::ConvISize(_), CILOp::BEq(target)) => {
                let target = *target;
                ops[idx] = CILOp::Nop;
                ops[idx + 1] = CILOp::Nop;
                ops[idx + 2] = CILOp::BZero(target);
            }
            (CILOp::LDLoc(a), CILOp::Dup, CILOp::STLoc(b)) => {
                if a == b {
                    let a = *a;
                    ops[idx] = CILOp::Nop;
                    ops[idx + 1] = CILOp::Nop;
                    ops[idx + 2] = CILOp::LDLoc(a);
                }
            }
            (CILOp::GoTo(a1), CILOp::Label(b), CILOp::Label(a2)) => {
                if a1 == a2 {
                    let a = *a1;
                    let b = *b;
                    ops[idx] = CILOp::Nop;
                    ops[idx + 1] = CILOp::Label(b);
                    ops[idx + 2] = CILOp::Label(a);
                }
            }
            //TODO: ensure changing the offset of ops does not cause issues.
            (
                CILOp::LdcI32(_) | CILOp::LdcI64(_) | CILOp::LdcF32(_) | CILOp::LdcF64(_),
                CILOp::ConvUSize(_) | CILOp::ConvF64(_),
                CILOp::STLoc(loc),
            ) => {
                let loc = *loc;

                let mut set_count = 0; //ops.iter().filter(|op|).count();
                for op in &*ops {
                    match op {
                        CILOp::STLoc(local) => {
                            if *local == loc {
                                set_count += 1;
                                // If this is not the only set, this optimization won't work.
                                if set_count != 1 {
                                    continue 'outer;
                                }
                            }
                        }
                        CILOp::LDLocA(local) => {
                            if *local == loc {
                                // If a pointer to this local is taken, this optmization won't work.
                                continue 'outer;
                            }
                        }
                        _ => (),
                    }
                }
                let load = [op1.clone(), op2.clone()];
                let mut new_ops = Vec::with_capacity(ops.len());
                for op in ops.iter() {
                    match op {
                        CILOp::LDLoc(oplocal) => {
                            if loc == *oplocal {
                                new_ops.extend(load.iter().cloned())
                            } else {
                                new_ops.push(op.clone())
                            }
                        }
                        _ => new_ops.push(op.clone()),
                    }
                }
                *ops = new_ops;
                continue;
            }
            _ => (),
        }
    }
}
