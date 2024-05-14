use super::append_vec;
use crate::{
    cil::{CILOp, CallSite, FieldDescriptor},
    cil_tree::cil_node::CILNode,
    function_sig::FnSig,
    r#type::{DotnetTypeRef, TyCache, Type},
    IString,
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
        dst: CILNode,
        src: CILNode,
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
    SourceFileInfo(Box<(u32, u32, IString)>),
    //LabelStart(u32),
    //LabelEnd(u32),
}
impl CILRoot {
    pub fn opt(&mut self) {
        match self {
            CILRoot::SourceFileInfo(_) => (),
            CILRoot::STLoc { tree, local: _ } => tree.opt(),
            CILRoot::BTrue {
                ops,
                sub_target: _,
                target: _,
            } => ops.opt(),
            CILRoot::GoTo { .. } => (),
            CILRoot::Call { args, site: _ } => {
                args.iter_mut().for_each(super::cil_node::CILNode::opt);
            }
            CILRoot::SetField {
                addr: fld_addr,
                value,
                desc: _,
            } => {
                fld_addr.opt();
                value.opt();

                match fld_addr {
                    CILNode::ConvUSize(addr) => match addr.as_mut() {
                        CILNode::LDLocA(_) | CILNode::LDFieldAdress { .. } => {
                            *fld_addr = addr.as_ref().clone();
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
                addr_calc,
                value_calc,
                tpe: _,
            } => {
                addr_calc.opt();
                value_calc.opt();
            }
            CILRoot::STArg { tree, arg: _ } => {
                tree.opt();
            }
            CILRoot::Break => (),
            CILRoot::Nop => (),
            CILRoot::InitBlk { dst, val, count } => {
                val.opt();
                dst.opt();
                count.opt();
            }
            CILRoot::CallVirt { site: _, args } => {
                args.iter_mut().for_each(super::cil_node::CILNode::opt);
            }
            CILRoot::Ret { tree } => tree.opt(),
            CILRoot::Pop { tree } => tree.opt(),
            CILRoot::VoidRet => (),
            CILRoot::Throw(ops) => ops.opt(),
            CILRoot::ReThrow => (),
            CILRoot::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                args.iter_mut().for_each(super::cil_node::CILNode::opt);
                fn_ptr.opt();
            }
            CILRoot::JumpingPad { ops: _ } => (),
            CILRoot::SetTMPLocal { value } => value.opt(),
            CILRoot::SetStaticField { descr: _, value } => value.opt(),
        }
    }
    #[must_use]
    pub fn throw(msg: &str) -> Self {
        let mut class =
            crate::r#type::DotnetTypeRef::new(Some("System.Runtime"), "System.Exception");
        class.set_valuetype(false);
        let name = ".ctor".into();
        let signature = crate::function_sig::FnSig::new(
            &[class.clone().into(), DotnetTypeRef::string_type().into()],
            &crate::r#type::Type::Void,
        );
        Self::Throw(CILNode::NewObj {
            site: CallSite::boxed(Some(class), name, signature, false),
            args: [CILNode::LdStr(msg.into())].into(),
        })
    }
    #[must_use]
    pub fn debug(msg: &str) -> Self {
        let mut class = crate::r#type::DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "WriteLine".into();
        let signature = crate::function_sig::FnSig::new(
            &[DotnetTypeRef::string_type().into()],
            &crate::r#type::Type::Void,
        );
        let message_or_check = if *crate::config::MEM_CHECKS{
            CILNode::SubTrees([CILRoot::Call { site: CallSite::mcheck_check_all(), args: [].into() }].into(),Box::new(CILNode::LdStr(msg.into())))
        }else{
            CILNode::LdStr(msg.into())
        };
        Self::Call {
            site: CallSite::new_extern(class, name, signature, true),
            args: [message_or_check].into(),
        }
    }
    #[must_use]
    pub fn into_ops(&self) -> Vec<CILOp> {
        match std::panic::catch_unwind(|| {
            match self {
                Self::SourceFileInfo(sfi) => vec![CILOp::SourceFileInfo(sfi.clone())],
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
                    let mut args: Vec<_> = args
                        .iter()
                        .flat_map(super::cil_node::CILNode::flatten)
                        .collect();
                    args.push(CILOp::Call(site.clone().into()));
                    args
                }
                Self::CallI { sig, fn_ptr, args } => {
                    let mut ops: Vec<_> = args
                        .iter()
                        .flat_map(super::cil_node::CILNode::flatten)
                        .collect();
                    ops.extend(fn_ptr.flatten());
                    ops.push(CILOp::CallI(sig.clone().into()));
                    ops
                }
                Self::CallVirt { site, args } => {
                    let mut args: Vec<_> = args
                        .iter()
                        .flat_map(super::cil_node::CILNode::flatten)
                        .collect();
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
                    // Argument order: destination, source, length
                    let mut res = dst.flatten();
                    res.extend(src.flatten());
                    res.extend(len.flatten());
                    res.push(CILOp::CpBlk);
                    res
                }
                Self::InitBlk { dst, val, count } => {
                    let mut res = dst.flatten();
                    res.extend(val.flatten());
                    res.extend(count.flatten());
                    res.push(CILOp::InitBlk);
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
                Self::SetTMPLocal { .. } => {
                    todo!("Can't flatten unresolved root!")
                }
                Self::SetStaticField { descr, value } => {
                    append_vec(value.flatten(), CILOp::STStaticField(descr.clone().into()))
                }
            }
        }) {
            Ok(ok) => ok,
            Err(_) => panic!("Could not flatten tree {self:?}"),
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
    #[must_use]
    pub fn shed_trees(mut self) -> Vec<Self> {
        let mut res = vec![];
        let trees: Vec<CILRoot> = match &mut self {
            CILRoot::SourceFileInfo(_) => vec![],
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
            CILRoot::CallVirt { site: _, args } | CILRoot::Call { site: _, args } => args
                .iter_mut()
                .flat_map(super::cil_node::CILNode::sheed_trees)
                .collect(),
            CILRoot::SetField { addr, value, .. } => {
                let mut res = addr.sheed_trees();
                res.extend(value.sheed_trees());
                // Check that trees were propely sheed.
                assert!(!matches!(value, CILNode::SubTrees(_, _)));
                res
            }
            CILRoot::SetTMPLocal { .. } => panic!("Unresolved TMP local!"),
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
            CILRoot::STArg { arg: _, tree } => tree.sheed_trees(),
            CILRoot::Break => vec![],
            CILRoot::Nop => vec![],
            CILRoot::InitBlk { dst, val, count } => {
                let mut res = dst.sheed_trees();
                res.extend(val.sheed_trees());
                res.extend(count.sheed_trees());
                res
            }
            CILRoot::Ret { tree } | CILRoot::Pop { tree } => tree.sheed_trees(),
            CILRoot::VoidRet => vec![],
            CILRoot::Throw(tree) => tree.sheed_trees(),
            CILRoot::ReThrow => vec![],
            CILRoot::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                let mut res = fn_ptr.sheed_trees();
                res.extend(
                    args.iter_mut()
                        .flat_map(super::cil_node::CILNode::sheed_trees),
                );
                res
            }
            CILRoot::JumpingPad { ops: _ } => vec![],
            CILRoot::SetStaticField { descr: _, value } => value.sheed_trees(),
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
            CILRoot::SourceFileInfo(_) => (),
            CILRoot::STLoc { tree, .. } => tree.allocate_tmps(curr_local, locals),
            CILRoot::BTrue { ops, .. } => ops.allocate_tmps(curr_local, locals),
            CILRoot::GoTo { .. } => (),
            CILRoot::CallVirt { site: _, args } | CILRoot::Call { site: _, args } => args
                .iter_mut()
                .for_each(|arg| arg.allocate_tmps(curr_local, locals)),
            CILRoot::SetField {
                addr,
                value,
                desc: _,
            } => {
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
            } => {
                addr_calc.allocate_tmps(curr_local, locals);
                value_calc.allocate_tmps(curr_local, locals);
            }
            CILRoot::STArg { arg: _, tree } => tree.allocate_tmps(curr_local, locals),
            CILRoot::Break => (),
            CILRoot::Nop => (),
            CILRoot::InitBlk { dst, val, count } => {
                dst.allocate_tmps(curr_local, locals);
                val.allocate_tmps(curr_local, locals);
                count.allocate_tmps(curr_local, locals);
            }

            CILRoot::Ret { tree } | CILRoot::Pop { tree } | CILRoot::Throw(tree) => {
                tree.allocate_tmps(curr_local, locals);
            }
            CILRoot::VoidRet => (),

            CILRoot::ReThrow => (),
            CILRoot::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                fn_ptr.allocate_tmps(curr_local, locals);
                args.iter_mut()
                    .for_each(|arg| arg.allocate_tmps(curr_local, locals));
            }
            CILRoot::JumpingPad { ops: _ } => (),
            CILRoot::SetTMPLocal { value } => {
                *self = Self::STLoc {
                    local: curr_local.expect("Referenced a tmp local when none present!"),
                    tree: value.clone(),
                };
            }
            CILRoot::SetStaticField { descr: _, value } => value.allocate_tmps(curr_local, locals),
        }
    }

    pub(crate) fn resolve_global_allocations(
        &mut self,
        asm: &mut crate::assembly::Assembly,
        tyctx: TyCtxt,
        tycache: &mut TyCache,
    ) {
        match self {
            CILRoot::SourceFileInfo(_) => (),
            CILRoot::STLoc { local: _, tree } => {
                tree.resolve_global_allocations(asm, tyctx, tycache);
            }
            CILRoot::BTrue {
                target: _,
                sub_target: _,
                ops,
            } => ops.resolve_global_allocations(asm, tyctx, tycache),
            CILRoot::GoTo {
                target: _,
                sub_target: _,
            } => (),
            CILRoot::CallVirt { site: _, args } | CILRoot::Call { site: _, args } => args
                .iter_mut()
                .for_each(|arg| arg.resolve_global_allocations(asm, tyctx, tycache)),
            CILRoot::SetField {
                addr,
                value,
                desc: _,
            } => {
                addr.resolve_global_allocations(asm, tyctx, tycache);
                value.resolve_global_allocations(asm, tyctx, tycache);
            }
            CILRoot::CpBlk { src, dst, len } => {
                src.resolve_global_allocations(asm, tyctx, tycache);
                dst.resolve_global_allocations(asm, tyctx, tycache);
                len.resolve_global_allocations(asm, tyctx, tycache);
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
                addr_calc.resolve_global_allocations(asm, tyctx, tycache);
                value_calc.resolve_global_allocations(asm, tyctx, tycache);
            }
            CILRoot::STArg { arg: _, tree } => tree.resolve_global_allocations(asm, tyctx, tycache),
            CILRoot::Break => (),
            CILRoot::Nop => (),
            CILRoot::InitBlk { dst, val, count } => {
                dst.resolve_global_allocations(asm, tyctx, tycache);
                val.resolve_global_allocations(asm, tyctx, tycache);
                count.resolve_global_allocations(asm, tyctx, tycache);
            }

            CILRoot::Ret { tree } | CILRoot::Pop { tree } | CILRoot::Throw(tree) => {
                tree.resolve_global_allocations(asm, tyctx, tycache);
            }
            CILRoot::VoidRet => (),

            CILRoot::ReThrow => (),
            CILRoot::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                fn_ptr.resolve_global_allocations(asm, tyctx, tycache);
                args.iter_mut()
                    .for_each(|arg| arg.resolve_global_allocations(asm, tyctx, tycache));
            }
            // Jump pads CAN'T ever allocate.
            CILRoot::JumpingPad { ops: _ } => (),
            CILRoot::SetTMPLocal { value } => value.resolve_global_allocations(asm, tyctx, tycache),
            CILRoot::SetStaticField { descr: _, value } => {
                value.resolve_global_allocations(asm, tyctx, tycache);
            }
        }
    }

    pub(crate) fn source_info(file: &str, line: u32, column: u32) -> Self {
        Self::SourceFileInfo(Box::new((line, column, file.into())))
    }
    pub(crate) fn span_source_info(tyctx: TyCtxt, span: rustc_span::Span) -> Self {
        let file = tyctx.sess.source_map().span_to_embeddable_string(span);
        let (line, column) = tyctx
            .sess
            .source_map()
            .span_to_lines(span)
            .map(|lines| match lines.lines.first() {
                Some(pos) => (pos.line_index, pos.start_col.0),
                None => (0, 0),
            })
            .unwrap_or((0, 0));
        Self::source_info(&file, line as u32, column as u32)
    }
}
#[test]
fn allocating_tmps() {
    let mut original_value = CILRoot::STLoc {
        local: 11,
        tree: CILNode::SubTrees(
            Box::new([CILRoot::STLoc {
                local: 14,
                tree: CILNode::TemporaryLocal(Box::new((
                    Type::DotnetType(
                        DotnetTypeRef::new(
                            None,
                            "core.ptr.metadata.PtrComponents.h2b679e9941d88b2f",
                        )
                        .into(),
                    ),
                    [CILRoot::SetTMPLocal {
                        value: CILNode::LDArg(0),
                    }]
                    .into(),
                    CILNode::LdObj {
                        ptr: CILNode::LoadAddresOfTMPLocal.into(),
                        obj: Type::DotnetType(
                            DotnetTypeRef::new(
                                None,
                                "core.ptr.metadata.PtrComponents.h2b679e9941d88b2f",
                            )
                            .into(),
                        )
                        .into(),
                    },
                ))),
            }]),
            CILNode::LdObj {
                ptr: CILNode::LDLocA(14).into(),
                obj: Type::DotnetType(
                    DotnetTypeRef::new(None, "core.ptr.metadata.PtrComponents.h2b679e9941d88b2f")
                        .into(),
                )
                .into(),
            }
            .into(),
        ),
    };
    //let mut method = crate::method::Method::new(crate::access_modifier::AccessModifer::Private,crate::method::MethodType::Static,FnSig::new(&[Type::I32],&Type::Void),"a",vec![],vec![]);
    original_value.allocate_tmps(None, &mut vec![]);
    let trees = original_value.shed_trees();
    let _ops: Vec<_> = trees.iter().map(CILRoot::into_ops).collect();
}
