use crate::{method::Method, cil::CILOp, r#type::Type, assembly::Assembly};

pub fn remove_unused_locals(method: &mut Method) {
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
/// A "Unused" local is one that is never written to or read from.
fn is_local_unused(ops: &[CILOp], local: u32) -> bool {
    !ops.iter().any(|op| match op {
        CILOp::LDLoc(loc) => *loc == local,
        CILOp::LDLocA(loc) => *loc == local,
        CILOp::STLoc(loc) => *loc == local,
        _ => false,
    })
}
pub fn try_split_locals(method: &mut Method, asm: &Assembly) {
    let splits: Vec<_> = method
        .locals()
        .iter()
        .enumerate()
        .filter(|(_, tpe)| is_type_splitable(&tpe.1))
        .filter(|(local, _)| can_split_local(*local as u32, method.get_ops()))
        .map(|(index, tpe)| (index, tpe.clone()))
        .collect();

    for (split_local, split_tpe) in splits {
        let dotnet_tpe = split_tpe
            .1
            .as_dotnet()
            .expect("Can't spilt non-dotnet types!");
        if dotnet_tpe.name_path().contains("PtrRepr") {
            eprintln!("WARINING: PtrRepr is bugged and causes issues during optimzation. It will not be optimized. TODO: figure out why the field `const_ptr` can't be found.");
            continue;
        }
        let type_def = asm.get_typedef_by_path(dotnet_tpe.name_path());
        let type_def = if let Some(type_def) = type_def {
            type_def
        } else {
            continue;
        };
        let local_map_start = method.locals().len();
        let morphic_fields: Option<Box<[_]>> =
            type_def.morphic_fields(dotnet_tpe.generics()).collect();
        let morphic_fields = if let Some(morphic_fields) = morphic_fields {
            morphic_fields
        } else {
            continue;
        };
        method.extend_locals(morphic_fields.iter().map(|(_name, tpe)| tpe));
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
            if let CILOp::LDField(field_desc) = op2 {
                let field_idx = if let Some(field) = morphic_fields
                    .iter()
                    .position(|mfield| mfield.0 == field_desc.name())
                {
                    field
                } else {
                    panic!("ERROR: field spliting failed because field {field_desc:?} could not be found. This may be caused by an error during codegen");
                };
                method.ops_mut()[index] = CILOp::Nop;
                method.ops_mut()[index + 1] = CILOp::LDLoc((field_idx + local_map_start) as u32);
                continue;
            }
            if let CILOp::LDFieldAdress(field_desc) = op2 {
                let field_idx = if let Some(field) = morphic_fields
                    .iter()
                    .position(|mfield| mfield.0 == field_desc.name())
                {
                    field
                } else {
                    panic!("ERROR: field spliting failed because field {field_desc:?} could not be found. This may be caused by an error during codegen");
                };
                method.ops_mut()[index] = CILOp::Nop;
                method.ops_mut()[index + 1] = CILOp::LDLocA((field_idx + local_map_start) as u32);
                continue;
            }
            if let CILOp::STField(field_desc) = op3 {
                let field_idx = if let Some(field) = morphic_fields
                    .iter()
                    .position(|mfield| mfield.0 == field_desc.name())
                {
                    field
                } else {
                    panic!("ERROR: field spliting failed because field {field_desc:?} could not be found. This may be caused by an error during codegen");
                };
                method.ops_mut()[index] = CILOp::Nop;
                method.ops_mut()[index + 2] = CILOp::STLoc((field_idx + local_map_start) as u32);
                continue;
            }
            panic!("Invalid field access in field split on ops {op1:?} {op2:?} {op3:?} split_local:{split_local:?} ");
        }
        //todo!("Can't yet split local {split_local:?} of type {split_tpe:?}. type_def:{type_def:?} morphic_fields:{morphic_fields:?}")
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
/// Checks if a local of type `tpe` could potentialy be split.
fn is_type_splitable(tpe: &Type) -> bool {
    if let Type::DotnetType(tref) = tpe {
        tref.is_valuetype() && tref.asm().is_none() //&& (!tref.name_path().contains("/"))
    } else {
        false
    }
}
