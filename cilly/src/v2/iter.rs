use std::fmt::Debug;

use streaming_iterator::StreamingIterator;

use crate::v2::{cilroot::BranchCond, BinOp, Const};

use super::{Assembly, CILNode, CILRoot};
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
                CILIterElem::Node(CILNode::BinOp(rhs, lhs, _)) => match idx {
                    1 => {
                        *idx += 1;
                        let rhs = self.asm.get_node(rhs.clone());
                        self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                        continue;
                    }
                    2 => {
                        *idx += 1;
                        let lhs = self.asm.get_node(lhs.clone());
                        self.elems.push((CILIterElem::Node(lhs.clone()), 0));
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
                        let arg = self.asm.get_node(arg.clone());
                        *idx += 1;
                        self.elems.push((CILIterElem::Node(arg.clone()), 0));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(
                    CILNode::UnOp(val, _)
                    | CILNode::PtrCast(val, _)
                    | CILNode::RefToPtr(val)
                    | CILNode::IntCast { input: val, .. },
                )
                | CILIterElem::Root(
                    CILRoot::StLoc(_, val)
                    | CILRoot::StArg(_, val)
                    | CILRoot::Ret(val)
                    | CILRoot::Throw(val),
                ) => match idx {
                    1 => {
                        *idx += 1;
                        let val = self.asm.get_node(val.clone());
                        self.elems.push((CILIterElem::Node(val.clone()), 0));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Node(CILNode::Const(_) | CILNode::LdArg(_) | CILNode::LdLoc(_))
                | CILIterElem::Root(
                    CILRoot::VoidRet | CILRoot::Break | CILRoot::SourceFileInfo { .. },
                ) => {
                    self.elems.pop();
                }
                CILIterElem::Root(CILRoot::Branch { cond, .. }) => {
                    let Some(cond) = cond else {
                        self.elems.pop();
                        continue;
                    };
                    match cond.as_ref() {
                        BranchCond::True(cond) | BranchCond::False(cond) => match idx {
                            1 => {
                                *idx += 1;
                                let val = self.asm.get_node(cond.clone());
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
                        | BranchCond::Gt(lhs, rhs, _) => match idx {
                            1 => {
                                *idx += 1;
                                let rhs = self.asm.get_node(rhs.clone());
                                self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                                continue;
                            }
                            2 => {
                                *idx += 1;
                                let lhs = self.asm.get_node(lhs.clone());
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
enum Either<A, B> {
    A(A),
    B(B),
}

impl<A, B> From<A> for Either<A, B> {
    fn from(v: A) -> Self {
        Self::A(v)
    }
}

trait LendingIter {
    type Ctx;
    type A: Debug;
    type B: Debug;
    fn advance(&mut self);
    fn get(&mut self) -> Option<(&mut Self::Ctx, Either<&mut Self::A, &mut Self::B>)>;
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
impl<'start> LendingIter for CILIterMut<'start> {
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
                        Either::A(CILNode::BinOp(lhs, rhs, op)) => match self.idx {
                            0 => {
                                let lhs = self.asm.get_node(lhs.clone());
                                self.elems.push((CILIterElem::Node(lhs.clone()), 0));
                                continue 'main;
                            }
                            1 => {
                                let curr = curr.take().expect("Iterator error").as_node().unwrap();
                                *lhs = self.asm.node_idx(curr);

                                let rhs = self.asm.get_node(rhs.clone());
                                self.elems.push((CILIterElem::Node(rhs.clone()), 0));
                                continue 'main;
                            }
                            2 => {
                                let curr = curr.take().expect("Iterator error").as_node().unwrap();
                                *rhs = self.asm.node_idx(curr);
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
                todo!();
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
                todo!();
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
#[test]
pub fn nodes() {
    let mut asm = Assembly::default();
    let mut add = asm.biop(Const::I8(2), Const::I8(1), BinOp::Add);
    let mut iter = CILIterMut::new(&mut add, &mut asm);
    assert!(matches!(
        iter.next(),
        Some((_, Either::A(CILNode::BinOp(_, _, BinOp::Add))))
    ));
    assert!(matches!(
        iter.next(),
        Some((_, Either::A(CILNode::Const(Const::I8(2)))))
    ));
    assert!(matches!(
        iter.next(),
        Some((_, Either::A(CILNode::Const(Const::I8(1)))))
    ));
    assert!(matches!(iter.next(), None));
}
