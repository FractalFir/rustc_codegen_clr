use std::ops::Range;

use crate::{assembly::Assembly, cil_op::CILOp, method::Method, r#type::Type};
const MAX_PASS: u32 = 16;
pub fn try_inline(caller: &mut Method, inlined: &Method, target: usize) {
    // Inlining is still sometimes quite buggy.
    if true {
        return;
    }
    // Can't yet inline non-empty methods!
    if !inlined.locals().is_empty() {
        return;
    }

    // Can't yet inline methods with multpile returns
    if inlined
        .get_ops()
        .iter()
        .filter(|op| **op == CILOp::Ret)
        .count()
        > 1
    {
        return;
    }

    // Can't yet handle inline methods with non-trivial control flow.
    if inlined
        .get_ops()
        .iter()
        .any(|op| matches!(op, CILOp::Label(_)))
    {
        return;
    }
    //eprintln!("Inlining {inlined_name}",inlined_name = inlined.name());
    let arg_beg = caller.locals().len();
    caller.add_local(inlined.sig().output().clone());
    caller.extend_locals(inlined.sig().inputs().iter());
    let mut inlined_call = Vec::new();
    for (index, _) in inlined.sig().inputs().iter().enumerate() {
        inlined_call.push(CILOp::STLoc((arg_beg + 1 + index) as u32));
    }
    let mut inlined_method_ops: Vec<_> = inlined.get_ops().into();
    inlined_method_ops.iter_mut().for_each(|op| match op {
        CILOp::LDArg(id) => *op = CILOp::LDLoc((arg_beg + 1 + *id as usize) as u32),
        CILOp::LDArgA(id) => *op = CILOp::LDLocA((arg_beg + 1 + *id as usize) as u32),
        CILOp::STArg(id) => *op = CILOp::STArg((arg_beg + 1 + *id as usize) as u32),
        CILOp::LDLoc(_) | CILOp::LDLocA(_) | CILOp::STLoc(_) => {
            todo!("Inlining locals not supported yet!")
        }
        CILOp::Ret => *op = CILOp::Nop,
        _ => (),
    });
    inlined_call.extend(inlined_method_ops);
    let ops = caller.ops_mut();
    // Remove the call
    let preamble = &ops[..target];
    //eprintln!("preamble:{preamble:?}");
    let epilouge = &ops[(target + 1)..];
    //eprintln!("epilouge:{epilouge:?}");
    // /eprintln!("inlined_call:{inlined_call:?}");
    let mut new_ops = Vec::with_capacity(ops.len() + inlined_call.len());
    new_ops.extend(preamble.iter().cloned());
    new_ops.extend(inlined_call);
    new_ops.extend(epilouge.iter().cloned());

    *ops = new_ops;
}
pub fn opt_method(method: &mut Method, asm: &Assembly) {
    if !crate::OPTIMIZE_CIL {
        return;
    };
    //panic!("opt");
    method.ops_mut().retain(|op| match op {
        CILOp::Call(site) => !site.is_nop(),
        _ => true,
    });
    repalce_const_sizes(method.ops_mut());
    for _ in 0..MAX_PASS {
        op2_combos(method.ops_mut());
        op3_combos(method.ops_mut());
        op4_combos(method.ops_mut());
        remove_zombie_sets(method.ops_mut());
        method.ops_mut().retain(|op| *op != CILOp::Nop);
        try_alias_locals(method.ops_mut());
        try_split_locals(method, asm);
        remove_unused_locals(method);
        //Inlining
        let inline_candidates: Vec<_> = method
            .get_ops()
            .iter()
            .enumerate()
            .filter_map(|(idx, op)| match op {
                CILOp::Call(callsite) => Some((idx, callsite.clone())),
                _ => None,
            })
            .filter(|(_, site)| site.is_static() && site.class().is_none())
            .collect();
        for (target, candidate) in inline_candidates {
            let linlined = if let Some(method) = asm.methods().find(|method| {
                method.name() == candidate.name()
                    && method.sig() == candidate.signature()
                    && method.is_static()
            }) {
                method
            } else {
                continue;
            };
            try_inline(method, linlined, target);
        }
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
            (CILOp::Label(source), CILOp::Label(target)) => {
                let source = *source;
                let target = *target;
                ops.iter_mut()
                    .for_each(|cilop| cilop.replace_target(source, target));
            }
            (CILOp::GoTo(_), CILOp::GoTo(_)) => ops[idx + 1] = CILOp::Nop,
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
            (
                CILOp::STLoc(b1),
                CILOp::LDLoc(_),
                CILOp::LDLoc(b2),
                CILOp::BGe(_) | CILOp::BEq(_) | CILOp::BNe(_) | CILOp::Eq,
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
            }
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
        CILOp::BLe(target) => label == *target,
        CILOp::BZero(target) => label == *target,
        CILOp::BTrue(target) => label == *target,
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
        [CILOp::Dup, CILOp::STLoc(0), CILOp::LDLoc(1), CILOp::BLe(0)]
    );
    //panic!("ops:{ops:?}")
}
fn alias_local(src: u32, dst: u32, ops: &mut [CILOp]) {
    ops.iter_mut().for_each(|op| match op {
        CILOp::LDLoc(loc) => {
            if *loc == src {
                *loc = dst;
            }
        }
        CILOp::LDLocA(loc) => {
            if *loc == src {
                *loc = dst;
            }
        }
        CILOp::STLoc(loc) => {
            if *loc == src {
                *loc = dst;
            }
        }
        _ => (),
    });
}
fn try_alias_locals(ops: &mut [CILOp]) {
    if ops.len() < 2 {
        return;
    }
    for index in 0..(ops.len() - 2) {
        let op1 = &ops[index];
        let op2 = &ops[index + 1];
        let op3 = &ops[index + 2];
        if let (CILOp::LDLoc(loc1), CILOp::STLoc(loc2)) = (op1, op2) {
            //eprintln!("Checking for alias between {loc1} and {loc2}!");
            if loc1 == loc2 {
                //eprintln!("locals equal!");
                continue;
            }
            if could_local_ptr_escape(*loc1, ops) {
                //eprintln!("ptr could escape!");
                continue;
            }
            let loc1_range = if let Some(range) = get_local_access_range(*loc1, ops) {
                range
            } else {
                continue;
            };
            let loc2_range = if let Some(range) = get_local_access_range(*loc2, ops) {
                range
            } else {
                continue;
            };
            // Ranges don't overlap, use simple aliasing
            if !do_ranges_overlap(&loc1_range, &loc2_range) {
                //println!("{loc2} will now be {loc1}!");
                alias_local(*loc2, *loc1, ops);
                continue;
            }
            //eprintln!("ranges {loc1_range:?} {loc2_range:?} overlap.");
        }
        if let (CILOp::LDLoc(loc1), CILOp::Dup, CILOp::STLoc(loc2)) = (op1, op2, op3) {
            //eprintln!("Checking for alias between {loc1} and {loc2}!");
            if loc1 == loc2 {
                //eprintln!("locals equal!");
                continue;
            }
            if could_local_ptr_escape(*loc1, ops) {
                //eprintln!("ptr could escape!");
                continue;
            }
            let loc1_range = if let Some(range) = get_local_access_range(*loc1, ops) {
                range
            } else {
                continue;
            };
            let loc2_range = if let Some(range) = get_local_access_range(*loc2, ops) {
                range
            } else {
                continue;
            };
            // Ranges don't overlap, use simple aliasing
            if !do_ranges_overlap(&loc1_range, &loc2_range) {
                //println!("{loc2} will now be {loc1}!");
                alias_local(*loc2, *loc1, ops);
                continue;
            }
            //eprintln!("ranges {loc1_range:?} {loc2_range:?} overlap.");
        }
    }
}
fn do_ranges_overlap(range1: &Range<usize>, range2: &Range<usize>) -> bool {
    range1.start <= range2.end && range2.start <= range1.end
}
fn get_local_access_range(local: u32, ops: &[CILOp]) -> Option<Range<usize>> {
    let mut start = ops.len();
    let mut end = 0;
    for (index, op) in ops.iter().enumerate() {
        match op {
            CILOp::LDLoc(loc) | CILOp::STLoc(loc) | CILOp::LDLocA(loc) => {
                if *loc == local {
                    start = start.min(index);
                    end = end.max(index);
                }
            }
            _ => (),
        }
    }
    if start > end {
        eprintln!("Can't get range!");
        None
    } else {
        Some(start..end)
    }
}
/// Checks if it is possible for a pointer to a local to escape.
fn could_local_ptr_escape(local: u32, ops: &[CILOp]) -> bool {
    for (index, op) in ops.iter().enumerate() {
        match op {
            CILOp::LDLocA(loc) => {
                if *loc == local {
                    assert!(
                        index + 2 < ops.len(),
                        "ERROR: malformed method. LDLocA must be followed by at least 2 ops."
                    );
                    let op2 = &ops[index + 1];
                    let op3 = &ops[index + 2];
                    if let CILOp::LDField(_) = op2 {
                        continue;
                    }
                    if let CILOp::STField(_) = op3 {
                        continue;
                    }
                    return true;
                }
            }
            _ => (),
        }
    }
    return false;
}
fn try_split_locals(method: &mut Method, asm: &Assembly) {
    let splits: Vec<_> = method
        .locals()
        .iter()
        .enumerate()
        .filter(|(_, tpe)| is_type_splitable(tpe))
        .collect();
    //eprintln!("Typewise splits:{splits:?}");
    let splits: Vec<_> = method
        .locals()
        .iter()
        .enumerate()
        .filter(|(_, tpe)| is_type_splitable(tpe))
        .filter(|(local, _)| can_split_local(*local as u32, method.get_ops()))
        .map(|(index, tpe)| (index, tpe.clone()))
        .collect();

    //eprintln!("Spliting locals within method {}", method.name());
    //eprintln!("Locals: {locals:?}.\n\n splits:{splits:?}v \n\nops:{ops:?} \n\n\n",locals = method.locals(),ops = method.get_ops());
    for (split_local, split_tpe) in splits {
        let dotnet_tpe = split_tpe
            .as_dotnet()
            .expect("Can't spilt non-dotnet types!");
        let type_def = asm.get_typedef_by_path(dotnet_tpe.name_path());
        let type_def = type_def.expect("Could not find type!");
        let local_map_start = method.locals().len();
        let morphic_fields: Box<[_]> = type_def.morphic_fields(dotnet_tpe.generics()).collect();
        method.extend_locals(morphic_fields.iter().map(|(name, tpe)| tpe));
        for index in 0..(method.get_ops().len() - 2) {
            //FIXME: this needs to be changed if we ever allow for this to optimize more compilcated split field access patterns.
            let (op1, op2, op3) = (
                &method.get_ops()[index],
                &method.get_ops()[index + 1],
                &method.get_ops()[index + 2],
            );
            // Check if op1 is LDLoc(split_local), otherwise ignore.
            if let CILOp::LDLocA(local) = op1 {
                if *local != split_local as u32 {
                    continue;
                }
            } else {
                continue;
            }
            //eprintln!("Prepalring to preform struct spiling modifications on ops {op1:?} {op2:?} {op3:?} split_local:{split_local:?}.");
            if let CILOp::LDField(field_desc) = op2 {
                let field_idx = morphic_fields
                    .iter()
                    .position(|mfield| mfield.0 == field_desc.name())
                    .expect("Cound not find field during spliting!");
                method.ops_mut()[index] = CILOp::Nop;
                method.ops_mut()[index + 1] = CILOp::LDLoc((field_idx + local_map_start) as u32);
                continue;
            }
            if let CILOp::LDFieldAdress(field_desc) = op2 {
                let field_idx = morphic_fields
                    .iter()
                    .position(|mfield| mfield.0 == field_desc.name())
                    .expect("Cound not find field during spliting!");
                method.ops_mut()[index] = CILOp::Nop;
                method.ops_mut()[index + 1] = CILOp::LDLocA((field_idx + local_map_start) as u32);
                continue;
            }
            if let CILOp::STField(field_desc) = op3 {
                let field_idx = morphic_fields
                    .iter()
                    .position(|mfield| mfield.0 == field_desc.name())
                    .expect("Cound not find field during spliting!");
                method.ops_mut()[index] = CILOp::Nop;
                method.ops_mut()[index + 2] = CILOp::STLoc((field_idx + local_map_start) as u32);
                continue;
            }
            panic!("Invalid field access in field split on ops {op1:?} {op2:?} {op3:?} split_local:{split_local:?} ");
        }
        //todo!("Can't yet split local {split_local:?} of type {split_tpe:?}. type_def:{type_def:?} morphic_fields:{morphic_fields:?}")
    }
}
/// Checks if a local of type `tpe` could potentialy be split.
fn is_type_splitable(tpe: &Type) -> bool {
    if let Type::DotnetType(tref) = tpe {
        tref.is_valuetype() && tref.asm().is_none() //&& (!tref.name_path().contains("/"))
    } else {
        false
    }
}
/// Checks if a local could be split.
fn can_split_local(local: u32, ops: &[CILOp]) -> bool {
    // Local is get/set by value, should not be split.
    if ops.iter().any(|op| {
        if let CILOp::LDLoc(loc) | CILOp::STLoc(loc) = op {
            *loc == local
        } else {
            false
        }
    }) {
        return false;
    }
    // Check if local adress is used ONLY to get fields.
    for (index, op) in ops.iter().enumerate() {
        match op {
            CILOp::LDLocA(loc) => {
                if *loc == local {
                    assert!(
                        index + 2 < ops.len(),
                        "ERROR: malformed method. LDLocA must be followed by at least 2 ops."
                    );
                    let op2 = &ops[index + 1];
                    let op3 = &ops[index + 2];
                    if let CILOp::LDField(_) | CILOp::LDFieldAdress(_) = op2 {
                        continue;
                    }
                    if let CILOp::STField(_) = op3 {
                        continue;
                    }
                    return false;
                }
            }
            _ => (),
        }
    }

    true
}
