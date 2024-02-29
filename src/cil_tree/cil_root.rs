use crate::{
    cil::{CILOp, CallSite, FieldDescriptor},
    cil_tree::cil_node::CILNode,
    function_sig::FnSig,
    r#type::Type,
};

use super::append_vec;
#[derive(Clone, Debug)]
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
    VoidRet,
    Throw(CILNode),
    ReThrow,
    CallI {
        sig: FnSig,
        fn_ptr: CILNode,
        args: Box<[CILNode]>,
    },
    Raw {
        ops: Box<[CILOp]>,
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
            CILRoot::SetField { addr, value, desc } => {
                addr.opt();
                value.opt();
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
            CILRoot::VoidRet => (),
            CILRoot::Throw(ops) => ops.opt(),
            CILRoot::ReThrow => (),
            CILRoot::CallI { sig, fn_ptr, args } => {
                args.iter_mut().for_each(|arg| arg.opt());
                fn_ptr.opt();
            }
            CILRoot::Raw { ops } => (),
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

    pub fn flatten(&self) -> Vec<CILOp> {
        match self {
            //Self::LabelStart(val)=> vec![CILOp::LabelStart(val)],
            //Self::LabelEnd(val)=> vec![CILOp::LabelEnd(val)],
            Self::ReThrow => vec![CILOp::ReThrow],
            Self::Throw(tree) => append_vec(tree.flatten(), CILOp::Throw),
            Self::Ret { tree } => append_vec(tree.flatten(), CILOp::Ret),
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
            Self::Raw { ops } => ops.clone().into(),
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
}
