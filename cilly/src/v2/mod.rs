use std::path::Path;

pub use access::Access;
pub use asm::{Assembly, IlasmFlavour};
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
pub mod asm_link;
pub mod basic_block;
pub mod bimap;
pub mod c_exporter;
pub mod cilnode;
pub mod cilroot;
pub mod class;
pub mod cst;
pub mod field;
pub mod float;
pub mod fnsig;
/// Defines hashable and equable floating point types. All NaNs are compared by bits, and -0.0 != 0.0.
pub mod hashable;
pub mod il_exporter;
pub mod int;
pub mod iter;
pub mod java_exporter;
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
        Some(CILIterElem::Node(CILNode::Const(_)))
    ));
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Const(_)))
    ));
    assert!(iter.next().is_none());
    let msg = asm.ldstr("Hi!");
    let mut iter = CILIter::new(msg, &asm);
    assert!(matches!(
        iter.next(),
        Some(CILIterElem::Node(CILNode::Const(_)))
    ));
    assert!(iter.next().is_none());
}
pub trait Exporter {
    type Error: std::fmt::Debug;
    fn export(&self, asm: &Assembly, target: &Path) -> Result<(), Self::Error>;
}

#[test]
fn no_collision() {
    let mut asm = Assembly::default();
    let mut curr: CILNode = Const::I8(1).into();
    for _ in 0..100_000 {
        curr = std::hint::black_box(asm.biop(curr.clone(), curr, BinOp::Add));
    }
    asm.alloc_node(CILNode::LdLoc(0));
    asm.alloc_node(Const::I32(0));
    asm.alloc_node(Const::I64(0));
}
#[test]
fn test_binops() {
    fn test_binop(asm: &mut Assembly, op: BinOp) -> CILNode {
        let mut curr: CILNode = Const::I8(1).into();
        for _ in 0..10 {
            curr = std::hint::black_box(asm.biop(curr.clone(), curr, op));
        }
        curr.get_type(asm.sig(vec![], Type::Void), &[], asm)
            .unwrap();
        curr
    }
    let mut asm = Assembly::default();
    test_binop(&mut asm, BinOp::Add);
    test_binop(&mut asm, BinOp::Sub);
    test_binop(&mut asm, BinOp::Mul);
    test_binop(&mut asm, BinOp::Rem);
}
