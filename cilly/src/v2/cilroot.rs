use super::{
    bimap::HashWrapper, cilnode::MethodKind, Assembly, CILNode, ClassRef, FnSig, MethodRef,
    MethodRefIdx, NodeIdx, StringIdx, Type,
};
use crate::cil_root::CILRoot as V1Root;
//use crate::cil_node::CILNode as V1Node;
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum CILRoot {
    StLoc(u64, NodeIdx),
    StArg(u64, NodeIdx),
    Ret(NodeIdx),
    Throw(NodeIdx),
    VoidRet,
    Break,
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
    Call(Box<(MethodRefIdx, Box<[NodeIdx]>)>),
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum BranchCond {
    True(NodeIdx),
    False(NodeIdx),
    Eq(NodeIdx, NodeIdx),
    Ne(NodeIdx, NodeIdx),
    Lt(NodeIdx, NodeIdx, CmpKind),
    Gt(NodeIdx, NodeIdx, CmpKind),
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum CmpKind {
    Ordered,
    Unordered,
    Signed,
    Unsigned,
}
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]

pub struct RootIdx(u64);
impl HashWrapper for RootIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
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
            _ => todo!("v1:{v1:?}"),
        }
    }
}
