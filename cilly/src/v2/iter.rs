use std::fmt::Debug;

use super::cilroot::BranchCond;

use super::{Assembly, CILNode, CILRoot, Type};
#[derive(Hash, PartialEq, Eq, Clone, Debug)]
pub enum CILIterElem {
    Node(CILNode),
    Root(CILRoot),
}

impl CILIterElem {
    pub fn as_node(self) -> Option<CILNode> {
        if let Self::Node(v) = self {
            Some(v)
        } else {
            None
        }
    }
}
impl From<CILRoot> for CILIterElem {
    fn from(v: CILRoot) -> Self {
        Self::Root(v)
    }
}
impl From<CILNode> for CILIterElem {
    fn from(v: CILNode) -> Self {
        Self::Node(v)
    }
}
pub struct CILIter<'asm> {
    elems: Vec<(CILIterElem, usize)>,
    asm: &'asm Assembly,
}

impl<'asm> CILIter<'asm> {
    pub fn new(elems: impl Into<CILIterElem>, asm: &'asm Assembly) -> Self {
        Self {
            elems: vec![(elems.into(), 0)],
            asm,
        }
    }
}
impl<'asm> Iterator for CILIter<'asm> {
    type Item = CILIterElem;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (elem, idx) = self.elems.iter_mut().last()?;
            if *idx == 0 {
                *idx += 1;
                return Some(elem.clone());
            }
            match elem {
                CILIterElem::Root(CILRoot::SetField(fld)) => match idx {
                    1 => {
                        *idx += 1;
                        let lhs = self.asm.get_node(fld.1);
                        self.elems.push((CILIterElem::Node(lhs.clone()), 0));
                        continue;
                    }
                    2 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(fld.2);
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Root(CILRoot::StInd(ind)) => match idx {
                    1 => {
                        *idx += 1;
                        let lhs = self.asm.get_node(ind.0);
                        self.elems.push((CILIterElem::Node(lhs.clone()), 0));
                        continue;
                    }
                    2 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(ind.1);
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Node(
                    CILNode::BinOp(lhs, rhs, _)
                    | CILNode::LdElelemRef {
                        array: lhs,
                        index: rhs,
                    },
                ) => match idx {
                    1 => {
                        *idx += 1;
                        let lhs = self.asm.get_node(*lhs);
                        self.elems.push((CILIterElem::Node(lhs.clone()), 0));
                        continue;
                    }
                    2 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(*rhs);
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Node(CILNode::Call(info)) | CILIterElem::Root(CILRoot::Call(info)) => {
                    if *idx - 1 < info.1.len() {
                        let arg = &info.1[*idx - 1];
                        let arg = self.asm.get_node(*arg);
                        *idx += 1;
                        self.elems.push((CILIterElem::Node(arg.clone()), 0));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(CILNode::CallI(info))
                | CILIterElem::Root(CILRoot::CallI(info)) => match (*idx - 1).cmp(&info.2.len()) {
                    std::cmp::Ordering::Less => {
                        let arg = &info.2[*idx - 1];
                        let arg = self.asm.get_node(*arg);
                        *idx += 1;
                        self.elems.push((CILIterElem::Node(arg.clone()), 0));
                        continue;
                    }
                    std::cmp::Ordering::Equal => {
                        let arg = self.asm.get_node(info.0);
                        *idx += 1;
                        self.elems.push((CILIterElem::Node(arg.clone()), 0));
                    }
                    std::cmp::Ordering::Greater => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Node(
                    CILNode::UnOp(val, _)
                    | CILNode::PtrCast(val, _)
                    | CILNode::LdLen(val)
                    | CILNode::RefToPtr(val)
                    | CILNode::IntCast { input: val, .. }
                    | CILNode::FloatCast { input: val, .. }
                    | CILNode::LdField { addr: val, .. }
                    | CILNode::LdFieldAdress { addr: val, .. }
                    | CILNode::LdInd { addr: val, .. }
                    | CILNode::IsInst(val, _)
                    | CILNode::CheckedCast(val, _)
                    | CILNode::LocAlloc { size: val }
                    | CILNode::UnboxAny { object: val, .. },
                )
                | CILIterElem::Root(
                    CILRoot::StLoc(_, val)
                    | CILRoot::StArg(_, val)
                    | CILRoot::Ret(val)
                    | CILRoot::Pop(val)
                    | CILRoot::Throw(val)
                    | CILRoot::SetStaticField { val, .. },
                ) => match idx {
                    1 => {
                        *idx += 1;
                        let val = self.asm.get_node(*val);
                        self.elems.push((CILIterElem::Node(val.clone()), 0));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Node(
                    CILNode::Const(_)
                    | CILNode::LdArg(_)
                    | CILNode::LdLoc(_)
                    | CILNode::LdArgA(_)
                    | CILNode::LdLocA(_)
                    | CILNode::SizeOf(_)
                    | CILNode::LdStaticField(_)
                    | CILNode::LdFtn(_)
                    | CILNode::LdTypeToken(_)
                    | CILNode::LocAllocAlgined { .. }
                    | CILNode::GetException,
                )
                | CILIterElem::Root(
                    CILRoot::VoidRet
                    | CILRoot::Break
                    | CILRoot::SourceFileInfo { .. }
                    | CILRoot::ExitSpecialRegion { .. }
                    | CILRoot::Nop
                    | CILRoot::ReThrow,
                ) => {
                    self.elems.pop();
                }
                CILIterElem::Root(CILRoot::InitBlk(blk) | CILRoot::CpBlk(blk)) => match idx {
                    1 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(blk.0);
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    2 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(blk.1);
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    3 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(blk.2);
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Root(CILRoot::Branch(packed)) => {
                    let (_, _, cond) = packed.as_ref();
                    let Some(cond) = cond else {
                        self.elems.pop();
                        continue;
                    };
                    match cond {
                        BranchCond::True(cond) | BranchCond::False(cond) => match idx {
                            1 => {
                                *idx += 1;
                                let val = self.asm.get_node(*cond);
                                self.elems.push((CILIterElem::Node(val.clone()), 0));
                                continue;
                            }
                            _ => {
                                self.elems.pop();
                                continue;
                            }
                        },
                        BranchCond::Eq(lhs, rhs)
                        | BranchCond::Ne(lhs, rhs)
                        | BranchCond::Lt(lhs, rhs, _)
                        | BranchCond::Gt(lhs, rhs, _)
                        | BranchCond::Le(lhs, rhs, _)
                        | BranchCond::Ge(lhs, rhs, _) => match idx {
                            1 => {
                                *idx += 1;
                                let rhs = self.asm.get_node(*rhs);
                                self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                                continue;
                            }
                            2 => {
                                *idx += 1;
                                let lhs = self.asm.get_node(*lhs);
                                self.elems.push((CILIterElem::Node(lhs.clone()), 0));
                                continue;
                            }
                            _ => {
                                self.elems.pop();
                                continue;
                            }
                        },
                    }
                }
            }
        }
    }
}

pub struct CILIterMut<'start> {
    start: Either<&'start mut CILNode, &'start mut CILRoot>,
    idx: u32,
    elems: Vec<(CILIterElem, usize)>,
    asm: &'start mut Assembly,
}
#[derive(Debug)]
pub enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> From<A> for Either<A, B> {
    fn from(v: A) -> Self {
        Self::A(v)
    }
}

pub trait CILIterMutTrait {
    type Ctx;
    type A: Debug;
    type B: Debug;
    fn advance(&mut self);
    #[allow(clippy::type_complexity)]
    fn get(&mut self) -> Option<(&mut Self::Ctx, Either<&mut Self::A, &mut Self::B>)>;
    #[allow(clippy::type_complexity)]
    fn next(&mut self) -> Option<(&mut Self::Ctx, Either<&mut Self::A, &mut Self::B>)> {
        self.advance();
        let got = self.get();
        /*match &got {
            Some((_, got)) => eprintln!("got:{got:?}"),
            _ => eprintln!("got:None"),
        }*/
        got
    }
}
impl<'start> CILIterMutTrait for CILIterMut<'start> {
    type Ctx = Assembly;

    type A = CILNode;
    type B = CILRoot;

    fn advance(&mut self) {
        let mut curr: Option<CILIterElem> = None;
        'main: loop {
            if self.elems.is_empty() {
                if self.idx == u32::MAX {
                    self.idx = 0;
                    return;
                } else {
                    match &mut self.start {
                        Either::A(CILNode::BinOp(lhs, rhs, _)) => match self.idx {
                            0 => {
                                let lhs = self.asm.get_node(*lhs);
                                self.elems.push((CILIterElem::Node(lhs.clone()), 0));
                                continue 'main;
                            }
                            1 => {
                                let curr = curr.take().expect("Iterator error").as_node().unwrap();
                                *lhs = self.asm.alloc_node(curr);

                                let rhs = self.asm.get_node(*rhs);
                                self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                                continue 'main;
                            }
                            2 => {
                                let curr = curr.take().expect("Iterator error").as_node().unwrap();
                                *rhs = self.asm.alloc_node(curr);
                                self.idx += 1;
                                return;
                            }
                            _ => return,
                        },
                        Either::A(node) => todo!("{node:?}"),
                        Either::B(root) => todo!("{root:?}"),
                    }
                }
            } else {
                let (elem, idx) = self.elems.iter_mut().last().unwrap();
                if *idx == 0 {
                    *idx += 1;
                    return;
                }
                match elem {
                    CILIterElem::Node(CILNode::Const(_)) => {
                        assert!(curr.is_none());
                        curr = Some(self.elems.pop().unwrap().0);
                        if self.elems.is_empty() {
                            self.idx += 1;
                        } else {
                            let (_, idx) = self.elems.iter_mut().last().unwrap();
                            *idx += 1;
                        }
                        continue 'main;
                    }
                    CILIterElem::Node(_) => todo!(),
                    CILIterElem::Root(_) => todo!(),
                }
            }
        }
    }

    fn get(&mut self) -> Option<(&mut Self::Ctx, Either<&mut Self::A, &mut Self::B>)> {
        {
            if self.elems.is_empty() {
                if self.idx == 0 {
                    Some((
                        self.asm,
                        match &mut self.start {
                            Either::A(a) => Either::A(a),
                            Either::B(b) => Either::B(b),
                        },
                    ))
                } else {
                    None
                }
            } else {
                let (elem, idx) = self.elems.iter_mut().last()?;
                if *idx == 1 {
                    return Some((
                        self.asm,
                        match elem {
                            CILIterElem::Node(node) => Either::A(node),
                            CILIterElem::Root(root) => Either::B(root),
                        },
                    ));
                }
                match elem {
                    //CILIterElem::Node(CILNode::Const(_)) =>
                    CILIterElem::Node(node) => todo!("node:{node:?}"),
                    CILIterElem::Root(root) => todo!("root:{root:?}"),
                }
            }
        }
    }
}
impl<'start> CILIterMut<'start> {
    pub fn new<'asm: 'start>(
        start: impl Into<Either<&'start mut CILNode, &'start mut CILRoot>>,
        asm: &'asm mut Assembly,
    ) -> Self {
        Self {
            start: start.into(),
            idx: u32::MAX,
            elems: vec![],
            asm,
        }
    }
}
pub(crate) trait TpeIter<'this>: Sized + 'this {
    fn iter_types<'asm: 'this>(self, asm: &'asm Assembly) -> impl Iterator<Item = Type> + 'this;
}
impl<'this, T: Iterator<Item = CILIterElem> + 'this> TpeIter<'this> for T {
    fn iter_types<'asm: 'this>(self, asm: &'asm Assembly) -> impl Iterator<Item = Type> + 'this {
        let this = self;
        this.filter_map(|cil_item| {
            let iter: Option<Box<dyn Iterator<Item = Type>>> = match cil_item {
                crate::v2::CILIterElem::Node(node) => match node {
                    CILNode::Const(_)
                    | CILNode::BinOp(_, _, _)
                    | CILNode::UnOp(_, _)
                    | CILNode::LdLoc(_)
                    | CILNode::LdLocA(_)
                    | CILNode::LdArg(_)
                    | CILNode::LdArgA(_)
                    | CILNode::IntCast { .. }
                    | CILNode::FloatCast { .. }
                    | CILNode::RefToPtr(_)
                    | CILNode::GetException
                    | CILNode::LocAlloc { .. }
                    | CILNode::LdLen(_)
                    | CILNode::LdElelemRef { .. } => None,
                    // Since this method is called, then if it uses an "internal" type, we must assume it is defined in this module. Thus, its types are already included, and we don't need to include them again.
                    CILNode::Call(_) | CILNode::LdFtn(_) => None,
                    CILNode::PtrCast(_, res) => match res.as_ref() {
                        crate::v2::cilnode::PtrCastRes::Ptr(inner) => {
                            Some(Box::new(std::iter::once(asm.get_type(*inner)).copied()))
                        }
                        crate::v2::cilnode::PtrCastRes::Ref(inner) => {
                            Some(Box::new(std::iter::once(asm.get_type(*inner)).copied()))
                        }
                        crate::v2::cilnode::PtrCastRes::FnPtr(sig) => {
                            Some(Box::new(asm.get_sig(*sig).iter_types()))
                        }
                        crate::v2::cilnode::PtrCastRes::USize => None,
                        crate::v2::cilnode::PtrCastRes::ISize => None,
                    },
                    CILNode::LdFieldAdress { field, .. } | CILNode::LdField { field, .. } => {
                        let field = asm.get_field(field);
                        let class = Type::ClassRef(field.owner());
                        let tpe = field.tpe();
                        Some(Box::new([class, tpe].into_iter()))
                    }
                    CILNode::LdInd { tpe, .. } => {
                        Some(Box::new(std::iter::once(asm.get_type(tpe)).copied()))
                    }
                    CILNode::SizeOf(tpe)
                    | CILNode::IsInst(_, tpe)
                    | CILNode::CheckedCast(_, tpe)
                    | CILNode::LdTypeToken(tpe)
                    | CILNode::UnboxAny { tpe, .. }
                    | CILNode::LocAllocAlgined { tpe, .. } => {
                        Some(Box::new(std::iter::once(asm.get_type(tpe)).copied()))
                    }
                    CILNode::CallI(info) => Some(Box::new(asm.get_sig(info.1).iter_types())),
                    CILNode::LdStaticField(sfld) => {
                        let field = asm.get_static_field(sfld);
                        let class = Type::ClassRef(field.owner());
                        let tpe = field.tpe();
                        Some(Box::new([class, tpe].into_iter()))
                    }
                },
                crate::v2::CILIterElem::Root(root) => match root {
                    CILRoot::StLoc(_, _)
                    | CILRoot::StArg(_, _)
                    | CILRoot::Ret(_)
                    | CILRoot::Pop(_)
                    | CILRoot::Throw(_)
                    | CILRoot::VoidRet
                    | CILRoot::Break
                    | CILRoot::Nop
                    | CILRoot::Branch(_)
                    | CILRoot::SourceFileInfo { .. }
                    | CILRoot::ExitSpecialRegion { .. }
                    | CILRoot::InitBlk(_)
                    | CILRoot::CpBlk(_)
                    | CILRoot::ReThrow => None,
                    CILRoot::SetStaticField { field, .. } => {
                        let field = asm.get_static_field(field);
                        let class = Type::ClassRef(field.owner());
                        let tpe = field.tpe();
                        Some(Box::new([class, tpe].into_iter()))
                    }
                    CILRoot::SetField(info) => {
                        let field = asm.get_field(info.0);
                        let class = Type::ClassRef(field.owner());
                        let tpe = field.tpe();
                        Some(Box::new([class, tpe].into_iter()))
                    }
                    // Since this method is called, then if it uses an "internal" type, we must assume it is defined in this module. Thus, its types are already included, and we don't need to include them again.
                    CILRoot::Call(_) | CILRoot::CallI(_) => None,
                    CILRoot::StInd(info) => Some(Box::new(std::iter::once(info.2))),
                },
            };
            iter
        })
        .flatten()
    }
}
#[test]
pub fn nodes() {
    use super::{BinOp, Const};
    let mut asm = Assembly::default();
    let mut add = asm.biop(Const::I8(2), Const::I8(1), BinOp::Add);
    let mut iter = CILIterMut::new(&mut add, &mut asm);
    assert!(matches!(
        iter.next(),
        Some((_, Either::A(CILNode::BinOp(_, _, BinOp::Add))))
    ));
    assert!(matches!(
        iter.next(),
        Some((_, Either::A(CILNode::Const(_))))
    ));
    assert!(matches!(
        iter.next(),
        Some((_, Either::A(CILNode::Const(_))))
    ));
    assert!(iter.next().is_none());
}
