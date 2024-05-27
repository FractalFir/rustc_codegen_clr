use crate::{
    cil::CILOp,
    method::Method,
    r#type::{TyCache, Type},
};

use cilly::{cil_node::CILNode, cil_root::CILRoot, cil_tree::CILTree};
use rustc_middle::ty::TyCtxt;



pub fn validate(_tree: &CILTree, method: &Method) -> Result<(), String> {
    //self.tree.validate(method)
    todo!("method:{method:?}");
}

pub(crate) fn resolve_global_allocations_tree(
    tree: &mut CILTree,
    arg: &mut crate::assembly::Assembly,
    tyctx: TyCtxt,
    tycache: &mut TyCache,
) {
    resolve_global_allocations(tree.root_mut(), arg, tyctx, tycache);
}
/// Converts a `CILTree` into a list of `CILOp`.
#[must_use]
pub fn into_ops_tree(tree: &CILTree) -> Vec<CILOp> {
    into_ops(tree.root())
}

/// Appends an op to a vector.
#[must_use]
pub fn append_vec(mut vec: Vec<CILOp>, by: CILOp) -> Vec<CILOp> {
    vec.push(by);
    vec
}
#[must_use]
pub fn flatten(node: &CILNode) -> Vec<CILOp> {
    let mut ops = match node {
        CILNode::LocAllocAligned { tpe, align } => vec![
            // Alloc buff
            CILOp::SizeOf(tpe.clone()),
            CILOp::LdcU64(*align),
            CILOp::ConvISize(false),
            CILOp::Add,
            CILOp::LocAlloc,
            // Adjust align
            CILOp::Dup,
            CILOp::LdcU64(*align - 1),
            CILOp::Add,
            CILOp::LdcU64(*align),
            CILOp::Rem,
            CILOp::Sub,
            CILOp::LdcU64(*align - 1),
            CILOp::Add,
        ],
        CILNode::LdFalse => vec![CILOp::LdcI32(0)],
        CILNode::LdTrue => vec![CILOp::LdcI32(1)],
        CILNode::TransmutePtr { val, new_ptr: _ } => flatten(val),
        CILNode::GetStackTop => vec![],
        CILNode::InspectValue { val, inspect } => {
            let mut ops = flatten(val);
            ops.push(CILOp::Dup);
            ops.extend(inspect.iter().flat_map(into_ops));
            ops
        }
        CILNode::PointerToConstValue(_value) => {
            panic!("ERROR: const values must be allocated before CILOp flattening phase")
        }
        CILNode::CallI(sig_ptr_args) => {
            let mut ops: Vec<_> = sig_ptr_args.2.iter().flat_map(flatten).collect();
            ops.extend(flatten(&sig_ptr_args.1));
            ops.push(CILOp::CallI(sig_ptr_args.0.clone().into()));
            ops
        }
        CILNode::SubTrees(trees, root) => {
            let mut flattened: Vec<_> = trees.iter().flat_map(into_ops).collect();
            flattened.extend(flatten(root));
            flattened
        }
        CILNode::LoadTMPLocal => {
            panic!("Unresolved temporary local during the CIL flattening phase!")
        }
        CILNode::LoadAddresOfTMPLocal => {
            panic!("Unresolved temporary local during the CIL flattening phase!")
        }
        CILNode::LDFtn(site) => vec![CILOp::LDFtn(site.clone())],
        CILNode::LDTypeToken(tpe) => vec![CILOp::LDTypeToken(tpe.clone())],
        CILNode::TemporaryLocal(tuple) => {
            panic!("Unresolved temporary local `{tuple:?}` during the CIL flattening phase!")
        }
        CILNode::LDLoc(local) => vec![CILOp::LDLoc(*local)],
        CILNode::LDArg(local) => vec![CILOp::LDArg(*local)],
        CILNode::SizeOf(tpe) => match **tpe {
            Type::Void => vec![CILOp::LdcU32(0)],
            _ => vec![CILOp::SizeOf(tpe.clone())],
        },
        CILNode::LDArgA(local) => vec![CILOp::LDArgA(*local)],
        CILNode::LDLocA(local) => vec![CILOp::LDLocA(*local)],

        CILNode::BlackBox(inner) => flatten(inner),

        CILNode::ZeroExtendToUSize(inner) => append_vec(flatten(inner), CILOp::ConvUSize(false)),
        CILNode::ZeroExtendToISize(inner) => append_vec(flatten(inner), CILOp::ConvUSize(false)),
        CILNode::MRefToRawPtr(inner) => append_vec(flatten(inner), CILOp::ConvUSize(false)),

        CILNode::ConvU8(inner) => append_vec(flatten(inner), CILOp::ConvU8(false)),
        CILNode::ConvU16(inner) => append_vec(flatten(inner), CILOp::ConvU16(false)),
        CILNode::ConvU32(inner) => append_vec(flatten(inner), CILOp::ConvU32(false)),
        CILNode::ConvU64(inner) => append_vec(flatten(inner), CILOp::ConvU64(false)),

        CILNode::ConvISize(inner) => append_vec(flatten(inner), CILOp::ConvISize(false)),
        CILNode::ConvI8(inner) => append_vec(flatten(inner), CILOp::ConvI8(false)),
        CILNode::ConvI16(inner) => append_vec(flatten(inner), CILOp::ConvI16(false)),
        CILNode::ConvI32(inner) => append_vec(flatten(inner), CILOp::ConvI32(false)),
        CILNode::ConvI64(inner) => append_vec(flatten(inner), CILOp::ConvI64(false)),

        CILNode::ConvF32(inner) => append_vec(flatten(inner), CILOp::ConvF32),
        CILNode::ConvF64(inner) => append_vec(flatten(inner), CILOp::ConvF64),
        CILNode::ConvF64Un(inner) => append_vec(flatten(inner), CILOp::ConvF64Un),
        CILNode::LDIndI8 { ptr } | CILNode::LDIndBool { ptr } => {
            append_vec(flatten(ptr), CILOp::LDIndI8)
        }
        CILNode::LDIndI16 { ptr } => append_vec(flatten(ptr), CILOp::LDIndI16),
        CILNode::LDIndI32 { ptr } => append_vec(flatten(ptr), CILOp::LDIndI32),
        CILNode::LDIndI64 { ptr } => append_vec(flatten(ptr), CILOp::LDIndI64),
        CILNode::LDIndISize { ptr } => append_vec(flatten(ptr), CILOp::LDIndISize),
        CILNode::LDIndPtr { ptr, .. } => append_vec(flatten(ptr), CILOp::LDIndISize),
        CILNode::LDIndUSize { ptr } => append_vec(flatten(ptr), CILOp::LDIndISize),
        CILNode::LDIndU8 { ptr } => append_vec(flatten(ptr), CILOp::LDIndU8),
        CILNode::LDIndU16 { ptr } => append_vec(flatten(ptr), CILOp::LDIndU16),
        CILNode::LDIndU32 { ptr } => append_vec(flatten(ptr), CILOp::LDIndU32),
        CILNode::LDIndU64 { ptr } => append_vec(flatten(ptr), CILOp::LDIndU64),
        CILNode::LdObj { ptr, obj } => append_vec(flatten(ptr), CILOp::LdObj(obj.clone())),

        CILNode::Neg(inner) => append_vec(flatten(inner), CILOp::Neg),
        CILNode::Not(inner) => append_vec(flatten(inner), CILOp::Not),

        CILNode::LDFieldAdress { addr, field } => {
            append_vec(flatten(addr), CILOp::LDFieldAdress(field.clone()))
        }
        CILNode::LDField { addr, field } => {
            append_vec(flatten(addr), CILOp::LDField(field.clone()))
        }
        CILNode::LDIndF32 { ptr } => append_vec(flatten(ptr), CILOp::LDIndF32),
        CILNode::LDIndF64 { ptr } => append_vec(flatten(ptr), CILOp::LDIndF64),
        CILNode::Add(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Add);
            res
        }
        CILNode::And(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::And);
            res
        }
        CILNode::Shr(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Shr);
            res
        }
        CILNode::Shl(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Shl);
            res
        }
        CILNode::ShrUn(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::ShrUn);
            res
        }
        CILNode::Or(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Or);
            res
        }
        CILNode::XOr(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::XOr);
            res
        }
        CILNode::Div(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Div);
            res
        }
        CILNode::DivUn(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::DivUn);
            res
        }
        CILNode::Rem(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Rem);
            res
        }
        CILNode::RemUn(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::RemUn);
            res
        }
        CILNode::Sub(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Sub);
            res
        }
        CILNode::Eq(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Eq);
            res
        }
        CILNode::Lt(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Lt);
            res
        }
        CILNode::LtUn(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::LtUn);
            res
        }
        CILNode::Gt(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Gt);
            res
        }
        CILNode::GtUn(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::GtUn);
            res
        }
        CILNode::Mul(a, b) => {
            let mut res = flatten(a);
            res.extend(flatten(b));
            res.push(CILOp::Mul);
            res
        }

        //CILNode::RawOpsParrentless { ops } => ops.clone().into(),
        CILNode::Call { args, site } => {
            let mut res: Vec<CILOp> = args.iter().flat_map(flatten).collect();
            res.push(CILOp::Call(site.clone()));
            res
        }
        CILNode::NewObj { args, site } => {
            let mut res: Vec<CILOp> = args.iter().flat_map(flatten).collect();
            res.push(CILOp::NewObj(site.clone()));
            res
        }
        CILNode::CallVirt { args, site } => {
            let mut res: Vec<CILOp> = args.iter().flat_map(flatten).collect();
            res.push(CILOp::CallVirt(site.clone()));
            res
        }
        CILNode::LdcI64(val) => vec![CILOp::LdcI64(*val)],
        CILNode::LdcU64(val) => vec![CILOp::LdcU64(*val)],
        CILNode::LdcI32(val) => vec![CILOp::LdcI32(*val)],
        CILNode::LdcU32(val) => vec![CILOp::LdcU32(*val)],
        CILNode::LdcF64(val) => vec![CILOp::LdcF64(*val)],
        CILNode::LdcF32(val) => vec![CILOp::LdcF32(*val)],
        CILNode::LdStr(string) => vec![CILOp::LdStr(string.clone())],
        CILNode::LoadGlobalAllocPtr { alloc_id } => {
            panic!("Unresolved global alloc with id:{alloc_id}  during the CIL flattening phase!")
        }
        CILNode::LDStaticField(sfield) => vec![CILOp::LDStaticField(sfield.clone())],
        CILNode::LDLen { arr } => {
            let mut res = flatten(arr);
            res.push(CILOp::LDLen);
            res
        }
        CILNode::LDElelemRef { arr, idx } => {
            let mut res = flatten(arr);
            res.extend(flatten(idx));
            res.push(CILOp::LDElelemRef);
            res
        }
    };
    {
        ops.push(CILOp::Pop);
        crate::utilis::check_debugable(&ops, node, false);
        ops.pop();
    }
    ops
}
pub(crate) fn resolve_global_allocations_node(
    node: &mut CILNode,
    asm: &mut crate::assembly::Assembly,
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
            CILNode:: PointerToConstValue(bytes)=> *node = CILNode::LDStaticField(Box::new(asm.add_const_value(*bytes,tyctx))),
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
                *node = CILNode::LDStaticField(asm.add_allocation(*alloc_id,tyctx,tycache).into());
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
#[must_use]
pub fn into_ops(node: &CILRoot) -> Vec<CILOp> {
    match std::panic::catch_unwind(|| {
        match node {
            CILRoot::SourceFileInfo(sfi) => vec![CILOp::SourceFileInfo(sfi.clone())],
            //CILRoot::LabelStart(val)=> vec![CILOp::LabelStart(val)],
            //CILRoot::LabelEnd(val)=> vec![CILOp::LabelEnd(val)],
            CILRoot::ReThrow => vec![CILOp::ReThrow],
            CILRoot::Throw(tree) => append_vec(flatten(tree), CILOp::Throw),
            CILRoot::Ret { tree } => append_vec(flatten(tree), CILOp::Ret),
            CILRoot::Pop { tree } => append_vec(flatten(tree), CILOp::Pop),
            CILRoot::VoidRet => vec![CILOp::Ret],
            CILRoot::STLoc { local, tree } => append_vec(flatten(tree), CILOp::STLoc(*local)),
            CILRoot::STArg { arg, tree } => append_vec(flatten(tree), CILOp::STArg(*arg)),
            CILRoot::BTrue {
                target,
                cond: ops,
                sub_target,
            } => append_vec(flatten(ops), CILOp::BTrue(*target, *sub_target)),
            CILRoot::GoTo { target, sub_target } => vec![CILOp::GoTo(*target, *sub_target)],
            CILRoot::Call { site, args } => {
                let mut args: Vec<_> = args.iter().flat_map(flatten).collect();
                args.push(CILOp::Call(site.clone().into()));
                args
            }
            CILRoot::CallI { sig, fn_ptr, args } => {
                let mut ops: Vec<_> = args.iter().flat_map(flatten).collect();
                ops.extend(flatten(fn_ptr));
                ops.push(CILOp::CallI(sig.clone().into()));
                ops
            }
            CILRoot::CallVirt { site, args } => {
                let mut args: Vec<_> = args.iter().flat_map(flatten).collect();
                args.push(CILOp::CallVirt(site.clone().into()));
                args
            }
            CILRoot::SetField {
                addr,
                value: root,
                desc,
            } => {
                let mut res = flatten(addr);
                res.extend(flatten(root));
                res.push(CILOp::STField(desc.clone().into()));
                res
            }
            CILRoot::CpBlk { src, dst, len } => {
                // Argument order: destination, source, length
                let mut res = flatten(dst);
                res.extend(flatten(src));
                res.extend(flatten(len));
                res.push(CILOp::CpBlk);
                res
            }
            CILRoot::InitBlk { dst, val, count } => {
                let mut res = flatten(dst);
                res.extend(flatten(val));
                res.extend(flatten(count));
                res.push(CILOp::InitBlk);
                res
            }
            CILRoot::STIndI8(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndI8);
                res
            }
            CILRoot::STIndI16(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndI16);
                res
            }
            CILRoot::STIndI32(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndI32);
                res
            }
            CILRoot::STIndI64(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndI64);
                res
            }
            CILRoot::STIndISize(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndISize);
                res
            }
            CILRoot::STIndF64(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndF64);
                res
            }
            CILRoot::STIndF32(addr, val) => {
                let mut res = flatten(addr);
                res.extend(flatten(val));
                res.push(CILOp::STIndF32);
                res
            }
            CILRoot::Break => vec![CILOp::Break],
            CILRoot::Nop => vec![CILOp::Nop],
            CILRoot::JumpingPad { target, source } => {
                vec![CILOp::Label(*source, *target), CILOp::Leave(*target)]
            }
            CILRoot::STObj {
                tpe,
                addr_calc,
                value_calc,
            } => {
                let mut res = flatten(addr_calc);
                res.extend(flatten(value_calc));
                res.push(CILOp::STObj(tpe.clone()));
                res
            }
            CILRoot::SetTMPLocal { .. } => {
                todo!("Can't flatten unresolved root!")
            }
            CILRoot::SetStaticField { descr, value } => {
                append_vec(flatten(value), CILOp::STStaticField(descr.clone().into()))
            }
        }
    }) {
        Ok(ok) => ok,
        Err(_) => panic!("Could not flatten tree {node:?}"),
    }
}

pub(crate) fn resolve_global_allocations(
    root: &mut CILRoot,
    asm: &mut crate::assembly::Assembly,
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
        } => resolve_global_allocations_node(ops, asm, tyctx, tycache),
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
