use crate::{cil_op::CILOp, method::Method, r#type::Type};
const MAX_PASS: u32 = 8;
pub fn opt_method(method: &mut Method) {
    if !crate::OPTIMIZE_CIL {
        return;
    };
    //panic!("opt");
    repalce_const_sizes(method.ops_mut());
    for _ in 0..MAX_PASS {
        op2_combos(method.ops_mut());
        op3_combos(method.ops_mut());
        op4_combos(method.ops_mut());
        remove_zombie_sets(method.ops_mut());
        method.ops_mut().retain(|op| *op != CILOp::Nop);
        remove_unused_locals(method);
    }
}
fn repalce_const_sizes(ops: &mut [CILOp]) {
    ops.iter_mut().for_each(|op| match op {
        CILOp::SizeOf(tpe) => match tpe.as_ref() {
            Type::U8 | Type::I8 => *op = CILOp::LdcI32(1),
            _ => (),
        },
        _ => (),
    })
}
fn remove_unused_locals(method: &mut Method) {
    let mut local_map = vec![u32::MAX; method.locals().len()];
    let mut new_locals = Vec::with_capacity(method.locals().len());
    for (local, tpe) in method.locals().iter().enumerate() {
        if local_map[local] == u32::MAX && !is_local_unused(method.get_ops(), local as u32) {
            local_map[local] = new_locals.len() as u32;
            new_locals.push(tpe.clone());
        }
    }
    method.ops_mut().iter_mut().for_each(|op| match op {
        CILOp::LDLoc(idx) => {
            let new_loc = local_map[*idx as usize];
            *op = CILOp::LDLoc(new_loc);
        }
        CILOp::LDLocA(idx) => {
            let new_loc = local_map[*idx as usize];
            *op = CILOp::LDLocA(new_loc);
        }
        CILOp::STLoc(idx) => {
            let new_loc = local_map[*idx as usize];
            *op = CILOp::STLoc(new_loc);
        }
        _ => (),
    });
    method.set_locals(new_locals);
}
fn remove_zombie_sets(ops: &mut Vec<CILOp>) {
    for idx in 0..ops.len() {
        match ops[idx] {
            CILOp::STLoc(loc) => {
                if is_local_dead(ops, loc) {
                    ops[idx] = CILOp::Pop;
                }
            }
            CILOp::Label(label) => {
                if is_label_unsused(ops, label) {
                    ops[idx] = CILOp::Nop;
                }
            }
            _ => (),
        }
    }
}
fn op2_combos(ops: &mut Vec<CILOp>) {
    if ops.is_empty() {
        return;
    }
    for idx in 0..(ops.len() - 1) {
        let (op1, op2) = (&ops[idx], &ops[idx + 1]);
        match (op1, op2) {
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
            (CILOp::LdcI32(0) | CILOp::LdcI64(0), CILOp::BEq(target)) => {
                ops[idx + 1] = CILOp::BZero(*target);
                ops[idx] = CILOp::Nop;
            }
            (CILOp::Lt, CILOp::BZero(target)) => {
                ops[idx + 1] = CILOp::BGe(*target);
                ops[idx] = CILOp::Nop;
            }
            _ => (),
        }
    }
}
fn op3_combos(ops: &mut Vec<CILOp>) {
    if ops.len() < 3 {
        return;
    }
    for idx in 0..(ops.len() - 2) {
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
            (CILOp::GoTo(a1), CILOp::Label(b), CILOp::Label(a2)) => {
                if a1 == a2 {
                    let a = *a1;
                    let b = *b;
                    ops[idx] = CILOp::Nop;
                    ops[idx + 1] = CILOp::Label(b);
                    ops[idx + 2] = CILOp::Label(a);
                }
            }
            _ => (),
        }
    }
}
fn op4_combos(ops: &mut [CILOp]) {
    if ops.len() < 4 {
        return;
    }
    // Beq(A) Br(B) Label(A)
    for idx in 0..(ops.len() - 3) {
        let (op1, op2, op3, op4) = (&ops[idx], &ops[idx + 1], &ops[idx + 2], &ops[idx + 3]);
        match (op1, op2, op3, op4) {
            (
                CILOp::STLoc(b1),
                CILOp::LDLoc(_) | CILOp::LDArg(_),
                CILOp::LDLoc(b2),
                CILOp::Add | CILOp::Mul,
            ) => {
                if b1 == b2 {
                    let op2 = op2.clone();
                    ops[idx + 1] = op4.clone();
                    ops[idx] = op2;
                    ops[idx + 2] = CILOp::Nop;
                    ops[idx + 3] = CILOp::Nop;
                }
            }
            //This is bugged.
            /*
            (
                CILOp::STLoc(b1),
                CILOp::LDLoc(_) | CILOp::LDArg(_),
                CILOp::LDLoc(b2),
                CILOp::BGe(target),
            ) => {
                if b1 == b2 {
                    // b
                    // dup  | b b
                    // stloc(b) | b
                    // ldloc(a) | b a instead of ab
                    // flipped contional
                    let op2 = op2.clone();
                    let op4 = op4.flip_cond();
                    let b = *b1;
                    ops[idx] = CILOp::Dup;
                    ops[idx + 1] = CILOp::STLoc(b);
                    ops[idx + 2] = op2;
                    ops[idx + 3] = op4;
                }
            }*/
            (
                CILOp::LDLoc(_a),
                CILOp::STLoc(b1),
                CILOp::LDLoc(c) | CILOp::LDArg(c),
                CILOp::LDLoc(b2),
            ) => {
                if b1 == b2 && c != b1 {
                    let op1 = op1.clone();
                    let op3 = op3.clone();
                    let b = *b1;
                    ops[idx] = op3;
                    ops[idx + 1] = op1;
                    ops[idx + 2] = CILOp::Dup;
                    ops[idx + 3] = CILOp::STLoc(b);
                }
            }
            _ => (),
        }
    }
}
/// A "Dead" local is one that is only written into - never read.
fn is_local_dead(ops: &[CILOp], local: u32) -> bool {
    !ops.iter().any(|op| match op {
        CILOp::LDLoc(loc) => *loc == local,
        CILOp::LDLocA(loc) => *loc == local,
        _ => false,
    })
}
/// A "Unused" local is one that is never written to or read from.
fn is_local_unused(ops: &[CILOp], local: u32) -> bool {
    !ops.iter().any(|op| match op {
        CILOp::LDLoc(loc) => *loc == local,
        CILOp::LDLocA(loc) => *loc == local,
        CILOp::STLoc(loc) => *loc == local,
        _ => false,
    })
}
/// A "Unused" label is one that is never jumped to
fn is_label_unsused(ops: &[CILOp], label: u32) -> bool {
    !ops.iter().any(|op| match op {
        CILOp::GoTo(target) => label == *target,
        CILOp::BEq(target) => label == *target,
        CILOp::BNe(target) => label == *target,
        CILOp::BLt(target) => label == *target,
        CILOp::BGe(target) => label == *target,
        CILOp::BZero(target) => label == *target,
        _ => false,
    })
}
#[test]
fn cond_reordering() {
    let mut ops = [
        CILOp::STLoc(0),
        CILOp::LDLoc(1),
        CILOp::LDLoc(0),
        CILOp::BGe(0),
    ];
    op4_combos(&mut ops);
    assert_eq!(
        ops,
        [CILOp::Dup, CILOp::STLoc(0), CILOp::LDLoc(1), CILOp::BLt(0)]
    );
    //panic!("ops:{ops:?}")
}
