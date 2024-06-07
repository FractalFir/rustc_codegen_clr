use crate::{call_site::CallSite, cil_node::CILNode, cil_root::CILRoot};

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
        let mut iter_count = 0;
        loop {
            iter_count += 1;
            assert!(iter_count < 100);
            let (idx, elem) = self.elems.iter_mut().last()?;
            if *idx == 0 {
                *idx += 1;
                return Some(*elem);
            }
            match elem {
                CILIterElem::Node(CILNode::CreateDelegate { obj, site: _ }) => {
                    if idx == &1 {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(obj)));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(
                    CILNode::Add(a, b)
                    | CILNode::And(a, b)
                    | CILNode::Div(a, b)
                    | CILNode::DivUn(a, b)
                    | CILNode::Rem(a, b)
                    | CILNode::RemUn(a, b)
                    | CILNode::Mul(a, b)
                    | CILNode::Eq(a, b)
                    | CILNode::LtUn(a, b)
                    | CILNode::Lt(a, b)
                    | CILNode::GtUn(a, b)
                    | CILNode::Gt(a, b)
                    | CILNode::Or(a, b)
                    | CILNode::Sub(a, b)
                    | CILNode::Shl(a, b)
                    | CILNode::Shr(a, b)
                    | CILNode::ShrUn(a, b)
                    | CILNode::XOr(a, b)
                    | CILNode::LDElelemRef { arr: a, idx: b },
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
                    | CILNode::ConvF64Un(a)
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
                    | CILNode::ZeroExtendToISize(a)
                    | CILNode::LDFieldAdress { addr: a, field: _ }
                    | CILNode::LDField { addr: a, field: _ }
                    | CILNode::TransmutePtr { val: a, .. }
                    | CILNode::LDIndI8 { ptr: a }
                    | CILNode::LDIndU8 { ptr: a }
                    | CILNode::LDIndI16 { ptr: a }
                    | CILNode::LDIndU16 { ptr: a }
                    | CILNode::LDIndI32 { ptr: a }
                    | CILNode::LDIndU32 { ptr: a }
                    | CILNode::LDIndI64 { ptr: a }
                    | CILNode::LDIndU64 { ptr: a }
                    | CILNode::LDIndBool { ptr: a }
                    | CILNode::LDIndF32 { ptr: a }
                    | CILNode::LDIndF64 { ptr: a }
                    | CILNode::LdObj { ptr: a, obj: _ }
                    | CILNode::LDIndPtr {
                        ptr: a,
                        loaded_ptr: _,
                    }
                    | CILNode::LDIndISize { ptr: a }
                    | CILNode::LDIndUSize { ptr: a }
                    | CILNode::Not(a)
                    | CILNode::Neg(a)
                    | CILNode::LDLen { arr: a }
                    | CILNode::BlackBox(a),
                ) => {
                    if idx == &1 {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(a)));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Root(CILRoot::Pop { tree: a }) => {
                    if idx == &1 {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(a)));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(
                    CILNode::LDLoc(_)
                    | CILNode::LDLocA(_)
                    | CILNode::LDArg(_)
                    | CILNode::LDArgA(_)
                    | CILNode::SizeOf(_)
                    | CILNode::LdcI32(_)
                    | CILNode::LdcF32(_)
                    | CILNode::LdcI64(_)
                    | CILNode::LdcF64(_)
                    | CILNode::LdcU32(_)
                    | CILNode::LdcU64(_)
                    | CILNode::LdStr(_)
                    | CILNode::LdFalse
                    | CILNode::LdTrue
                    | CILNode::LDStaticField(_)
                    | CILNode::LDFtn(_)
                    | CILNode::LDTypeToken(_)
                    | CILNode::LocAllocAligned { tpe: _, align: _ }
                    | CILNode::LoadGlobalAllocPtr { alloc_id: _ }
                    | CILNode::LoadAddresOfTMPLocal
                    | CILNode::PointerToConstValue(_)
                    | CILNode::LoadTMPLocal
                    | CILNode::GetStackTop,
                ) => {
                    self.elems.pop();
                    continue;
                }
                CILIterElem::Root(
                    CILRoot::STLoc { tree, local: _ }
                    | CILRoot::SetTMPLocal { value: tree }
                    | CILRoot::STArg { tree, arg: _ }
                    | CILRoot::Ret { tree }
                    | CILRoot::BTrue { cond: tree, .. }
                    | CILRoot::BFalse { cond: tree, .. }
                    | CILRoot::Throw(tree)
                    | CILRoot::SetStaticField {
                        descr: _,
                        value: tree,
                    },
                ) => {
                    if idx == &1 {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(tree)));
                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Root(
                    CILRoot::SetField {
                        addr: a, value: b, ..
                    }
                    | CILRoot::STIndI8(a, b)
                    | CILRoot::STIndI16(a, b)
                    | CILRoot::STIndI32(a, b)
                    | CILRoot::STIndI64(a, b)
                    | CILRoot::STIndISize(a, b)
                    | CILRoot::STIndF32(a, b)
                    | CILRoot::STIndF64(a, b)
                    | CILRoot::STObj {
                        tpe: _,
                        addr_calc: a,
                        value_calc: b,
                    },
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
                CILIterElem::Root(
                    CILRoot::SourceFileInfo(_)
                    | CILRoot::GoTo { .. }
                    | CILRoot::VoidRet
                    | CILRoot::Break
                    | CILRoot::Nop
                    | CILRoot::ReThrow
                    | CILRoot::JumpingPad { .. },
                ) => {
                    self.elems.pop();
                    continue;
                }
                CILIterElem::Node(CILNode::CallI(fn_sig_and_args)) => {
                    if *idx - 1 < fn_sig_and_args.2.len() {
                        let arg = &fn_sig_and_args.2[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(arg)));
                        continue;
                    } else if *idx - 1 < fn_sig_and_args.2.len() + 1 {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(&fn_sig_and_args.1)));
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }

                CILIterElem::Root(
                    CILRoot::Call { site: _, args } | CILRoot::CallVirt { site: _, args },
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
                CILIterElem::Node(
                    CILNode::Call { site: _, args }
                    | CILNode::NewObj { site: _, args }
                    | CILNode::CallVirt { site: _, args },
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
                CILIterElem::Node(CILNode::SubTrees(roots, node)) => {
                    if *idx - 1 < roots.len() {
                        let root: &CILRoot = &roots[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Root(root)));

                        continue;
                    } else if *idx - 1 == roots.len() {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(node)));

                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(CILNode::InspectValue { val, inspect }) => {
                    if *idx - 1 < inspect.len() {
                        let root = &inspect[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Root(root)));

                        continue;
                    } else if *idx - 1 == inspect.len() {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(val)));

                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Node(CILNode::TemporaryLocal(pack)) => {
                    let (_, roots, node) = pack.as_ref();
                    if *idx - 1 < roots.len() {
                        let root: &CILRoot = &roots[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Root(root)));

                        continue;
                    } else if *idx - 1 == roots.len() {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(node)));

                        continue;
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Root(CILRoot::CallI {
                    sig: _,
                    args,
                    fn_ptr,
                }) => {
                    if *idx - 1 < args.len() {
                        let arg = &args[*idx - 1];
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(arg)));
                        continue;
                    } else if *idx - 1 < args.len() + 1 {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(fn_ptr)));
                    } else {
                        self.elems.pop();
                        continue;
                    }
                }
                CILIterElem::Root(
                    CILRoot::CpBlk {
                        dst: a,
                        src: b,
                        len: c,
                    }
                    | CILRoot::InitBlk {
                        dst: a,
                        val: b,
                        count: c,
                    },
                ) => match *idx {
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
                    3 => {
                        *idx += 1;
                        self.elems.push((0, CILIterElem::Node(c)));
                        continue;
                    }
                    _ => {
                        self.elems.pop();
                        continue;
                    }
                },
                CILIterElem::Root(
                    CILRoot::BEq {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BNe {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BLt {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BLtUn {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BGt {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BGtUn {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BLe {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    }
                    | CILRoot::BGe {
                        target: _,
                        sub_target: _,
                        a,
                        b,
                    },
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
                //_ => todo!("Unhandled iter elem {elem:?}"),
            }
        }
    }
}
impl<'a> CILIter<'a> {
    #[must_use]
    pub fn new_node(node: &'a CILNode) -> Self {
        Self {
            elems: vec![(0, CILIterElem::Node(node))],
        }
    }
    #[must_use]
    pub fn new_root(root: &'a CILRoot) -> Self {
        Self {
            elems: vec![(0, CILIterElem::Root(root))],
        }
    }
}
pub trait CILIterTrait<'a> {
    fn call_sites(self) -> impl Iterator<Item = &'a CallSite>;
}
impl<'a, T: Iterator<Item = CILIterElem<'a>>> CILIterTrait<'a> for T {
    fn call_sites(self) -> impl Iterator<Item = &'a CallSite> {
        self.filter_map(|node| match node {
            CILIterElem::Node(
                CILNode::Call { args: _, site }
                | CILNode::CallVirt { args: _, site }
                | CILNode::NewObj { args: _, site }
                | CILNode::LDFtn(site),
            ) => Some(site.as_ref()),
            CILIterElem::Root(
                CILRoot::Call { site, args: _ } | CILRoot::CallVirt { site, args: _ },
            ) => Some(site),
            _ => None,
        })
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
