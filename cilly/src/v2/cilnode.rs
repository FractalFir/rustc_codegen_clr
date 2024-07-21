use super::{bimap::HashWrapper, Assembly, Const, Int, MethodRefIdx, TypeIdx};
use crate::{
    cil_node::CILNode as V1Node,
    v2::{ClassRef, FnSig, MethodRef, Type},
};
#[derive(Hash, PartialEq, Eq, Clone, Default, Debug)]
pub struct NodeIdx(u64);
impl HashWrapper for NodeIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum CILNode {
    Const(Const),
    BinOp(NodeIdx, NodeIdx, BinOp),
    UnOp(NodeIdx, UnOp),
    LdLoc(u64),
    LdArg(u64),
    Call(Box<(MethodRefIdx, Box<[NodeIdx]>)>),
    IntCast {
        input: NodeIdx,
        target: Int,
        extend: ExtendKind,
    },
    RefToPtr(NodeIdx),
    PtrCast(NodeIdx, PtrCastRes),
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum PtrCastRes {
    Ptr(TypeIdx),
    USize,
    ISize,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]

pub enum ExtendKind {
    ZeroExtend,
    SignExtend,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum MethodKind {
    Static,
    Instance,
    Virtual,
    Constructor,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum UnOp {
    Not,
    Neg,
}
#[derive(Hash, PartialEq, Eq, Clone, Debug)]

pub enum BinOp {
    Add,
}
impl CILNode {
    pub fn from_v1(v1: &V1Node, asm: &mut Assembly) -> Self {
        match v1 {
            V1Node::LDArg(arg) => CILNode::LdArg(*arg as u64),
            V1Node::LDLoc(arg) => CILNode::LdLoc(*arg as u64),
            V1Node::ZeroExtendToU64(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.node_idx(node),
                    target: Int::U64,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::ConvU8(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.node_idx(node),
                    target: Int::U8,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::ConvU16(inner) => {
                let node = Self::from_v1(inner, asm);
                CILNode::IntCast {
                    input: asm.node_idx(node),
                    target: Int::U16,
                    extend: ExtendKind::ZeroExtend,
                }
            }
            V1Node::MRefToRawPtr(inner) => {
                let raw = Self::from_v1(inner, asm);
                CILNode::RefToPtr(asm.node_idx(raw))
            }
            V1Node::Call(callargs) => {
                let args: Box<[_]> = callargs
                    .args
                    .iter()
                    .map(|arg| {
                        let node = Self::from_v1(arg, asm);
                        asm.node_idx(node)
                    })
                    .collect();
                let sig = FnSig::from_v1(callargs.site.signature(), asm);
                let sig = asm.sig_idx(sig);
                let generics: Box<[_]> = callargs
                    .site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = callargs.site.class().map(|dt| {
                    let cref = ClassRef::from_v1(dt, asm);
                    asm.class_idx(cref)
                });
                let name = asm.alloc_string(callargs.site.name());
                let method_ref = if callargs.site.is_static() {
                    MethodRef::new(class, name, sig, MethodKind::Static, generics)
                } else {
                    MethodRef::new(class, name, sig, MethodKind::Instance, generics)
                };
                let method_ref = asm.methodref_idx(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            V1Node::CallVirt(callargs) => {
                let args: Box<[_]> = callargs
                    .args
                    .iter()
                    .map(|arg| {
                        let node = Self::from_v1(arg, asm);
                        asm.node_idx(node)
                    })
                    .collect();
                let sig = FnSig::from_v1(callargs.site.signature(), asm);
                let sig = asm.sig_idx(sig);
                let generics: Box<[_]> = callargs
                    .site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = callargs.site.class().map(|dt| {
                    let cref = ClassRef::from_v1(dt, asm);
                    asm.class_idx(cref)
                });
                let name = asm.alloc_string(callargs.site.name());
                assert!(!callargs.site.is_static());
                let method_ref = MethodRef::new(class, name, sig, MethodKind::Virtual, generics);
                let method_ref = asm.methodref_idx(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            V1Node::NewObj(callargs) => {
                let args: Box<[_]> = callargs
                    .args
                    .iter()
                    .map(|arg| {
                        let node = Self::from_v1(arg, asm);
                        asm.node_idx(node)
                    })
                    .collect();
                let sig = FnSig::from_v1(callargs.site.signature(), asm);
                let sig = asm.sig_idx(sig);
                let generics: Box<[_]> = callargs
                    .site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = callargs.site.class().map(|dt| {
                    let cref = ClassRef::from_v1(dt, asm);
                    asm.class_idx(cref)
                });
                let name = asm.alloc_string(callargs.site.name());
                assert!(!callargs.site.is_static());
                let method_ref =
                    MethodRef::new(class, name, sig, MethodKind::Constructor, generics);
                let method_ref = asm.methodref_idx(method_ref);
                Self::Call(Box::new((method_ref, args)))
            }
            //V1Node::NewObj(call)
            V1Node::LdStr(string) => {
                let string = asm.alloc_string(string.clone());
                Const::PlatformString(string).into()
            }
            V1Node::LdcU64(val) => Const::U64(*val).into(),
            V1Node::LdcU32(val) => Const::U32(*val).into(),
            V1Node::LdcU16(val) => Const::U16(*val).into(),
            V1Node::LdcU8(val) => Const::U8(*val).into(),
            _ => todo!("v1:{v1:?}"),
        }
    }
}
