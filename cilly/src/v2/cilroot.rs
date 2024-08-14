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
    /// Field, value, addr
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
                    | BranchCond::Gt(lhs, rhs, _) => [lhs, rhs].into(),
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
            CILRoot::InitBlk(info) => {
                let (addr, val, len) = info.as_mut();
                [addr, val, len].into()
            }
            CILRoot::CpBlk(info) => {
                let (dst, src, len) = info.as_mut();
                [dst, src, len].into()
            }
            CILRoot::CallI(info) => {
                let (ptr, _, args) = info.as_mut();
                let mut args = many_mut(args);
                args.push(ptr);
                args.into()
            }
        }
    }
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
                let sig = FnSig::from_v1(site.signature(), asm);
                let sig = asm.alloc_sig(sig);
                let generics: Box<[_]> = site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = site
                    .class()
                    .map(|dt| {
                        let cref = ClassRef::from_v1(dt, asm);
                        asm.alloc_class_ref(cref)
                    })
                    .unwrap_or_else(|| *asm.main_module());
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
                let sig = FnSig::from_v1(site.signature(), asm);
                let sig = asm.alloc_sig(sig);
                let generics: Box<[_]> = site
                    .generics()
                    .iter()
                    .map(|gen| Type::from_v1(gen, asm))
                    .collect();
                let class = site
                    .class()
                    .map(|dt| {
                        let cref = ClassRef::from_v1(dt, asm);
                        asm.alloc_class_ref(cref)
                    })
                    .unwrap_or_else(|| *asm.main_module());
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
                let ptr = Type::from_v1(ptr, asm);
                let ptr = asm.nptr(ptr);
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
                let tpe = Type::from_v1(tpe, asm);
                Self::StInd(Box::new((addr_calc, value_calc, tpe, false)))
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
                let sig = FnSig::from_v1(sig, asm);
                let sig = asm.alloc_sig(sig);
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
                    field: asm.alloc_sfld(descr),
                    val: asm.alloc_node(val),
                }
            }
            _ => todo!("v1:{v1:?}"),
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
            let mut res = many_mut(lhs);
            res.extend(many_mut(rhs));
            res
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
