use super::append_vec;
use crate::{
    cil::{CILOp, CallSite, FieldDescriptor},
    cil_tree::cil_node::CILNode,
    function_sig::FnSig,
    r#type::Type,
};
use rustc_middle::ty::TyCtxt;
use serde::{Deserialize, Serialize};
#[derive(Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum CILRoot {
    STLoc {
        local: u32,
        tree: CILNode,
    },
    BTrue {
        target: u32,
        sub_target: u32,
        ops: CILNode,
    },
    GoTo {
        target: u32,
        sub_target: u32,
    },
    Call {
        site: CallSite,
        args: Box<[CILNode]>,
    },
    SetField {
        addr: CILNode,
        value: CILNode,
        desc: FieldDescriptor,
    },
    SetTMPLocal {
        value: CILNode,
    },
    CpBlk {
        src: CILNode,
        dst: CILNode,
        len: CILNode,
    },
    STIndI8(CILNode, CILNode),
    STIndI16(CILNode, CILNode),
    STIndI32(CILNode, CILNode),
    STIndI64(CILNode, CILNode),
    STIndISize(CILNode, CILNode),
    STIndF64(CILNode, CILNode),
    STIndF32(CILNode, CILNode),
    STObj {
        tpe: Box<Type>,
        addr_calc: CILNode,
        value_calc: CILNode,
    },
    STArg {
        arg: u32,
        tree: CILNode,
    },
    Break,
    Nop,
    InitBlk {
        dst: CILNode,
        val: CILNode,
        count: CILNode,
    },
    CallVirt {
        site: CallSite,
        args: Box<[CILNode]>,
    },
    Ret {
        tree: CILNode,
    },
    Pop {
        tree: CILNode,
    },
    VoidRet,
    Throw(CILNode),
    ReThrow,
    CallI {
        sig: FnSig,
        fn_ptr: CILNode,
        args: Box<[CILNode]>,
    },
    JumpingPad {
        ops: Box<[CILOp]>,
    },
    SetStaticField {
        descr: crate::cil::StaticFieldDescriptor,
        value: CILNode,
    },
    //LabelStart(u32),
    //LabelEnd(u32),
}
impl CILRoot {
    pub fn opt(&mut self) {
        match self {
            CILRoot::STLoc { local, tree } => tree.opt(),
            CILRoot::BTrue {
                target,
                sub_target,
                ops,
            } => ops.opt(),
            CILRoot::GoTo { target, sub_target } => (),
            CILRoot::Call { site, args } => args.iter_mut().for_each(|arg| arg.opt()),
            CILRoot::SetField {
                addr: fld_addr,
                value,
                desc,
            } => {
                fld_addr.opt();
                value.opt();
                match fld_addr {
                    CILNode::ConvUSize(addr) => match addr.as_mut() {
                        CILNode::LDLocA(_) | CILNode::LDFieldAdress { .. } => {
                            *fld_addr = addr.as_ref().clone()
                        }
                        _ => (),
                    },
                    _ => (),
                }
            }
            CILRoot::CpBlk { src, dst, len } => {
                src.opt();
                dst.opt();
                len.opt();
            }
            CILRoot::STIndI8(addr, val)
            | CILRoot::STIndI16(addr, val)
            | CILRoot::STIndI32(addr, val)
            | CILRoot::STIndI64(addr, val)
            | CILRoot::STIndISize(addr, val)
            | CILRoot::STIndF64(addr, val)
            | CILRoot::STIndF32(addr, val) => {
                addr.opt();
                val.opt();
            }
            CILRoot::STObj {
                tpe,
                addr_calc,
                value_calc,
            } => {
                addr_calc.opt();
                value_calc.opt();
            }
            CILRoot::STArg { arg, tree } => tree.opt(),
            CILRoot::Break => (),
            CILRoot::Nop => (),
            CILRoot::InitBlk { dst, val, count } => {
                val.opt();
                dst.opt();
                count.opt();
            }
            CILRoot::CallVirt { site, args } => args.iter_mut().for_each(|arg| arg.opt()),
            CILRoot::Ret { tree } => tree.opt(),
            CILRoot::Pop { tree } => tree.opt(),
            CILRoot::VoidRet => (),
            CILRoot::Throw(ops) => ops.opt(),
            CILRoot::ReThrow => (),
            CILRoot::CallI { sig, fn_ptr, args } => {
                args.iter_mut().for_each(|arg| arg.opt());
                fn_ptr.opt();
            }
            CILRoot::JumpingPad { ops } => (),
            CILRoot::SetTMPLocal { value } => value.opt(),
            CILRoot::SetStaticField { descr, value } => value.opt(),
        }
    }
    pub fn throw(msg: &str) -> Self {
        let mut class =
            crate::r#type::DotnetTypeRef::new(Some("System.Runtime"), "System.Exception");
        class.set_valuetype(false);
        let name = ".ctor".into();
        let signature = crate::function_sig::FnSig::new(
            &[class.clone().into(), crate::utilis::string_class().into()],
            &crate::r#type::Type::Void,
        );
        Self::Throw(CILNode::NewObj {
            site: CallSite::boxed(Some(class), name, signature, false),
            args: [CILNode::LdStr(msg.into())].into(),
        })
    }
    pub fn debug(msg: &str) -> Self {
        let mut class = crate::r#type::DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "WriteLine".into();
        let signature = crate::function_sig::FnSig::new(
            &[crate::utilis::string_class().into()],
            &crate::r#type::Type::Void,
        );
        Self::Call {
            site: CallSite::new(Some(class), name, signature, true),
            args: [CILNode::LdStr(msg.into())].into(),
        }
    }
    pub fn into_ops(&self) -> Vec<CILOp> {
        match self {
            //Self::LabelStart(val)=> vec![CILOp::LabelStart(val)],
            //Self::LabelEnd(val)=> vec![CILOp::LabelEnd(val)],
            Self::ReThrow => vec![CILOp::ReThrow],
            Self::Throw(tree) => append_vec(tree.flatten(), CILOp::Throw),
            Self::Ret { tree } => append_vec(tree.flatten(), CILOp::Ret),
            Self::Pop { tree } => append_vec(tree.flatten(), CILOp::Pop),
            Self::VoidRet => vec![CILOp::Ret],
            Self::STLoc { local, tree } => append_vec(tree.flatten(), CILOp::STLoc(*local)),
            Self::STArg { arg, tree } => append_vec(tree.flatten(), CILOp::STArg(*arg)),
            Self::BTrue {
                target,
                ops,
                sub_target,
            } => append_vec(ops.flatten(), CILOp::BTrue(*target, *sub_target)),
            Self::GoTo { target, sub_target } => vec![CILOp::GoTo(*target, *sub_target)],
            Self::Call { site, args } => {
                let mut args: Vec<_> = args.iter().flat_map(|arg| arg.flatten()).collect();
                args.push(CILOp::Call(site.clone().into()));
                args
            }
            Self::CallI { sig, fn_ptr, args } => {
                let mut ops: Vec<_> = fn_ptr.flatten();
                ops.extend(args.iter().flat_map(|arg| arg.flatten()));
                ops.push(CILOp::CallI(sig.clone().into()));
                ops
            }
            Self::CallVirt { site, args } => {
                let mut args: Vec<_> = args.iter().flat_map(|arg| arg.flatten()).collect();
                args.push(CILOp::CallVirt(site.clone().into()));
                args
            }
            Self::SetField {
                addr,
                value: root,
                desc,
            } => {
                let mut res = addr.flatten();
                res.extend(root.flatten());
                res.push(CILOp::STField(desc.clone().into()));
                res
            }
            Self::CpBlk { src, dst, len } => {
                let mut res = src.flatten();
                res.extend(dst.flatten());
                res.extend(len.flatten());
                res.push(CILOp::CpBlk);
                res
            }
            Self::InitBlk { dst, val, count } => {
                let mut res = dst.flatten();
                res.extend(val.flatten());
                res.extend(count.flatten());
                res.push(CILOp::CpBlk);
                res
            }
            Self::STIndI8(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndI8);
                res
            }
            Self::STIndI16(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndI16);
                res
            }
            Self::STIndI32(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndI32);
                res
            }
            Self::STIndI64(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndI64);
                res
            }
            Self::STIndISize(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndISize);
                res
            }
            Self::STIndF64(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndF64);
                res
            }
            Self::STIndF32(addr, val) => {
                let mut res = addr.flatten();
                res.extend(val.flatten());
                res.push(CILOp::STIndF32);
                res
            }
            Self::Break => vec![CILOp::Break],
            Self::Nop => vec![CILOp::Nop],
            Self::JumpingPad { ops } => ops.clone().into(),
            Self::STObj {
                tpe,
                addr_calc,
                value_calc,
            } => {
                let mut res = addr_calc.flatten();
                res.extend(value_calc.flatten());
                res.push(CILOp::STObj(tpe.clone()));
                res
            }
            Self::SetTMPLocal { value } => {
                todo!("Can't flatten unresolved root!")
            }
            Self::SetStaticField { descr, value } => {
                append_vec(value.flatten(), CILOp::STStaticField(descr.clone().into()))
            }
        }
    }
    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        match self {
            CILRoot::BTrue {
                target, sub_target, ..
            }
            | CILRoot::GoTo { target, sub_target } => {
                targets.push((*target, *sub_target));
            }
            _ => (),
        }
    }
    pub(crate) fn fix_for_exception_handler(&mut self, id: u32) {
        match self {
            CILRoot::BTrue {
                target, sub_target, ..
            }
            | CILRoot::GoTo { target, sub_target } => {
                assert_eq!(
                    *sub_target, 0,
                    "An exception handler can't contain inner exception handler!"
                );
                *sub_target = *target;
                *target = id;
            }
            _ => (),
        }
    }
    pub fn shed_trees(mut self) -> Vec<Self> {
        let mut res = vec![];
        let trees: Vec<CILRoot> = match &mut self {
            CILRoot::STLoc { local: _, tree } => tree.sheed_trees(),
            CILRoot::BTrue {
                target: _,
                sub_target: _,
                ops,
            } => ops.sheed_trees(),
            CILRoot::GoTo {
                target: _,
                sub_target: _,
            } => vec![],
            CILRoot::CallVirt { site: _, args } | CILRoot::Call { site: _, args } => {
                args.iter_mut().flat_map(|arg| arg.sheed_trees()).collect()
            }
            CILRoot::SetField { addr, value, desc } => {
                let mut res = addr.sheed_trees();
                res.extend(value.sheed_trees());
                // Check that trees were propely sheed.
                assert!(!matches!(value,CILNode::SubTrees(_, _)));
                res
            }
            CILRoot::SetTMPLocal { value } => panic!("Unresolved TMP local!"),
            CILRoot::CpBlk { src, dst, len } => {
                let mut res = src.sheed_trees();
                res.extend(dst.sheed_trees());
                res.extend(len.sheed_trees());
                res
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
                let mut res = addr_calc.sheed_trees();
                res.extend(value_calc.sheed_trees());
                res
            }
            CILRoot::STArg { arg, tree } => tree.sheed_trees(),
            CILRoot::Break => vec![],
            CILRoot::Nop => vec![],
            CILRoot::InitBlk { dst, val, count } =>{
                let mut res = dst.sheed_trees();
                res.extend(val.sheed_trees());
                res.extend(count.sheed_trees());
                res
            }
            CILRoot::Ret { tree } | CILRoot::Pop { tree } => tree.sheed_trees(),
            CILRoot::VoidRet => vec![],
            CILRoot::Throw(tree) => tree.sheed_trees(),
            CILRoot::ReThrow => vec![],
            CILRoot::CallI { sig, fn_ptr, args } => todo!(),
            CILRoot::JumpingPad { ops } => vec![],
            CILRoot::SetStaticField { descr, value } => value.sheed_trees(),
        };
        res.extend(trees);
        res.push(self);
        res
    }
    pub(crate) fn allocate_tmps(
        &mut self,
        curr_local: Option<u32>,
        locals: &mut Vec<(Option<Box<str>>, Type)>,
    ) {
        match self {
            CILRoot::STLoc { local, tree } => tree.allocate_tmps(curr_local, locals),
            CILRoot::BTrue {
                target,
                sub_target,
                ops,
            } => ops.allocate_tmps(curr_local, locals),
            CILRoot::GoTo { target, sub_target } => (),
            CILRoot::CallVirt { site, args } |
            CILRoot::Call { site, args } => args
                .iter_mut()
                .for_each(|arg| arg.allocate_tmps(curr_local, locals)),
            CILRoot::SetField { addr, value, desc } => {
                addr.allocate_tmps(curr_local, locals);
                value.allocate_tmps(curr_local, locals);
            }
            CILRoot::CpBlk { src, dst, len } => {
                src.allocate_tmps(curr_local, locals);
                dst.allocate_tmps(curr_local, locals);
                len.allocate_tmps(curr_local, locals);
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
            } => addr_calc.allocate_tmps(curr_local, locals),
            CILRoot::STArg { arg, tree } => tree.allocate_tmps(curr_local, locals),
            CILRoot::Break => (),
            CILRoot::Nop => (),
            CILRoot::InitBlk { dst, val, count } => {
                dst.allocate_tmps(curr_local, locals);
                val.allocate_tmps(curr_local, locals);
                count.allocate_tmps(curr_local, locals);
            }
           
            CILRoot::Ret { tree } | CILRoot::Pop { tree } | CILRoot::Throw(tree) => {
                tree.allocate_tmps(curr_local, locals)
            }
            CILRoot::VoidRet => (),

            CILRoot::ReThrow => (),
            CILRoot::CallI { sig, fn_ptr, args } => {
                fn_ptr.allocate_tmps(curr_local, locals);
                args
                .iter_mut()
                .for_each(|arg| arg.allocate_tmps(curr_local, locals));
            }
            CILRoot::JumpingPad { ops } => (),
            CILRoot::SetTMPLocal { value } => {
                *self = Self::STLoc {
                    local: curr_local.expect("Referenced a tmp local when none present!"),
                    tree: value.clone().into(),
                };
            }
            CILRoot::SetStaticField { descr, value } => value.allocate_tmps(curr_local, locals),
        }
    }

    pub(crate) fn resolve_global_allocations(
        &mut self,
        asm: &mut crate::assembly::Assembly,
        tyctx: TyCtxt,
    ) {
        match self {
            CILRoot::STLoc { local, tree } => tree.resolve_global_allocations(asm, tyctx),
            CILRoot::BTrue {
                target,
                sub_target,
                ops,
            } => ops.resolve_global_allocations(asm, tyctx),
            CILRoot::GoTo { target, sub_target } => (),
            CILRoot::CallVirt { site, args } |
            CILRoot::Call { site, args } => args
                .iter_mut()
                .for_each(|arg| arg.resolve_global_allocations(asm, tyctx)),
            CILRoot::SetField { addr, value, desc } => {
                addr.resolve_global_allocations(asm, tyctx);
                value.resolve_global_allocations(asm, tyctx);
            }
            CILRoot::CpBlk { src, dst, len } =>  {
                src.resolve_global_allocations(asm, tyctx);
                dst.resolve_global_allocations(asm, tyctx);
                len.resolve_global_allocations(asm, tyctx);
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
            } => addr_calc.resolve_global_allocations(asm, tyctx),
            CILRoot::STArg { arg, tree } => tree.resolve_global_allocations(asm, tyctx),
            CILRoot::Break => (),
            CILRoot::Nop => (),
            CILRoot::InitBlk { dst, val, count } => {
                dst.resolve_global_allocations(asm, tyctx);
                val.resolve_global_allocations(asm, tyctx);
                count.resolve_global_allocations(asm, tyctx);
            }
           
            
            CILRoot::Ret { tree } | CILRoot::Pop { tree } | CILRoot::Throw(tree) => {
                tree.resolve_global_allocations(asm, tyctx)
            }
            CILRoot::VoidRet => (),

            CILRoot::ReThrow => (),
            CILRoot::CallI { sig, fn_ptr, args } =>  {
                fn_ptr.resolve_global_allocations(asm, tyctx);
                args
                .iter_mut()
                .for_each(|arg| arg.resolve_global_allocations(asm, tyctx));
            }
            // Jump pads CAN'T ever allocate.
            CILRoot::JumpingPad { ops } => (),
            CILRoot::SetTMPLocal { value } => value.resolve_global_allocations(asm, tyctx),
            CILRoot::SetStaticField { descr, value } => {
                value.resolve_global_allocations(asm, tyctx)
            }
        }
    }
}
