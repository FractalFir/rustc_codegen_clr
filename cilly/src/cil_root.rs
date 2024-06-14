use crate::{
    call_site::CallSite,
    cil_node::{CILNode, CallOpArgs, ValidationContext},
    field_desc::FieldDescriptor,
    fn_sig::FnSig,
    static_field_desc::StaticFieldDescriptor,
    DotnetTypeRef, IString, Type,
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
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BLt {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BLtUn {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BGt {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BGtUn {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BLe {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BGe {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    BNe {
        target: u32,
        sub_target: u32,
        a: Box<CILNode>,
        b: Box<CILNode>,
    },
    GoTo {
        target: u32,
        sub_target: u32,
    },

    Call {
        site: Box<CallSite>,
        args: Box<[CILNode]>,
    },
    SetField {
        addr: Box<CILNode>,
        value: Box<CILNode>,
        desc: Box<FieldDescriptor>,
    },
    SetTMPLocal {
        value: CILNode,
    },
    CpBlk {
        dst: Box<CILNode>,
        src: Box<CILNode>,
        len: Box<CILNode>,
    },
    STIndI8(CILNode, CILNode),
    STIndI16(CILNode, CILNode),
    STIndI32(CILNode, CILNode),
    STIndI64(CILNode, CILNode),
    STIndISize(CILNode, CILNode),
    STIndPtr(CILNode, CILNode, Box<Type>),
    STIndF64(CILNode, CILNode),
    STIndF32(CILNode, CILNode),
    STObj {
        tpe: Box<Type>,
        addr_calc: Box<CILNode>,
        value_calc: Box<CILNode>,
    },
    STArg {
        arg: u32,
        tree: CILNode,
    },
    Break,
    Nop,
    InitBlk {
        dst: Box<CILNode>,
        val: Box<CILNode>,
        count: Box<CILNode>,
    },
    CallVirt {
        site: Box<CallSite>,
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
        sig: Box<FnSig>,
        fn_ptr: Box<CILNode>,
        args: Box<[CILNode]>,
    },
    JumpingPad {
        source: u32,
        target: u32,
    },
    SetStaticField {
        descr: Box<StaticFieldDescriptor>,
        value: CILNode,
    },
    SourceFileInfo(SFI),
    //LabelStart(u32),
    //LabelEnd(u32),
}
pub type SFI = Box<(std::ops::Range<u64>, std::ops::Range<u64>, IString)>;
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
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    CILNode::Lt(a, b) => {
                        *self = CILRoot::BLt {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    CILNode::LtUn(a, b) => {
                        *self = CILRoot::BLtUn {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    CILNode::Gt(a, b) => {
                        *self = CILRoot::BGt {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    CILNode::GtUn(a, b) => {
                        *self = CILRoot::BGtUn {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
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
                match ops {
                    CILNode::Eq(a, b) => {
                        *self = CILRoot::BNe {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    CILNode::Lt(a, b) => {
                        *self = CILRoot::BGe {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    CILNode::Gt(a, b) => {
                        *self = CILRoot::BLe {
                            target: *target,
                            sub_target: *sub_target,
                            a: Box::new(*a.clone()),
                            b: Box::new(*b.clone()),
                        };
                        *opt_count += 1;
                    }
                    _ => (),
                }
            }
            Self::BEq {
                a,
                b,
                sub_target,
                target,
            } => {
                a.opt(opt_count);
                b.opt(opt_count);
                match (a.as_mut(), b.as_mut()) {
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
            Self::BLt {
                a,
                b,
                target: _,
                sub_target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count)
            }
            Self::BLtUn {
                a,
                b,
                target: _,
                sub_target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count)
            }
            Self::BGt {
                a,
                b,
                target: _,
                sub_target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count)
            }
            Self::BGtUn {
                a,
                b,
                target: _,
                sub_target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count)
            }
            Self::BLe {
                a,
                b,
                target: _,
                sub_target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count)
            }
            Self::BGe {
                a,
                b,
                target: _,
                sub_target: _,
            } => {
                a.opt(opt_count);
                b.opt(opt_count)
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

                if let CILNode::MRefToRawPtr(addr) = fld_addr.as_mut() {
                    match addr.as_mut() {
                        CILNode::LDLocA(_) | CILNode::LDFieldAdress { .. } => {
                            *fld_addr = Box::new(addr.as_ref().clone());
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
            | Self::STIndPtr(addr, val, _)
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
        Self::Throw(CILNode::NewObj(Box::new(CallOpArgs {
            site: CallSite::boxed(Some(class), name, signature, false),
            args: [CILNode::LdStr(msg.into())].into(),
        })))
    }
    #[must_use]
    pub fn debug(msg: &str) -> Self {
        let mut class = DotnetTypeRef::new(Some("System.Console"), "System.Console");
        class.set_valuetype(false);
        let name = "WriteLine".into();
        let signature = FnSig::new(&[DotnetTypeRef::string_type().into()], Type::Void);
        let message_or_check = if crate::mem_checks() {
            CILNode::SubTrees(Box::new((
                [Self::Call {
                    site: Box::new(CallSite::mcheck_check_all()),
                    args: [].into(),
                }]
                .into(),
                Box::new(CILNode::LdStr(msg.into())),
            )))
        } else {
            CILNode::LdStr(msg.into())
        };
        Self::Call {
            site: Box::new(CallSite::new_extern(class, name, signature, true)),
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
            .flat_map(|tree| match tree {
                crate::cil_iter_mut::CILIterElemMut::Node(node) => match node {
                    CILNode::SubTrees(tm) => {
                        let (trees, main) = tm.as_mut();
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
            Self::BEq { a, b, .. }
            | Self::BNe { a, b, .. }
            | Self::BLt { a, b, .. }
            | Self::BLtUn { a, b, .. }
            | Self::BGt { a, b, .. }
            | Self::BGtUn { a, b, .. }
            | Self::BLe { a, b, .. }
            | Self::BGe { a, b, .. } => {
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
            | Self::STIndPtr(addr_calc, value_calc, _)
            | Self::STIndF64(addr_calc, value_calc)
            | Self::STIndF32(addr_calc, value_calc) => {
                addr_calc.allocate_tmps(curr_loc, locals);
                value_calc.allocate_tmps(curr_loc, locals);
            }
            Self::STObj {
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
    pub(crate) fn validate(
        &self,
        vctx: ValidationContext,
        tmp_loc: Option<&Type>,
    ) -> Result<(), String> {
        match self {
            Self::Pop { tree } => {
                tree.validate(vctx, tmp_loc)?;
                Ok(())
            }
            Self::STIndI8(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::I8 | Type::U8 | Type::Bool => (),
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
                    Type::I8 | Type::U8 | Type::Bool  => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type i8/u8 because the provided value is {val:?}")),
                }
            }
            Self::CpBlk { dst, src, len } => {
                let dst = dst.validate(vctx, tmp_loc)?;
                let src = src.validate(vctx, tmp_loc)?;
                let len = len.validate(vctx, tmp_loc)?;
                // TODO: verify the types of those!
                Ok(())
            }
            Self::InitBlk { dst, val, count } => {
                let dst = dst.validate(vctx, tmp_loc)?;
                let src = val.validate(vctx, tmp_loc)?;
                let len = count.validate(vctx, tmp_loc)?;
                // TODO: verify the types of those!
                Ok(())
            }
            Self::STIndI16(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::I16 | Type::U16 => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type i16/u16 at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i16/u16 at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::I16 | Type::U16 => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type i16/u16 because the provided value is {val:?}")),
                }
            }
            Self::STIndI64(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::I64 | Type::U64 => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type i64/u64 at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i64/u64 at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::I64 | Type::U64 => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type i64/u64 because the provided value is {val:?}")),
                }
            }
            Self::STIndF64(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::F64 => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type f64 at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type f64 at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::F64  => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type f64 because the provided value is {val:?}")),
                }
            }
            Self::STIndF32(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::F32 => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type f32 at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type f32 at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::F32  => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type f32 because the provided value is {val:?}")),
                }
            }
            Self::STIndI32(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::I32 | Type::U32 => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type i32/u32 at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type i32/u32 at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::I32 | Type::U32 => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type i32/u32 because the provided value is {val:?}")),
                }
            }
            Self::STIndISize(addr, val) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::Ptr(_) | Type::ManagedReference(_) | Type::USize | Type::ISize => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type isize/usize at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type isize/usize at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::ISize | Type::USize => Ok(()),
                    _=>Err(format!("Can't indirectly set a value of type isize/usize because the provided value is {val:?}")),
                }
            }
            Self::STIndPtr(addr, val, points_to) => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let val = val.validate(vctx, tmp_loc)?;
                match &addr {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => match inner.as_ref() {
                        Type::Ptr(_) | Type::ManagedReference(_) | Type::USize | Type::ISize => (),
                        _ => {
                            return Err(format!(
                                "Can't set a vaule of type ptr at address of type {addr:?}"
                            ))
                        }
                    },
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type ptr at address of type {addr:?}"
                        ))
                    }
                }
                match val{
                    Type::Ptr(val) | Type::ManagedReference(val) =>if val != *points_to {
                        Err(format!("Can't indirectly set a value of type ptr because the provided value is {val:?}, and it should be {points_to:?}"))
                    } else {
                        Ok(())
                    },
                    _=> Err(format!("Can't indirectly set a value of type ptr because the provided value is {val:?}, and it should be {points_to:?}"))
                }
            }
            Self::STObj {
                tpe,
                addr_calc,
                value_calc,
            } => {
                let addr_calc = addr_calc.validate(vctx, tmp_loc)?;
                let value_calc = value_calc.validate(vctx, tmp_loc)?;
                let inner = match &addr_calc {
                    Type::Ptr(inner) | Type::ManagedReference(inner) => inner,
                    _ => {
                        return Err(format!(
                            "Can't set a vaule of type ptr at address of type {addr_calc:?}"
                        ))
                    }
                };
                if **inner != **tpe && !matches!(inner.as_ref(), Type::I128 | Type::U128) {
                    Err(format!("Can't indirectly set a value of type {tpe:?} because the pointer points to {inner:?}, and it should point to {tpe:?}"))?;
                }
                if **inner != value_calc {
                    Err(format!("Can't indirectly set a value of type {tpe:?} because the provided value is {value_calc:?}, and it should be {inner:?}"))
                } else {
                    Ok(())
                }
            }
            Self::Break => Ok(()),
            Self::JumpingPad { .. } => Ok(()),
            Self::BTrue {
                target,
                sub_target,
                cond,
            } => {
                // Just check that `cond` is a boolean.
                let cond = cond.validate(vctx, tmp_loc)?;
                if cond != Type::Bool {
                    Err(format!(
                        "BTrue must have a boolean argument. cond is:{cond:?}"
                    ))
                } else {
                    Ok(())
                }
            }
            Self::GoTo {
                target: _,
                sub_target: _,
            } => Ok(()),
            Self::STLoc { local, tree } => {
                let expected_tpe = if let Some(loc) = vctx.locals().get(*local as usize) {
                    loc
                } else {
                    return Err(format!("Local out of range! Local{local:?}"));
                };
                let got = tree.validate(vctx, tmp_loc)?;
                if expected_tpe.1 != got {
                    if expected_tpe.1 == Type::DotnetChar && got == Type::U16 {
                        return Ok(());
                    }
                    Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting local {local:?}"))
                } else {
                    Ok(())
                }
            }

            Self::SetTMPLocal { value } => {
                let expected_tpe = if let Some(loc) = tmp_loc {
                    loc
                } else {
                    return Err(format!("SetTMPLocal used where no tmp local present!"));
                };
                let got = value.validate(vctx, tmp_loc)?;
                if *expected_tpe != got {
                    Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting a tmp local."))
                } else {
                    Ok(())
                }
            }
            Self::STArg { arg, tree } => {
                let expected_tpe = if let Some(arg) = vctx.sig().inputs().get(*arg as usize) {
                    arg
                } else {
                    return Err(format!("Arg out of range! Arg {arg:?}"));
                };
                let got = tree.validate(vctx, tmp_loc)?;
                if *expected_tpe != got {
                    Err(format!("Expected a value of {expected_tpe:?}, but got {got:?} when seting arg {arg:?}"))
                } else {
                    Ok(())
                }
            }
            Self::Call { site, args } => {
                if site.inputs().len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        site.explicit_inputs().len(),
                        args.len()
                    ));
                }
                for (arg_node, arg_tpe) in args.iter().zip(site.inputs().iter()) {
                    let got = arg_node.validate(vctx, tmp_loc)?;
                    if got != *arg_tpe {
                        if (matches!(arg_tpe, Type::ManagedReference(_))
                            && matches!(got, Type::Ptr(_)))
                            || (matches!(arg_tpe, Type::Ptr(_))
                                && matches!(got, Type::ManagedReference(_)))
                        {
                            // TODO: check the mref and ptr point to the same mem.
                            continue;
                        }
                        return Err(format!(
                            "Expected a call argument of type {arg_tpe:?}, but got {got:?} calling{site:?}"
                        ));
                    }
                }
                Ok(())
            }
            Self::CallI { args, sig, fn_ptr } => {
                let ptr = fn_ptr.validate(vctx, tmp_loc)?;
                if sig.inputs().len() != args.len() {
                    return Err(format!(
                        "Expected {} arguments, got {}",
                        sig.inputs().len(),
                        args.len()
                    ));
                }
                for (arg, tpe) in args.iter().zip(sig.inputs().iter()) {
                    let arg = arg.validate(vctx, tmp_loc)?;
                    if arg != *tpe {
                        return Err(format!(
                            "Expected a call argument of type {tpe:?}, but got {arg:?}"
                        ));
                    }
                }
                Ok(())
            }
            Self::VoidRet => Ok(()),
            Self::SourceFileInfo(_) => Ok(()),
            Self::Nop => Ok(()),
            Self::Throw(execption) => {
                let tpe = execption.validate(vctx, tmp_loc)?;
                if let Some(_) = tpe.as_dotnet() {
                    Ok(())
                } else {
                    Err("`throw` instruction suplied with a non-object type.".into())
                }
            }
            Self::Ret { tree } => {
                let expected = vctx.sig().output();
                let got = tree.validate(vctx, tmp_loc)?;
                if got != *expected {
                    Err(format!(
                        "Mismatched return type. Expected {expected:?} got {got:?}"
                    ))
                } else {
                    Ok(())
                }
            }
            Self::SetField { addr, value, desc } => {
                let addr = addr.validate(vctx, tmp_loc)?;
                let value = value.validate(vctx, tmp_loc)?;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allocating_tmps() {
        let mut original_value = CILNode::SubTrees(Box::new((
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
        )));
        //let mut method = crate::method::Method::new(crate::access_modifier::AccessModifer::Private,crate::method::MethodType::Static,FnSig::new(&[Type::I32],&Type::Void),"a",vec![],vec![]);
        original_value.allocate_tmps(None, &mut vec![]);
        println!("original_value:{original_value:?}");
        //let _trees = original_value.sheed_trees();
        //let _ops: Vec<_> = trees.iter().map(CILRoot::into_ops).collect();
    }

    // Test comparison optimizations
    #[test]
    fn optimize_equality_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BTrue {
            target: 1,
            sub_target: 0,
            cond: CILNode::Eq(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BEq {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_lt_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BTrue {
            target: 1,
            sub_target: 0,
            cond: CILNode::Lt(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BLt {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_gt_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BTrue {
            target: 1,
            sub_target: 0,
            cond: CILNode::Gt(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BGt {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_lt_un_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BTrue {
            target: 1,
            sub_target: 0,
            cond: CILNode::LtUn(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BLtUn {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_gt_un_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BTrue {
            target: 1,
            sub_target: 0,
            cond: CILNode::GtUn(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BGtUn {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_ne_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BFalse {
            target: 1,
            sub_target: 0,
            cond: CILNode::Eq(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BNe {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_not_lt_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BFalse {
            target: 1,
            sub_target: 0,
            cond: CILNode::Lt(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BGe {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
    #[test]
    fn optimize_not_gt_branch() {
        let mut opt_count = 0;
        let mut comparison = CILRoot::BFalse {
            target: 1,
            sub_target: 0,
            cond: CILNode::Gt(Box::new(CILNode::LDArg(0)), Box::new(CILNode::LDArg(1))),
        };
        comparison.opt(&mut opt_count);
        assert_eq!(
            comparison,
            CILRoot::BLe {
                target: 1,
                sub_target: 0,
                a: Box::new(CILNode::LDArg(0)),
                b: Box::new(CILNode::LDArg(1))
            }
        )
    }
}
/*

    compile_test::assign::stable::debug
    compile_test::assign::stable::release

    compile_test::slice::stable::debug
    compile_test::slice::stable::release


*/
