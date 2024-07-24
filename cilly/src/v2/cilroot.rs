use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    cilnode::MethodKind,
    field::FieldIdx,
    Assembly, CILNode, ClassRef, FieldDesc, Float, FnSig, Int, MethodRef, MethodRefIdx, NodeIdx,
    SigIdx, StaticFieldDesc, StaticFieldIdx, StringIdx, Type,
};
use crate::cil_root::CILRoot as V1Root;
//use crate::cil_node::CILNode as V1Node;
#[derive(PartialEq, Hash, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum CILRoot {
    StLoc(u64, NodeIdx),
    StArg(u64, NodeIdx),
    Ret(NodeIdx),
    Pop(NodeIdx),
    Throw(NodeIdx),
    VoidRet,
    Break,
    Nop,
    Branch {
        target: u32,
        sub_target: u32,
        cond: Option<Box<BranchCond>>,
    },
    SourceFileInfo {
        line_start: u32,
        line_len: u16,
        col_start: u16,
        col_len: u16,
        file: StringIdx,
    },
    // Field, value, addr
    SetField(Box<(FieldIdx, NodeIdx, NodeIdx)>),
    Call(Box<(MethodRefIdx, Box<[NodeIdx]>)>),
    // value, addr, type
    StInd(Box<(NodeIdx, NodeIdx, Type, bool)>),
    // dst, val, count
    InitBlk(Box<(NodeIdx, NodeIdx, NodeIdx)>),
    // dst src len
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
}

#[derive(Hash, PartialEq, Eq, Clone, Debug, Serialize, Deserialize)]
pub enum BranchCond {
    True(NodeIdx),
    False(NodeIdx),
    Eq(NodeIdx, NodeIdx),
    Ne(NodeIdx, NodeIdx),
    Lt(NodeIdx, NodeIdx, CmpKind),
    Gt(NodeIdx, NodeIdx, CmpKind),
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
    fn from_hash(val: BiMapIndex) -> Self {
        Self(val)
    }
    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
impl CILRoot {
    pub fn from_v1(v1: &V1Root, asm: &mut Assembly) -> Self {
        match v1 {
            V1Root::SourceFileInfo(sfi) => {
                let line_start = sfi.0.start.min(u32::MAX as u64) as u32;
                let line_end = sfi.0.end.min(u32::MAX as u64) as u32;
                let line_len = (line_end - line_start).min(u16::MAX as u32) as u16;
                let col_start = sfi.1.start.min(u16::MAX as u64) as u16;
                let col_end = sfi.1.end.min(u16::MAX as u64) as u16;
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
                Self::Ret(asm.node_idx(tree))
            }
            V1Root::Pop { tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::Pop(asm.node_idx(tree))
            }
            V1Root::Throw(tree) => {
                let tree = CILNode::from_v1(tree, asm);
                Self::Throw(asm.node_idx(tree))
            }
            V1Root::STLoc { local, tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::StLoc(*local as u64, asm.node_idx(tree))
            }
            V1Root::STArg { arg, tree } => {
                let tree = CILNode::from_v1(tree, asm);
                Self::StArg(*arg as u64, asm.node_idx(tree))
            }
            V1Root::GoTo { target, sub_target } => Self::Branch {
                target: *target,
                sub_target: *sub_target,
                cond: None,
            },
            V1Root::BEq {
                target,
                sub_target,
                a,
                b,
            } => {
                let a = CILNode::from_v1(a, asm);
                let b = CILNode::from_v1(b, asm);
                Self::Branch {
                    target: *target,
                    sub_target: *sub_target,
                    cond: Some(Box::new(BranchCond::Eq(asm.node_idx(a), asm.node_idx(b)))),
                }
            }
            V1Root::BTrue {
                target,
                sub_target,
                cond,
            } => {
                let cond = CILNode::from_v1(cond, asm);
                Self::Branch {
                    target: *target,
                    sub_target: *sub_target,
                    cond: Some(Box::new(BranchCond::True(asm.node_idx(cond)))),
                }
            }
            V1Root::BFalse {
                target,
                sub_target,
                cond,
            } => {
                let cond = CILNode::from_v1(cond, asm);
                Self::Branch {
                    target: *target,
                    sub_target: *sub_target,
                    cond: Some(Box::new(BranchCond::False(asm.node_idx(cond)))),
                }
            }
            V1Root::BNe {
                target,
                sub_target,
                a,
                b,
            } => {
                let a = CILNode::from_v1(a, asm);
                let b = CILNode::from_v1(b, asm);
                Self::Branch {
                    target: *target,
                    sub_target: *sub_target,
                    cond: Some(Box::new(BranchCond::Ne(asm.node_idx(a), asm.node_idx(b)))),
                }
            }
            V1Root::Call { site, args } => {
                let args: Box<[_]> = args
                    .iter()
                    .map(|arg| {
                        let node = CILNode::from_v1(arg, asm);
                        asm.node_idx(node)
                    })
                    .collect();
                let sig = FnSig::from_v1(site.signature(), asm);
                let sig = asm.sig_idx(sig);
                let generics: Box<[_]> = site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = site.class().map(|dt| {
                    let cref = ClassRef::from_v1(dt, asm);
                    asm.class_idx(cref)
                });
                let name = asm.alloc_string(site.name());
                let method_ref = if site.is_static() {
                    MethodRef::new(class, name, sig, MethodKind::Static, generics)
                } else {
                    MethodRef::new(class, name, sig, MethodKind::Instance, generics)
                };
                let method_ref = asm.methodref_idx(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            V1Root::CallVirt { site, args } => {
                let args: Box<[_]> = args
                    .iter()
                    .map(|arg| {
                        let node = CILNode::from_v1(arg, asm);
                        asm.node_idx(node)
                    })
                    .collect();
                let sig = FnSig::from_v1(site.signature(), asm);
                let sig = asm.sig_idx(sig);
                let generics: Box<[_]> = site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = site.class().map(|dt| {
                    let cref = ClassRef::from_v1(dt, asm);
                    asm.class_idx(cref)
                });
                let name = asm.alloc_string(site.name());
                assert!(!site.is_static());
                let method_ref = MethodRef::new(class, name, sig, MethodKind::Virtual, generics);
                let method_ref = asm.methodref_idx(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            V1Root::SetField { value, addr, desc } => {
                let field = FieldDesc::from_v1(desc, asm);
                let field = asm.field_idx(field);
                let value = CILNode::from_v1(value, asm);
                let value = asm.node_idx(value);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);

                Self::SetField(Box::new((field, addr, value)))
            }
            V1Root::STIndI8(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I8), false)))
            }

            V1Root::STIndI16(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I16), false)))
            }
            V1Root::STIndI32(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I32), false)))
            }
            V1Root::STIndI64(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::I64), false)))
            }
            V1Root::STIndISize(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Int(Int::ISize), false)))
            }
            V1Root::STIndPtr(addr, val, ptr) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                let ptr = Type::from_v1(ptr, asm);
                Self::StInd(Box::new((addr, val, ptr, false)))
            }
            V1Root::STObj {
                tpe,
                addr_calc,
                value_calc,
            } => {
                let value_calc = CILNode::from_v1(value_calc, asm);
                let value_calc = asm.node_idx(value_calc);
                let addr_calc = CILNode::from_v1(addr_calc, asm);
                let addr_calc = asm.node_idx(addr_calc);
                let tpe = Type::from_v1(tpe, asm);
                Self::StInd(Box::new((addr_calc, value_calc, tpe, false)))
            }
            V1Root::STIndF32(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Float(Float::F32), false)))
            }
            V1Root::STIndF64(addr, val) => {
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let addr = CILNode::from_v1(addr, asm);
                let addr = asm.node_idx(addr);
                Self::StInd(Box::new((addr, val, Type::Float(Float::F64), false)))
            }
            V1Root::Nop => Self::Nop,
            V1Root::Break => Self::Break,
            V1Root::InitBlk { dst, val, count } => {
                let dst = CILNode::from_v1(dst, asm);
                let dst = asm.node_idx(dst);
                let val = CILNode::from_v1(val, asm);
                let val = asm.node_idx(val);
                let count = CILNode::from_v1(count, asm);
                let count = asm.node_idx(count);
                Self::InitBlk(Box::new((dst, val, count)))
            }
            V1Root::CpBlk { dst, src, len } => {
                let dst = CILNode::from_v1(dst, asm);
                let dst = asm.node_idx(dst);
                let src = CILNode::from_v1(src, asm);
                let src = asm.node_idx(src);
                let len = CILNode::from_v1(len, asm);
                let len = asm.node_idx(len);
                Self::CpBlk(Box::new((dst, src, len)))
            }
            V1Root::CallI { sig, fn_ptr, args } => {
                let sig = FnSig::from_v1(sig, asm);
                let sig = asm.sig_idx(sig);
                let ptr = CILNode::from_v1(fn_ptr, asm);
                let ptr = asm.node_idx(ptr);
                let args: Box<[_]> = args
                    .iter()
                    .map(|arg| {
                        let arg = CILNode::from_v1(arg, asm);
                        asm.node_idx(arg)
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
                match &mut tmp {
                    Self::StInd(inner) => inner.3 = true,
                    _ => panic!(),
                }
                tmp
            }
            V1Root::ReThrow => Self::ReThrow,
            V1Root::SetStaticField { descr, value } => {
                let descr = StaticFieldDesc::from_v1(descr, asm);
                let val = CILNode::from_v1(value, asm);
                Self::SetStaticField {
                    field: asm.sfld_idx(descr),
                    val: asm.node_idx(val),
                }
            }
            _ => todo!("v1:{v1:?}"),
        }
    }
}
