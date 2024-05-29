use crate::r#type::TyCache;

use cilly::{asm::Assembly, cil_node::CILNode, cil_root::CILRoot, cil_tree::CILTree};
use rustc_middle::ty::TyCtxt;

pub(crate) fn resolve_global_allocations_tree(
    tree: &mut CILTree,
    arg: &mut Assembly,
    tyctx: TyCtxt,
    tycache: &mut TyCache,
) {
    resolve_global_allocations(tree.root_mut(), arg, tyctx, tycache);
}

pub(crate) fn resolve_global_allocations_node(
    node: &mut CILNode,
    asm: &mut Assembly,
    tyctx: TyCtxt,
    tycache: &mut TyCache,
) {
    match node {
            CILNode::LocAllocAligned {..}=>(),
            CILNode::LdFalse=>(),
            CILNode::LdTrue=>(),
            CILNode::TransmutePtr { val, new_ptr: _ }=>resolve_global_allocations_node(val,asm, tyctx, tycache),
            CILNode::GetStackTop => (),
            CILNode::InspectValue { val, inspect }=>{
                resolve_global_allocations_node(val,asm, tyctx, tycache);
                inspect.iter_mut().for_each(|i|resolve_global_allocations(i,asm, tyctx, tycache));
            }
            CILNode:: PointerToConstValue(bytes)=> *node = CILNode::LDStaticField(Box::new(crate::assembly::add_const_value(asm,*bytes,tyctx))),
            CILNode::LDLoc(_) |
            CILNode::LDArg(_) |
            CILNode::LDLocA(_)|
            CILNode::LDArgA(_) => (),
            CILNode::BlackBox(_) => todo!(),
            CILNode::SizeOf(_) => (),
            CILNode::LDIndI8 { ptr }|
            CILNode::LDIndBool { ptr }|
            CILNode::LDIndI16 { ptr }|
            CILNode::LDIndI32 { ptr }|
            CILNode::LDIndI64 { ptr }|
            CILNode::LDIndU8 { ptr }|
            CILNode::LDIndU16 { ptr }|
            CILNode::LDIndU32 { ptr }|
            CILNode::LDIndU64 { ptr }|
            CILNode::LDIndISize { ptr }|
            CILNode::LDIndPtr { ptr, .. }|
            CILNode::LDIndUSize { ptr }|
            CILNode::LdObj { ptr, .. }|
            CILNode::LDIndF32 { ptr } |
            CILNode::LDIndF64 { ptr } => resolve_global_allocations_node(ptr,asm,tyctx,tycache),
            CILNode::LDFieldAdress { addr, field: _ }|
            CILNode::LDField { addr, field: _ } => resolve_global_allocations_node(addr,asm,tyctx,tycache),
            CILNode::Add(a, b)
            | CILNode::And(a, b)
            | CILNode::Sub(a, b)
            | CILNode::Mul(a, b)
            | CILNode::Div(a, b)
            | CILNode::DivUn(a, b)
            | CILNode::Rem(a, b)
            | CILNode::RemUn(a, b)
            | CILNode::Or(a, b)
            | CILNode::XOr(a, b)
            | CILNode::Shr(a, b)
            | CILNode::Shl(a, b)
            | CILNode::ShrUn(a, b)
            | CILNode::Eq(a, b)
            | CILNode::Lt(a, b)
            | CILNode::LtUn(a, b)
            | CILNode::Gt(a, b)
            | CILNode::GtUn(a, b) => {
                resolve_global_allocations_node(a,asm,tyctx,tycache);
                resolve_global_allocations_node(b,asm,tyctx,tycache);
            }
            CILNode::Call { args, site: _ } |
            CILNode::CallVirt { args, site: _ } =>args.iter_mut().for_each(|arg|resolve_global_allocations_node(arg,asm,tyctx,tycache)),
            CILNode::LdcI64(_) |
            CILNode::LdcU64(_) |
            CILNode::LdcI32(_)  |
            CILNode::LdcU32(_) |
            CILNode::LdcF64(_) |
            CILNode::LdcF32(_) =>(),
            CILNode::LoadGlobalAllocPtr { alloc_id } => {
                *node = CILNode::LDStaticField(crate::assembly::add_allocation(asm, *alloc_id,tyctx,tycache).into());
            }
            CILNode::ConvF64Un(val) |
            CILNode::ConvF32(val)|
            CILNode::ConvF64(val) |
            CILNode::ConvU8(val)|
            CILNode::ConvU16(val)|
            CILNode::ConvU32(val)|
            CILNode::ConvU64(val)|
            CILNode::ZeroExtendToUSize(val)|
            CILNode::ZeroExtendToISize(val)|
            CILNode::MRefToRawPtr(val) |
            CILNode::ConvI8(val) |
            CILNode::ConvI16(val)|
            CILNode::ConvI32(val)|
            CILNode::ConvI64(val) |
            CILNode::ConvISize(val)|
            //CILNode::Volatile(_) => todo!(),
            CILNode::Neg(val) |
            CILNode::Not(val) =>resolve_global_allocations_node(val,asm,tyctx,tycache),

            CILNode::TemporaryLocal(tmp_loc) => {
                tmp_loc.1.iter_mut().for_each(|tree|resolve_global_allocations(tree,asm,tyctx,tycache));
                resolve_global_allocations_node(&mut tmp_loc.2,asm,tyctx,tycache);
            },
            CILNode::SubTrees(trees, main) =>{
                trees.iter_mut().for_each(|arg|resolve_global_allocations(arg,asm,tyctx,tycache));
                resolve_global_allocations_node(main,asm,tyctx,tycache);
            }
            CILNode::LoadAddresOfTMPLocal => (),
            CILNode::LoadTMPLocal => (),
            CILNode::LDFtn(_) => (),
            CILNode::LDTypeToken(_) => (),
            CILNode::NewObj { site: _, args } => args.iter_mut().for_each(|arg|resolve_global_allocations_node(arg,asm,tyctx,tycache)),
            CILNode::LdStr(_) => (),
            CILNode::CallI(sig_ptr_args) => {
                resolve_global_allocations_node(&mut sig_ptr_args.1,asm, tyctx,tycache);
                sig_ptr_args.2.iter_mut().for_each(|arg|resolve_global_allocations_node(arg,asm,tyctx,tycache));
            }
            CILNode::LDStaticField(_sfield)=>(),
            CILNode::LDLen { arr } =>{
                resolve_global_allocations_node(arr,asm, tyctx,tycache);
            }
            CILNode::LDElelemRef { arr, idx }=>{
                resolve_global_allocations_node(arr,asm, tyctx,tycache);
                resolve_global_allocations_node(idx,asm, tyctx,tycache);
            }
        }
}

pub(crate) fn resolve_global_allocations(
    root: &mut CILRoot,
    asm: &mut Assembly,
    tyctx: TyCtxt,
    tycache: &mut TyCache,
) {
    match root {
        CILRoot::SourceFileInfo(_) => (),
        CILRoot::STLoc { local: _, tree } => {
            resolve_global_allocations_node(tree, asm, tyctx, tycache);
        }
        CILRoot::BTrue {
            target: _,
            sub_target: _,
            cond: ops,
        }
        | CILRoot::BFalse {
            target: _,
            sub_target: _,
            cond: ops,
        } => resolve_global_allocations_node(ops, asm, tyctx, tycache),
        CILRoot::BEq {
            target: _,
            sub_target: _,
            a,
            b,
        }
        | CILRoot::BNe {
            target: _,
            sub_target: _,
            a,
            b,
        } => {
            resolve_global_allocations_node(a, asm, tyctx, tycache);
            resolve_global_allocations_node(b, asm, tyctx, tycache);
        }
        CILRoot::GoTo {
            target: _,
            sub_target: _,
        } => (),
        CILRoot::CallVirt { site: _, args } | CILRoot::Call { site: _, args } => args
            .iter_mut()
            .for_each(|arg| resolve_global_allocations_node(arg, asm, tyctx, tycache)),
        CILRoot::SetField {
            addr,
            value,
            desc: _,
        } => {
            resolve_global_allocations_node(addr, asm, tyctx, tycache);
            resolve_global_allocations_node(value, asm, tyctx, tycache);
        }
        CILRoot::CpBlk { src, dst, len } => {
            resolve_global_allocations_node(src, asm, tyctx, tycache);
            resolve_global_allocations_node(dst, asm, tyctx, tycache);
            resolve_global_allocations_node(len, asm, tyctx, tycache);
        }
        CILRoot::STIndI8(addr_calc, value_calc)
        | CILRoot::STIndI16(addr_calc, value_calc)
        | CILRoot::STIndI32(addr_calc, value_calc)
        | CILRoot::STIndI64(addr_calc, value_calc)
        | CILRoot::STIndISize(addr_calc, value_calc)
        | CILRoot::STIndF64(addr_calc, value_calc)
        | CILRoot::STIndF32(addr_calc, value_calc)
        | CILRoot::STObj {
            addr_calc,
            value_calc,
            ..
        } => {
            resolve_global_allocations_node(addr_calc, asm, tyctx, tycache);
            resolve_global_allocations_node(value_calc, asm, tyctx, tycache);
        }
        CILRoot::STArg { arg: _, tree } => {
            resolve_global_allocations_node(tree, asm, tyctx, tycache)
        }
        CILRoot::Break => (),
        CILRoot::Nop => (),
        CILRoot::InitBlk { dst, val, count } => {
            resolve_global_allocations_node(dst, asm, tyctx, tycache);
            resolve_global_allocations_node(val, asm, tyctx, tycache);
            resolve_global_allocations_node(count, asm, tyctx, tycache);
        }

        CILRoot::Ret { tree } | CILRoot::Pop { tree } | CILRoot::Throw(tree) => {
            resolve_global_allocations_node(tree, asm, tyctx, tycache);
        }
        CILRoot::VoidRet => (),

        CILRoot::ReThrow => (),
        CILRoot::CallI {
            sig: _,
            fn_ptr,
            args,
        } => {
            resolve_global_allocations_node(fn_ptr, asm, tyctx, tycache);
            args.iter_mut()
                .for_each(|arg| resolve_global_allocations_node(arg, asm, tyctx, tycache));
        }
        // Jump pads CAN'T ever allocate.
        CILRoot::JumpingPad { .. } => (),
        CILRoot::SetTMPLocal { value } => {
            resolve_global_allocations_node(value, asm, tyctx, tycache)
        }
        CILRoot::SetStaticField { descr: _, value } => {
            resolve_global_allocations_node(value, asm, tyctx, tycache);
        }
    }
}
/*
pub(crate) fn validate(&self, method: &crate::method::Method) -> Result<(), String> {
    match self {
        CILRoot::STIndI8(addr, val) => {
            let addr = addr.validate(method)?;
            let val = val.validate(method)?;
            match &addr {
                Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                    Type::I8 | Type::U8 => (),
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                        ))
                    }
                },
                _ => {
                    return Err(format!(
                        "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                    ))
                }
            }
            match val{
                Type::I8 | Type::U8 => Ok(()),
                _=>Err(format!("Can't indirectly set a valur of type i8/u8 because the provided value is {val:?}")),
            }
        }

        CILRoot::STIndISize(addr, val) => {
            let addr = addr.validate(method)?;
            let val = val.validate(method)?;
            match &addr {
                Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                    Type::Ptr(_) | Type::ManagedReference(_) | Type::USize | Type::ISize => (),
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                        ))
                    }
                },
                _ => {
                    return Err(format!(
                        "Can't set a vaule of type i8/u8 at address of type {addr:?}"
                    ))
                }
            }
            match val{
                Type::I8 | Type::U8 => Ok(()),
                _=>Err(format!("Can't indirectly set a valur of type i8/u8 because the provided value is {val:?}")),
            }
        }
        CILRoot::Break => Ok(()),
        CILRoot::JumpingPad { ops: _ } => Ok(()),
        CILRoot::BTrue {
            target: _,
            sub_target: _,
            cond,
        } => {
            // Just check that `cond` is a boolean.
            let cond = cond.validate(method)?;
            if cond != Type::Bool {
                Err(format!(
                    "BTrue must have a boolean argument. cond is:{cond:?}"
                ))
            } else {
                Ok(())
            }
        }
        CILRoot::GoTo {
            target: _,
            sub_target: _,
        } => Ok(()),
        CILRoot::STLoc { local, tree } => {
            let expected_tpe = if let Some(loc) = method.locals().get(*local as usize) {
                loc
            } else {
                return Err(format!("Local out of range! Local{local:?}"));
            };
            let got = tree.validate(method)?;
            if expected_tpe.1 != got {
                Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting local {local:?}"))
            } else {
                Ok(())
            }
        }
        CILRoot::STArg { arg, tree } => {
            let expected_tpe = if let Some(arg) = method.locals().get(*arg as usize) {
                arg
            } else {
                return Err(format!("Arg out of range! Arg {arg:?}"));
            };
            let got = tree.validate(method)?;
            if expected_tpe.1 != got {
                Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting local {arg:?}"))
            } else {
                Ok(())
            }
        }
        CILRoot::Call { site, args } => {
            if site.inputs().len() != args.len() {
                return Err(format!(
                    "Expected {} arguments, got {}",
                    site.explicit_inputs().len(),
                    args.len()
                ));
            }
            for (arg, tpe) in args.iter().zip(site.inputs().iter()) {
                let arg = arg.validate(method)?;
                if arg != *tpe {
                    return Err(format!(
                        "Expected an argument of type {tpe:?}, but got {arg:?}"
                    ));
                }
            }
            Ok(())
        }
        CILRoot::CallI { args, sig, fn_ptr } => {
            let _ptr = fn_ptr.validate(method)?;
            if sig.inputs().len() != args.len() {
                return Err(format!(
                    "Expected {} arguments, got {}",
                    sig.inputs().len(),
                    args.len()
                ));
            }
            for (arg, tpe) in args.iter().zip(sig.inputs().iter()) {
                let arg = arg.validate(method)?;
                if arg != *tpe {
                    return Err(format!(
                        "Expected an argument of type {tpe:?}, but got {arg:?}"
                    ));
                }
            }
            Ok(())
        }
        CILRoot::VoidRet => Ok(()),
        CILRoot::SourceFileInfo(_) => Ok(()),
        CILRoot::Nop => Ok(()),
        CILRoot::Throw(execption) => {
            let tpe = execption.validate(method)?;
            if tpe.as_dotnet().is_some() {
                Ok(())
            } else {
                Err("`throw` instruction suplied with a non-object type.".into())
            }
        }
        CILRoot::Ret { tree } => {
            let expected = method.sig().output();
            let got = tree.validate(method)?;
            if got != *expected {
                Err(format!(
                    "Mismatched return type. Expected {expected:?} got {got:?}"
                ))
            } else {
                Ok(())
            }
        }
        CILRoot::SetField { addr, value, desc } => {
            let addr = addr.validate(method)?;
            let value = value.validate(method)?;
            if *desc.tpe() != value {
                return Err(format!(
                    "Mismatched field type. Expected {expected:?} got {value:?}",
                    expected = desc.tpe(),
                ));
            }
            match addr {
                Type::ManagedReference(tpe) | Type::Ptr(tpe) => {
                    if tpe.as_dotnet() != Some(desc.owner().clone()) {
                        return Err(format!(
                            "Mismatched pointer type. Expected {desc:?} got {tpe:?}"
                        ));
                    }
                }
                _ => (),
            }
            Ok(())
        }
        _ => todo!("Can't check the type safety of cil root {self:?}"),
    }
}*/
