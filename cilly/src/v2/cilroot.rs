use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::MethodKind,
    field::FieldIdx,
    Assembly, CILNode, FieldDesc, Float, Int, MethodRef, MethodRefIdx, NodeIdx, SigIdx,
    StaticFieldDesc, StaticFieldIdx, StringIdx, Type, TypeIdx,
};
use crate::cil_root::CILRoot as V1Root;
//use crate::cil_node::CILNode as V1Node;
#[derive(PartialEq, Hash, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum CILRoot {
    StLoc(u32, NodeIdx),
    StArg(u32, NodeIdx),
    Ret(NodeIdx),
    Pop(NodeIdx),
    Throw(NodeIdx),
    VoidRet,
    Break,
    Nop,
    /// target subtarget cond
    Branch(Box<(u32, u32, Option<BranchCond>)>),
    SourceFileInfo {
        line_start: u32,
        line_len: u16,
        col_start: u16,
        col_len: u16,
        file: StringIdx,
    },
    /// Field,  addr,value
    SetField(Box<(FieldIdx, NodeIdx, NodeIdx)>),
    Call(Box<(MethodRefIdx, Box<[NodeIdx]>)>),
    /// addr, value, type
    StInd(Box<(NodeIdx, NodeIdx, Type, bool)>),
    /// dst, val, count
    InitBlk(Box<(NodeIdx, NodeIdx, NodeIdx)>),
    /// dst src len
    CpBlk(Box<(NodeIdx, NodeIdx, NodeIdx)>),
    /// Calls fn pointer with args
    CallI(Box<(NodeIdx, SigIdx, Box<[NodeIdx]>)>),
    /// Exits a protected region of code.
    ExitSpecialRegion {
        target: u32,
        source: u32,
    },
    /// Rethrows the current exception
    ReThrow,
    /// Sets the static field to a value.
    SetStaticField {
        field: StaticFieldIdx,
        val: NodeIdx,
    },
    CpObj {
        src: NodeIdx,
        dst: NodeIdx,
        tpe: TypeIdx,
    },
    /// Executing this root is instant UB.
    Unreachable(StringIdx),
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum BranchCond {
    True(NodeIdx),
    False(NodeIdx),
    Eq(NodeIdx, NodeIdx),
    Ne(NodeIdx, NodeIdx),
    Lt(NodeIdx, NodeIdx, CmpKind),
    Gt(NodeIdx, NodeIdx, CmpKind),
    Le(NodeIdx, NodeIdx, CmpKind),
    Ge(NodeIdx, NodeIdx, CmpKind),
}
impl BranchCond {
    pub(crate) fn nodes(&self) -> Vec<NodeIdx> {
        match self {
            BranchCond::True(cond) | BranchCond::False(cond) => vec![*cond],
            BranchCond::Eq(lhs, rhs)
            | BranchCond::Ne(lhs, rhs)
            | BranchCond::Lt(lhs, rhs, _)
            | BranchCond::Gt(lhs, rhs, _)
            | BranchCond::Le(lhs, rhs, _)
            | BranchCond::Ge(lhs, rhs, _) => vec![*lhs, *rhs],
        }
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum CmpKind {
    Ordered,
    Unordered,
    Signed,
    Unsigned,
}
#[derive(Hash, PartialEq, Eq, Copy, Clone, Debug, Serialize, Deserialize)]

pub struct RootIdx(BiMapIndex);
impl IntoBiMapIndex for RootIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
impl CILRoot {
    /// Returns a mutable reference to all the arguments of this CIL root, in the order they are evaluated.
    pub fn nodes_mut(&mut self) -> Box<[&mut NodeIdx]> {
        match self {
            CILRoot::Unreachable(_) => [].into(),
            CILRoot::StLoc(_, tree)
            | CILRoot::StArg(_, tree)
            | CILRoot::Ret(tree)
            | CILRoot::Pop(tree)
            | CILRoot::Throw(tree)
            | CILRoot::SetStaticField { val: tree, .. } => [tree].into(),
            CILRoot::SourceFileInfo { .. }
            | CILRoot::ExitSpecialRegion { .. }
            | CILRoot::VoidRet
            | CILRoot::Break
            | CILRoot::Nop
            | CILRoot::ReThrow => [].into(),
            CILRoot::Branch(info) => {
                let (_, _, cond) = info.as_mut();
                let Some(cond) = cond else { return [].into() };
                match cond {
                    BranchCond::True(cond) | BranchCond::False(cond) => [cond].into(),
                    BranchCond::Eq(lhs, rhs)
                    | BranchCond::Ne(lhs, rhs)
                    | BranchCond::Lt(lhs, rhs, _)
                    | BranchCond::Gt(lhs, rhs, _)
                    | BranchCond::Le(lhs, rhs, _)
                    | BranchCond::Ge(lhs, rhs, _) => [lhs, rhs].into(),
                }
            }
            CILRoot::SetField(info) => {
                let (_, addr, val) = info.as_mut();
                [addr, val].into()
            }
            CILRoot::Call(info) => many_mut(&mut info.1).into(),
            CILRoot::StInd(info) => {
                let (addr, val, _, _) = info.as_mut();
                [addr, val].into()
            }
            CILRoot::InitBlk(info) | CILRoot::CpBlk(info) => {
                let (addr, val, len) = info.as_mut();
                [addr, val, len].into()
            }
            CILRoot::CallI(info) => {
                let (ptr, _, args) = info.as_mut();
                let mut args = many_mut(args);
                args.push(ptr);
                args.into()
            }
            CILRoot::CpObj { src, dst, .. } => [src, dst].into(),
        }
    }
    #[allow(clippy::too_many_lines)]
    pub fn from_v1(v1: &V1Root, asm: &mut Assembly) -> Self {
        match v1 {
            V1Root::SourceFileInfo(sfi) => {
                let line_start = u32::try_from(sfi.0.start.min(u64::from(u32::MAX))).unwrap();
                let line_end = u32::try_from(sfi.0.end.min(u64::from(u32::MAX))).unwrap();
                let line_len =
                    u16::try_from((line_end - line_start).min(u32::from(u16::MAX))).unwrap();
                let col_start = u16::try_from(sfi.1.start.min(u64::from(u16::MAX))).unwrap();
                let col_end = u16::try_from(sfi.1.end.min(u64::from(u16::MAX))).unwrap();
                let col_len = col_end - col_start;
                let file = asm.alloc_string(sfi.2.clone());
                Self::SourceFileInfo {
                    line_start,
                    line_len,
                    col_start,
                    col_len,
                    file,
                }
            }
            V1Root::VoidRet => Self::VoidRet,
            V1Root::Ret { tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::Ret(asm.alloc_node(tree))
            }
            V1Root::Pop { tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::Pop(asm.alloc_node(tree))
            }
            V1Root::Throw(tree) => {
                let tree = CILNode::from_v1(tree, asm);
                Self::Throw(asm.alloc_node(tree))
            }
            V1Root::STLoc { local, tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::StLoc(*local, asm.alloc_node(tree))
            }
            V1Root::STArg { arg, tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::StArg(*arg, asm.alloc_node(tree))
            }
            V1Root::GoTo { target, sub_target } => {
                Self::Branch(Box::new((*target, *sub_target, None)))
            }
            V1Root::BEq {
                target,
                sub_target,
                a,
                b,
            } => {
                let a = CILNode::from_v1(a, asm);
                let b = CILNode::from_v1(b, asm);
                Self::Branch(Box::new((
                    *target,
                    *sub_target,
                    Some(BranchCond::Eq(asm.alloc_node(a), asm.alloc_node(b))),
                )))
            }
            V1Root::BNe {
                target,
                sub_target,
                a,
                b,
            } => {
                let a = CILNode::from_v1(a, asm);
                let b = CILNode::from_v1(b, asm);
                Self::Branch(Box::new((
                    *target,
                    *sub_target,
                    Some(BranchCond::Ne(asm.alloc_node(a), asm.alloc_node(b))),
                )))
            }
            V1Root::BTrue {
                target,
                sub_target,
                cond,
            } => {
                let cond = CILNode::from_v1(cond, asm);
                Self::Branch(Box::new((
                    *target,
                    *sub_target,
                    Some(BranchCond::True(asm.alloc_node(cond))),
                )))
            }
            V1Root::BFalse {
                target,
                sub_target,
                cond,
            } => {
                let cond = CILNode::from_v1(cond, asm);
                Self::Branch(Box::new((
                    *target,
                    *sub_target,
                    Some(BranchCond::False(asm.alloc_node(cond))),
                )))
            }

            V1Root::Call { site, args } => {
                let args: Box<[_]> = args
                    .iter()
                    .map(|arg| {
                        let node = CILNode::from_v1(arg, asm);
                        asm.alloc_node(node)
                    })
                    .collect();
                let sig = asm.alloc_sig(site.signature().clone());
                let generics: Box<[_]> = (site.generics()).into();
                let class = site.class().unwrap_or_else(|| *asm.main_module());
                let name = asm.alloc_string(site.name());
                let method_ref = if site.is_static() {
                    MethodRef::new(class, name, sig, MethodKind::Static, generics)
                } else {
                    MethodRef::new(class, name, sig, MethodKind::Instance, generics)
                };
                let method_ref = asm.alloc_methodref(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            V1Root::CallVirt { site, args } => {
                let args: Box<[_]> = args
                    .iter()
                    .map(|arg| {
                        let node = CILNode::from_v1(arg, asm);
                        asm.alloc_node(node)
                    })
                    .collect();
                let sig = asm.alloc_sig(site.signature().clone());
                let generics: Box<[_]> = (site.generics()).into();
                let class = site.class().unwrap_or_else(|| *asm.main_module());
                let name = asm.alloc_string(site.name());
                assert!(!site.is_static());
                let method_ref = MethodRef::new(class, name, sig, MethodKind::Virtual, generics);
                let method_ref = asm.alloc_methodref(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            V1Root::SetField { value, addr, desc } => {
                let field = FieldDesc::from_v1(desc, asm);
                let field = asm.alloc_field(field);
                let value = CILNode::from_v1(value, asm);
                let value = asm.alloc_node(value);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);

                Self::SetField(Box::new((field, addr, value)))
            }
            V1Root::STIndI8(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I8), false)))
            }

            V1Root::STIndI16(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I16), false)))
            }
            V1Root::STIndI32(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I32), false)))
            }
            V1Root::STIndI64(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I64), false)))
            }
            V1Root::STIndISize(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::ISize), false)))
            }
            V1Root::STIndPtr(addr, val, ptr) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);

                let ptr = asm.nptr(**ptr);
                Self::StInd(Box::new((addr, val, ptr, false)))
            }
            V1Root::STObj {
                tpe,
                addr_calc,
                value_calc,
            } => {
                let value_calc = CILNode::from_v1(value_calc, asm);
                let value_calc = asm.alloc_node(value_calc);
                let addr_calc = CILNode::from_v1(addr_calc, asm);
                let addr_calc = asm.alloc_node(addr_calc);

                Self::StInd(Box::new((addr_calc, value_calc, **tpe, false)))
            }
            V1Root::STIndF32(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Float(Float::F32), false)))
            }
            V1Root::STIndF64(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.alloc_node(addr);
                Self::StInd(Box::new((addr, val, Type::Float(Float::F64), false)))
            }
            V1Root::Nop => Self::Nop,
            V1Root::Break => Self::Break,
            V1Root::InitBlk { dst, val, count } => {
                let dst = CILNode::from_v1(dst, asm);
                let dst = asm.alloc_node(dst);
                let val = CILNode::from_v1(val, asm);
                let val = asm.alloc_node(val);
                let count = CILNode::from_v1(count, asm);
                let count = asm.alloc_node(count);
                Self::InitBlk(Box::new((dst, val, count)))
            }
            V1Root::CpBlk { dst, src, len } => {
                let dst = CILNode::from_v1(dst, asm);
                let dst = asm.alloc_node(dst);
                let src = CILNode::from_v1(src, asm);
                let src = asm.alloc_node(src);
                let len = CILNode::from_v1(len, asm);
                let len = asm.alloc_node(len);
                Self::CpBlk(Box::new((dst, src, len)))
            }
            V1Root::CallI { sig, fn_ptr, args } => {
                let sig = asm.alloc_sig(*sig.clone());
                let ptr = CILNode::from_v1(fn_ptr, asm);
                let ptr = asm.alloc_node(ptr);
                let args: Box<[_]> = args
                    .iter()
                    .map(|arg| {
                        let arg = CILNode::from_v1(arg, asm);
                        asm.alloc_node(arg)
                    })
                    .collect();
                Self::CallI(Box::new((ptr, sig, args)))
            }
            V1Root::JumpingPad { source, target } => Self::ExitSpecialRegion {
                target: *target,
                source: *source,
            },
            V1Root::Volatile(inner) => {
                let mut tmp = Self::from_v1(inner, asm);
                if let Self::StInd(inner) = &mut tmp {
                    inner.3 = true;
                } else {
                    panic!()
                }
                tmp
            }
            V1Root::ReThrow => Self::ReThrow,
            V1Root::SetStaticField { descr, value } => {
                let descr = StaticFieldDesc::from_v1(descr, asm);
                let val = CILNode::from_v1(value, asm);
                Self::SetStaticField {
                    field: asm.alloc_sfld(descr),
                    val: asm.alloc_node(val),
                }
            }
            _ => todo!("v1:{v1:?}"),
        }
    }
    /// Maps this root using `root_map` and `node_map`.
    #[allow(clippy::too_many_lines)]
    #[must_use]
    pub fn map(
        self,
        asm: &mut Assembly,
        root_map: &mut impl FnMut(Self, &mut Assembly) -> Self,
        node_map: &mut impl FnMut(CILNode, &mut Assembly) -> CILNode,
    ) -> Self {
        match self {
            CILRoot::Unreachable(_) => root_map(self, asm),
            CILRoot::StLoc(loc, val) => {
                let val: CILNode = asm.get_node(val).clone().map(asm, node_map);
                let root = CILRoot::StLoc(loc, asm.alloc_node(val));
                root_map(root, asm)
            }
            CILRoot::StArg(arg, val) => {
                let val: CILNode = asm.get_node(val).clone().map(asm, node_map);
                let root = CILRoot::StArg(arg, asm.alloc_node(val));
                root_map(root, asm)
            }
            CILRoot::Ret(ret) => {
                let ret = asm.get_node(ret).clone().map(asm, node_map);
                let root = CILRoot::Ret(asm.alloc_node(ret));
                root_map(root, asm)
            }
            CILRoot::Pop(pop) => {
                let pop = asm.get_node(pop).clone().map(asm, node_map);
                let root = CILRoot::Pop(asm.alloc_node(pop));
                root_map(root, asm)
            }
            CILRoot::Throw(throw) => {
                let throw = asm.get_node(throw).clone().map(asm, node_map);
                let root = CILRoot::Throw(asm.alloc_node(throw));
                root_map(root, asm)
            }
            CILRoot::SourceFileInfo { .. }
            | CILRoot::VoidRet
            | CILRoot::Break
            | CILRoot::Nop
            | CILRoot::ExitSpecialRegion { .. }
            | CILRoot::ReThrow => root_map(self, asm),
            CILRoot::Branch(branch) => {
                let (a, b, cond) = *branch;
                let cond = match cond {
                    Some(BranchCond::True(tr)) => {
                        let tr = asm.get_node(tr).clone().map(asm, node_map);
                        Some(BranchCond::True(asm.alloc_node(tr)))
                    }
                    Some(BranchCond::False(fl)) => {
                        let fl = asm.get_node(fl).clone().map(asm, node_map);
                        Some(BranchCond::False(asm.alloc_node(fl)))
                    }
                    Some(BranchCond::Eq(lhs, rhs)) => {
                        let lhs = asm.get_node(lhs).clone().map(asm, node_map);
                        let rhs = asm.get_node(rhs).clone().map(asm, node_map);
                        Some(BranchCond::Eq(asm.alloc_node(lhs), asm.alloc_node(rhs)))
                    }
                    Some(BranchCond::Ne(lhs, rhs)) => {
                        let lhs = asm.get_node(lhs).clone().map(asm, node_map);
                        let rhs = asm.get_node(rhs).clone().map(asm, node_map);
                        Some(BranchCond::Ne(asm.alloc_node(lhs), asm.alloc_node(rhs)))
                    }
                    Some(BranchCond::Lt(lhs, rhs, cmp_kind)) => {
                        let lhs = asm.get_node(lhs).clone().map(asm, node_map);
                        let rhs = asm.get_node(rhs).clone().map(asm, node_map);
                        Some(BranchCond::Lt(
                            asm.alloc_node(lhs),
                            asm.alloc_node(rhs),
                            cmp_kind,
                        ))
                    }
                    Some(BranchCond::Gt(lhs, rhs, cmp_kind)) => {
                        let lhs = asm.get_node(lhs).clone().map(asm, node_map);
                        let rhs = asm.get_node(rhs).clone().map(asm, node_map);
                        Some(BranchCond::Gt(
                            asm.alloc_node(lhs),
                            asm.alloc_node(rhs),
                            cmp_kind,
                        ))
                    }
                    Some(BranchCond::Le(lhs, rhs, cmp_kind)) => {
                        let lhs = asm.get_node(lhs).clone().map(asm, node_map);
                        let rhs = asm.get_node(rhs).clone().map(asm, node_map);
                        Some(BranchCond::Le(
                            asm.alloc_node(lhs),
                            asm.alloc_node(rhs),
                            cmp_kind,
                        ))
                    }
                    Some(BranchCond::Ge(lhs, rhs, cmp_kind)) => {
                        let lhs = asm.get_node(lhs).clone().map(asm, node_map);
                        let rhs = asm.get_node(rhs).clone().map(asm, node_map);
                        Some(BranchCond::Ge(
                            asm.alloc_node(lhs),
                            asm.alloc_node(rhs),
                            cmp_kind,
                        ))
                    }
                    None => None,
                };
                let root = CILRoot::Branch(Box::new((a, b, cond)));
                root_map(root, asm)
            }
            CILRoot::SetStaticField { field, val } => {
                let val = asm.get_node(val).clone().map(asm, node_map);
                let root = CILRoot::SetStaticField {
                    field,
                    val: asm.alloc_node(val),
                };
                root_map(root, asm)
            }
            CILRoot::SetField(set_field) => {
                let (field, addr, val) = *set_field;
                let addr = asm.get_node(addr).clone().map(asm, node_map);
                let val = asm.get_node(val).clone().map(asm, node_map);
                let root =
                    CILRoot::SetField(Box::new((field, asm.alloc_node(addr), asm.alloc_node(val))));
                root_map(root, asm)
            }
            CILRoot::Call(call_info) => {
                let (method_id, args) = *call_info;
                let args = args
                    .iter()
                    .map(|arg| {
                        let node = asm.get_node(*arg).clone().map(asm, node_map);
                        asm.alloc_node(node)
                    })
                    .collect();

                let root = CILRoot::Call(Box::new((method_id, args)));
                root_map(root, asm)
            }
            CILRoot::StInd(ind) => {
                let (addr, val, tpe, volitale) = *ind;
                let addr = asm.get_node(addr).clone().map(asm, node_map);
                let val = asm.get_node(val).clone().map(asm, node_map);
                let root = CILRoot::StInd(Box::new((
                    asm.alloc_node(addr),
                    asm.alloc_node(val),
                    tpe,
                    volitale,
                )));
                root_map(root, asm)
            }
            CILRoot::CpObj { src, dst, tpe } => {
                let src = asm.get_node(src).clone().map(asm, node_map);
                let dst = asm.get_node(dst).clone().map(asm, node_map);
                let root = CILRoot::CpObj {
                    src: asm.alloc_node(src),
                    dst: asm.alloc_node(dst),
                    tpe,
                };
                root_map(root, asm)
            }
            CILRoot::InitBlk(blk) => {
                let (dst, val, count) = *blk;
                let dst = asm.get_node(dst).clone().map(asm, node_map);
                let val = asm.get_node(val).clone().map(asm, node_map);
                let count = asm.get_node(count).clone().map(asm, node_map);
                let root = CILRoot::InitBlk(Box::new((
                    asm.alloc_node(dst),
                    asm.alloc_node(val),
                    asm.alloc_node(count),
                )));
                root_map(root, asm)
            }
            CILRoot::CpBlk(blk) => {
                let (dst, src, len) = *blk;
                let dst = asm.get_node(dst).clone().map(asm, node_map);
                let src = asm.get_node(src).clone().map(asm, node_map);
                let len = asm.get_node(len).clone().map(asm, node_map);
                let root = CILRoot::CpBlk(Box::new((
                    asm.alloc_node(dst),
                    asm.alloc_node(src),
                    asm.alloc_node(len),
                )));
                root_map(root, asm)
            }
            CILRoot::CallI(call_info) => {
                let (ptr, sig, args) = *call_info;
                let args = args
                    .iter()
                    .map(|arg| {
                        let node = asm.get_node(*arg).clone().map(asm, node_map);
                        asm.alloc_node(node)
                    })
                    .collect();
                let ptr = asm.get_node(ptr).clone().map(asm, node_map);
                let root = CILRoot::CallI(Box::new((asm.alloc_node(ptr), sig, args)));
                root_map(root, asm)
            }
        }
    }
}
/// Changes a mutable reference to a slice to an vec of mutable references to the elements.
fn many_mut<T>(input: &mut [T]) -> Vec<&mut T> {
    let input_len = input.len();
    let res = match input.len() {
        0 => [].into(),
        1 => [&mut input[0]].into(),
        2 => {
            let (a, b) = input.split_at_mut(1);

            [&mut a[0], &mut b[0]].into()
        }
        3 => {
            let (a, b) = input.split_at_mut(1);
            let (b, c) = b.split_at_mut(1);
            [&mut a[0], &mut b[0], &mut c[0]].into()
        }
        4 => {
            let (lhs, rhs) = input.split_at_mut(2);
            let (a, b) = lhs.split_at_mut(1);
            let (c, d) = rhs.split_at_mut(1);
            [&mut a[0], &mut b[0], &mut c[0], &mut d[0]].into()
        }
        _ => {
            let half = input.len() / 2;
            let (lhs, rhs) = input.split_at_mut(half);
            let mut result = many_mut(lhs);
            result.extend(many_mut(rhs));
            result
        }
    };
    assert_eq!(res.len(), input_len);
    res
}
#[test]
fn test_many_mut() {
    // 0 elements
    many_mut::<i32>(&mut []);
    // 1 element
    *many_mut(&mut [1])[0] = 1;
    // 2 elements
    *many_mut(&mut [1, 2])[0] = 1;
    *many_mut(&mut [1, 2])[1] = 2;
    // 3 elements
    *many_mut(&mut [1, 2, 3])[0] = 1;
    *many_mut(&mut [1, 2, 3])[1] = 2;
    *many_mut(&mut [1, 2, 3])[2] = 3;
    // 4 elements
    *many_mut(&mut [1, 2, 3, 4])[0] = 1;
    *many_mut(&mut [1, 2, 3, 4])[1] = 2;
    *many_mut(&mut [1, 2, 3, 4])[2] = 3;
    *many_mut(&mut [1, 2, 3, 4])[3] = 4;
    // 5 elements
    *many_mut(&mut [1, 2, 3, 4, 5])[0] = 1;
    *many_mut(&mut [1, 2, 3, 4, 5])[1] = 2;
    *many_mut(&mut [1, 2, 3, 4, 5])[2] = 3;
    *many_mut(&mut [1, 2, 3, 4, 5])[3] = 4;
    *many_mut(&mut [1, 2, 3, 4, 5])[4] = 5;
}
