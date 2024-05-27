use crate::{cil_node::CILNode, cil_root::CILRoot};

#[derive(Debug, Clone, Copy)]
pub enum CILIterElem<'a> {
    Node(&'a CILNode),
    Root(&'a CILRoot),
}
pub struct CILIter<'a> {
    elems: Vec<(usize, CILIterElem<'a>)>,
}
impl<'a> Iterator for CILIter<'a> {
    type Item = CILIterElem<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (idx, elem) = self.elems.iter_mut().last()?;
            if *idx == 0 {
                *idx += 1;
                return Some(*elem);
            }
            match elem {
                CILIterElem::Node(
                    CILNode::Add(a, b)
                    | CILNode::Mul(a, b)
                    | CILNode::Eq(a, b)
                    | CILNode::LtUn(a, b)
                    | CILNode::Or(a, b)
                    | CILNode::Sub(a, b),
                ) => match idx {
                    1 => {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(a)));
                        continue;
                    }
                    2 => {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(b)));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Node(
                    CILNode::ConvU64(a)
                    | CILNode::ConvI64(a)
                    | CILNode::ConvF64(a)
                    | CILNode::ConvU32(a)
                    | CILNode::ConvI32(a)
                    | CILNode::ConvF32(a)
                    | CILNode::ConvISize(a)
                    | CILNode::MRefToRawPtr(a)
                    | CILNode::ConvU16(a)
                    | CILNode::ConvI16(a)
                    | CILNode::ConvU8(a)
                    | CILNode::ConvI8(a)
                    | CILNode::ZeroExtendToUSize(a)
                    | CILNode::TransmutePtr { val: a, .. },
                ) => if idx == &1 {
                    *idx += 1;
                    self.elems.push((0, CILIterElem::Node(a)));
                    continue;
                } else {
                    self.elems.pop();
                    continue;
                },
                CILIterElem::Node(
                    CILNode::LDLoc(_)
                    | CILNode::LDLocA(_)
                    | CILNode::SizeOf(_)
                    | CILNode::LdcI32(_)
                    | CILNode::LdcF32(_)
                    | CILNode::LdcI64(_)
                    | CILNode::LdcF64(_)
                    | CILNode::LdcU32(_)
                    | CILNode::LdcU64(_)
                    | CILNode::LdStr(_)
                    | CILNode::LdFalse
                    | CILNode::LdTrue,
                ) => {
                    self.elems.pop();
                    continue;
                }
                CILIterElem::Root(
                    CILRoot::STLoc { tree, .. }
                    | CILRoot::Ret { tree }
                    | CILRoot::BTrue { cond: tree, .. }
                    | CILRoot::Throw(tree),
                ) => if idx == &1 {
                    *idx += 1;
                    self.elems.push((0, CILIterElem::Node(tree)));
                    continue;
                } else {
                    self.elems.pop();
                    continue;
                },
                CILIterElem::Root(
                    CILRoot::SourceFileInfo(_)
                    | CILRoot::GoTo { .. }
                    | CILRoot::VoidRet
                    | CILRoot::Break,
                ) => {
                    self.elems.pop();
                    continue;
                }
                CILIterElem::Root(CILRoot::Call { site: _, args }) => {
                    if *idx - 1 < args.len() {
                        let arg = &args[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(arg)));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(
                    CILNode::Call { site: _, args } | CILNode::NewObj { site: _, args },
                ) => {
                    if *idx - 1 < args.len() {
                        let arg = &args[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(arg)));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }

                _ => todo!("Unhandled iter elem {elem:?}"),
            }
        }
    }
}
impl<'a> CILIter<'a> {
    #[must_use] pub fn new_node(node: &'a CILNode) -> Self {
        Self {
            elems: vec![(0, CILIterElem::Node(node))],
        }
    }
    #[must_use] pub fn new_root(root: &'a CILRoot) -> Self {
        Self {
            elems: vec![(0, CILIterElem::Root(root))],
        }
    }
}
impl<'a> IntoIterator for &'a CILNode {
    type Item = CILIterElem<'a>;

    type IntoIter = CILIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CILIter::new_node(self)
    }
}
impl<'a> IntoIterator for &'a CILRoot {
    type Item = CILIterElem<'a>;

    type IntoIter = CILIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        CILIter::new_root(self)
    }
}
#[test]
fn iter() {
    use crate::{call_site::CallSite, FnSig, Type};
    let node = CILNode::Add(
        Box::new(CILNode::Mul(
            Box::new(CILNode::LDLoc(0)),
            Box::new(CILNode::SizeOf(Box::new(Type::U8))),
        )),
        Box::new(CILNode::LDLoc(1)),
    );
    let mut iter = node.into_iter();
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Add(_, _)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Mul(_, _)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::LDLoc(_)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::SizeOf(_)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::LDLoc(1)))
    ));
    assert!(matches!(iter.next(), None));
    let root = CILRoot::Call {
        site: CallSite::new(
            None,
            "bob".into(),
            FnSig::new(&[Type::I32, Type::F32], Type::Void),
            true,
        ),
        args: [CILNode::LdcI32(-77), CILNode::LdcF32(3.14159)].into(),
    };
    let mut iter = root.into_iter();
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Root(CILRoot::Call { .. }))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::LdcI32(-77)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::LdcF32(3.14159)))
    ));
    assert!(matches!(iter.next(), None));
}
