use crate::{
    call_site::CallSite, cil_iter_mut::CILIterMut, cil_node::CILNode, field_desc::FieldDescriptor,
    fn_sig::FnSig, static_field_desc::StaticFieldDescriptor, DotnetTypeRef, IString, Type,
};

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
        cond: CILNode,
    },
    BFalse {
        target: u32,
        sub_target: u32,
        cond: CILNode,
    },
    BEq {
        target: u32,
        sub_target: u32,
        a: CILNode,
        b: CILNode,
    },
    BNe {
        target: u32,
        sub_target: u32,
        a: CILNode,
        b: CILNode,
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
        source: u32,
        target: u32,
    },
    SetStaticField {
        descr: StaticFieldDescriptor,
        value: CILNode,
    },
    SourceFileInfo(Box<(std::ops::Range<u64>, std::ops::Range<u64>, IString)>),
    //LabelStart(u32),
    //LabelEnd(u32),
}
impl CILRoot {
    pub fn opt(&mut self, opt_count: &mut usize) {
        match self {
            Self::SourceFileInfo(_) => (),
            Self::STLoc { tree, local: _ } => tree.opt(opt_count),
            Self::BTrue {
                cond: ops,
                sub_target,
                target,
            } => {
                ops.opt(opt_count);
                match ops {
                    CILNode::Eq(a, b) => {
                        *self = CILRoot::BEq {
                            target: *target,
                            sub_target: *sub_target,
                            a: *a.clone(),
                            b: *b.clone(),
                        };
                        *opt_count += 1;
                    }

                    _ => (),
                }
            }
            Self::BFalse {
                cond: ops,
                sub_target,
                target,
            } => {
                ops.opt(opt_count);
            }
            Self::BEq {
                a,
                b,
                sub_target,
                target,
            } => {
                a.opt(opt_count);
                b.opt(opt_count);
                match (a, b) {
                    (CILNode::LdcU32(0) | CILNode::LdFalse, cond)
                    | (cond, CILNode::LdcU32(0) | CILNode::LdFalse) => {
                        *self = CILRoot::BFalse {
                            target: *target,
                            sub_target: *sub_target,
                            cond: cond.clone(),
                        };
                        *opt_count += 1;
                    }
                    _ => (),
                }
            }
            Self::BNe {
                a,
                b,
                sub_target: _,
                target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count);
            }
            Self::GoTo { .. } => (),
            Self::Call { args, site: _ } => {
                args.iter_mut().for_each(|arg| arg.opt(opt_count));
            }
            Self::SetField {
                addr: fld_addr,
                value,
                desc: _,
            } => {
                fld_addr.opt(opt_count);
                value.opt(opt_count);

                if let CILNode::ZeroExtendToUSize(addr) = fld_addr {
                    match addr.as_mut() {
                        CILNode::LDLocA(_) | CILNode::LDFieldAdress { .. } => {
                            *fld_addr = addr.as_ref().clone();
                        }
                        _ => (),
                    }
                }
            }
            Self::CpBlk { src, dst, len } => {
                src.opt(opt_count);
                dst.opt(opt_count);
                len.opt(opt_count);
            }
            Self::STIndI8(addr, val)
            | Self::STIndI16(addr, val)
            | Self::STIndI32(addr, val)
            | Self::STIndI64(addr, val)
            | Self::STIndISize(addr, val)
            | Self::STIndF64(addr, val)
            | Self::STIndF32(addr, val) => {
                addr.opt(opt_count);
                val.opt(opt_count);
            }
            Self::STObj {
                addr_calc,
                value_calc,
                tpe: _,
            } => {
                addr_calc.opt(opt_count);
                value_calc.opt(opt_count);
            }
            Self::STArg { tree, arg: _ } => {
                tree.opt(opt_count);
            }
            Self::Break => (),
            Self::Nop => (),
            Self::InitBlk { dst, val, count } => {
                val.opt(opt_count);
                dst.opt(opt_count);
                count.opt(opt_count);
            }
            Self::CallVirt { site: _, args } => {
                args.iter_mut().for_each(|arg| arg.opt(opt_count));
            }
            Self::Ret { tree } => tree.opt(opt_count),
            Self::Pop { tree } => tree.opt(opt_count),
            Self::VoidRet => (),
            Self::Throw(ops) => ops.opt(opt_count),
            Self::ReThrow => (),
            Self::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                args.iter_mut().for_each(|arg| arg.opt(opt_count));
                fn_ptr.opt(opt_count);
            }

            Self::SetTMPLocal { value } => value.opt(opt_count),
            Self::SetStaticField { descr: _, value } => value.opt(opt_count),
            Self::JumpingPad { .. } => (),
        }
    }
    #[must_use]
    pub fn throw(msg: &str) -> Self {
        let mut class = DotnetTypeRef::new(Some("System.Runtime"), "System.Exception");
        class.set_valuetype(false);
        let name = ".ctor".into();
        let signature = FnSig::new(
            &[class.clone().into(), DotnetTypeRef::string_type().into()],
            Type::Void,
        );
        Self::Throw(CILNode::NewObj {
            site: CallSite::boxed(Some(class), name, signature, false),
            args: [CILNode::LdStr(msg.into())].into(),
        })
    }
    #[must_use]
    pub fn debug(msg: &str) -> Self {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "WriteLine".into();
        let signature = FnSig::new(&[DotnetTypeRef::string_type().into()], Type::Void);
        let message_or_check = if crate::mem_checks() {
            CILNode::SubTrees(
                [Self::Call {
                    site: CallSite::mcheck_check_all(),
                    args: [].into(),
                }]
                .into(),
                Box::new(CILNode::LdStr(msg.into())),
            )
        } else {
            CILNode::LdStr(msg.into())
        };
        Self::Call {
            site: CallSite::new_extern(class, name, signature, true),
            args: [message_or_check].into(),
        }
    }
    pub fn targets(&self, targets: &mut Vec<(u32, u32)>) {
        match self {
            Self::BTrue {
                target, sub_target, ..
            }
            | Self::BFalse {
                target, sub_target, ..
            }
            | Self::BEq {
                target, sub_target, ..
            }
            | Self::BNe {
                target, sub_target, ..
            }
            | Self::GoTo { target, sub_target } => {
                targets.push((*target, *sub_target));
            }
            _ => (),
        }
    }
    pub fn fix_for_exception_handler(&mut self, id: u32) {
        match self {
            Self::BTrue {
                target, sub_target, ..
            }
            | Self::BFalse {
                target, sub_target, ..
            }
            | Self::BEq {
                target, sub_target, ..
            }
            | Self::BNe {
                target, sub_target, ..
            }
            | Self::GoTo { target, sub_target } => {
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
    pub fn sheed_trees(mut self) -> Vec<Self> {
        let iter_mut = (&mut self).into_iter();
        let mut res: Vec<CILRoot> = iter_mut
            .map(|tree| match tree {
                crate::cil_iter_mut::CILIterElemMut::Node(node) => match node {
                    CILNode::SubTrees(trees, main) => {
                        let vec = trees.to_vec();
                        let iter = vec.into_iter();
                        let trees = iter.flat_map(|tree| tree.sheed_trees()).collect();
                        *node = *main.clone();
                        trees
                    }
                    _ => vec![],
                },
                _ => vec![],
            })
            .flat_map(|vec| vec)
            .collect();
        res.push(self);
        res
    }
    pub fn allocate_tmps(
        &mut self,
        curr_loc: Option<u32>,
        locals: &mut Vec<(Option<Box<str>>, Type)>,
    ) {
        match self {
            Self::SourceFileInfo(_) => (),
            Self::STLoc { tree, .. } => {
                tree.allocate_tmps(curr_loc, locals);
            }
            Self::BTrue { cond: ops, .. } => ops.allocate_tmps(curr_loc, locals),
            Self::BFalse { cond: ops, .. } => ops.allocate_tmps(curr_loc, locals),
            Self::BEq { a, b, .. } | Self::BNe { a, b, .. } => {
                a.allocate_tmps(curr_loc, locals);
                b.allocate_tmps(curr_loc, locals);
            }
            Self::GoTo { .. } => (),
            Self::CallVirt { site: _, args } | Self::Call { site: _, args } => args
                .iter_mut()
                .for_each(|arg| arg.allocate_tmps(curr_loc, locals)),
            Self::SetField {
                addr,
                value,
                desc: _,
            } => {
                addr.allocate_tmps(curr_loc, locals);
                value.allocate_tmps(curr_loc, locals);
            }
            Self::CpBlk { src, dst, len } => {
                src.allocate_tmps(curr_loc, locals);
                dst.allocate_tmps(curr_loc, locals);
                len.allocate_tmps(curr_loc, locals);
            }
            Self::STIndI8(addr_calc, value_calc)
            | Self::STIndI16(addr_calc, value_calc)
            | Self::STIndI32(addr_calc, value_calc)
            | Self::STIndI64(addr_calc, value_calc)
            | Self::STIndISize(addr_calc, value_calc)
            | Self::STIndF64(addr_calc, value_calc)
            | Self::STIndF32(addr_calc, value_calc)
            | Self::STObj {
                addr_calc,
                value_calc,
                ..
            } => {
                addr_calc.allocate_tmps(curr_loc, locals);
                value_calc.allocate_tmps(curr_loc, locals);
            }
            Self::STArg { arg: _, tree } => tree.allocate_tmps(curr_loc, locals),
            Self::Break => (),
            Self::Nop => (),
            Self::InitBlk { dst, val, count } => {
                dst.allocate_tmps(curr_loc, locals);
                val.allocate_tmps(curr_loc, locals);
                count.allocate_tmps(curr_loc, locals);
            }

            Self::Ret { tree } | Self::Pop { tree } | Self::Throw(tree) => {
                tree.allocate_tmps(curr_loc, locals);
            }
            Self::VoidRet => (),

            Self::ReThrow => (),
            Self::CallI {
                sig: _,
                fn_ptr,
                args,
            } => {
                fn_ptr.allocate_tmps(curr_loc, locals);
                args.iter_mut()
                    .for_each(|arg| arg.allocate_tmps(curr_loc, locals));
            }

            Self::SetTMPLocal { value } => {
                value.allocate_tmps(curr_loc, locals);
                *self = Self::STLoc {
                    local: curr_loc.expect("Referenced a tmp local when none present!"),
                    tree: value.clone(),
                };
            }
            Self::SetStaticField { descr: _, value } => value.allocate_tmps(curr_loc, locals),
            Self::JumpingPad { .. } => (),
        };
    }

    #[must_use]
    pub fn source_info(
        file: &str,
        line: std::ops::Range<u64>,
        column: std::ops::Range<u64>,
    ) -> Self {
        assert!(
            column.start < column.end,
            "PDB files must have columns that contain at least one element "
        );
        Self::SourceFileInfo(Box::new((line, column, file.into())))
    }
}
#[test]
fn allocating_tmps() {
    let mut original_value = CILNode::SubTrees(
        Box::new([CILRoot::STLoc {
            local: 14,
            tree: CILNode::TemporaryLocal(Box::new((
                Type::DotnetType(
                    DotnetTypeRef::new::<&str, _>(
                        None,
                        "core.ptr.metadata.PtrComponents.h2b679e9941d88b2f",
                    )
                    .into(),
                ),
                [CILRoot::SetTMPLocal {
                    value: CILNode::LDArg(0),
                }]
                .into(),
                CILNode::LDLoc(3),
            ))),
        }]),
        CILNode::LDLoc(2).into(),
    );
    //let mut method = crate::method::Method::new(crate::access_modifier::AccessModifer::Private,crate::method::MethodType::Static,FnSig::new(&[Type::I32],&Type::Void),"a",vec![],vec![]);
    original_value.allocate_tmps(None, &mut vec![]);
    println!("original_value:{original_value:?}");
    //let _trees = original_value.sheed_trees();
    //let _ops: Vec<_> = trees.iter().map(CILRoot::into_ops).collect();
}
