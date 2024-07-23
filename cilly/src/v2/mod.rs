pub use access::Access;
pub use asm::Assembly;
pub use basic_block::BasicBlock;
pub use bimap::BiMap;
pub use cilnode::{BinOp, CILNode, NodeIdx};
pub use cilroot::{CILRoot, RootIdx};
pub use class::{ClassDef, ClassDefIdx, ClassRef, ClassRefIdx};
pub use cst::Const;
pub use field::{FieldDesc, FieldIdx, StaticFieldDesc, StaticFieldIdx};
pub use float::Float;
pub use fnsig::{FnSig, SigIdx};
pub use int::Int;
pub use iter::{CILIter, CILIterElem};
pub use method::{MethodDef, MethodDefIdx, MethodImpl, MethodRef, MethodRefIdx};
pub use strings::StringIdx;
pub use tpe::{Type, TypeIdx};

pub mod access;
pub mod asm;
pub mod basic_block;
pub mod bimap;
pub mod cilnode;
pub mod cilroot;
pub mod class;
pub mod cst;
pub mod field;
pub mod float;
pub mod fnsig;
pub mod int;
pub mod iter;
pub mod method;
pub mod strings;
pub mod tpe;
#[test]
fn types() {
    let mut asm = Assembly::default();
    let tpe = asm.nptr(Type::Int(Int::U8));
    let tpe = asm.nref(tpe);
    assert_eq!(*tpe.deref(&asm).deref(&asm), Type::Int(Int::U8));
}
#[test]
pub fn nodes() {
    let mut asm = Assembly::default();
    let add = asm.biop(Const::I8(2), Const::I8(1), BinOp::Add);
    let mut iter = CILIter::new(add, &asm);
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::BinOp(_, _, BinOp::Add)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Const(Const::I8(2))))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Const(Const::I8(1))))
    ));
    assert!(iter.next().is_none());
    let msg = asm.ldstr("Hi!");
    let mut iter = CILIter::new(msg, &asm);
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Const(Const::PlatformString(_))))
    ));
    assert!(iter.next().is_none());
}
#[test]
fn no_collision() {
    let mut asm = Assembly::default();
    let mut curr = CILNode::Const(Const::I8(1));
    for _ in 0..100_000 {
        curr = std::hint::black_box(asm.biop(curr.clone(), curr, BinOp::Add));
    }
    asm.node_idx(CILNode::LdLoc(0));
    asm.node_idx(CILNode::Const(Const::I32(0)));
    asm.node_idx(CILNode::Const(Const::I64(0)));
}
