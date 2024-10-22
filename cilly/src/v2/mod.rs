//#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// This lint includes tests for some bizzare reason, so ignoring it seems like the best course of action
#![allow(clippy::missing_panics_doc)]
use std::path::Path;

pub use access::Access;
pub use asm::{Assembly, IlasmFlavour};
pub use basic_block::BasicBlock;
pub use bimap::BiMap;
pub use cilnode::{BinOp, CILNode, NodeIdx};
pub use cilroot::{BranchCond, CILRoot, RootIdx};
pub use class::{ClassDef, ClassDefIdx, ClassRef, ClassRefIdx};
pub use cst::Const;
pub use field::{FieldDesc, FieldIdx, StaticFieldDesc, StaticFieldIdx};
pub use fnsig::{FnSig, SigIdx};
pub use iter::{CILIter, CILIterElem};
pub use method::{MethodDef, MethodDefIdx, MethodImpl, MethodRef, MethodRefIdx};
pub use strings::StringIdx;
pub use tpe::float::Float;
pub use tpe::int::Int;
pub use tpe::{Type, TypeIdx};

use crate::IString;

pub mod access;
pub mod asm;
pub mod asm_link;
pub mod basic_block;
pub mod bimap;
pub mod builtins;
pub mod c_exporter;
pub mod cache;
pub mod cillyir_exporter;
pub mod cilnode;
pub mod cilroot;
pub mod class;
pub mod cst;
pub mod field;
pub mod fnsig;
/// Defines hashable and equable floating point types. All NaNs are compared by bits, and -0.0 != 0.0.
pub mod hashable;
pub mod il_exporter;
pub mod iter;
pub mod java_exporter;
pub mod macros;
pub mod method;
pub mod method_builder;
pub mod opt;
pub mod strings;
pub mod tpe;
pub mod typecheck;
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
    /// # Errors
    /// Returns an error if the export process failed.
    fn export(&self, asm: &Assembly, target: &Path) -> Result<(), Self::Error>;
}

#[test]
fn no_collision() {
    fn test_binop(asm: &mut Assembly, op: BinOp) -> CILNode {
        let mut curr: NodeIdx = IntoAsmIndex::into_idx(Const::I8(1), asm);
        for _ in 0..100 {
            curr = IntoAsmIndex::into_idx(asm.biop(curr, curr, op), asm);
        }
        asm[curr]
            .clone()
            .typecheck(asm.sig(vec![], Type::Void), &[], asm)
            .unwrap();
        asm[curr].clone()
    }
    let mut asm = Assembly::default();
    test_binop(&mut asm, BinOp::Add);
}
#[test]
fn test_binops() {
    fn test_binop(asm: &mut Assembly, op: BinOp) -> CILNode {
        let mut curr: NodeIdx = IntoAsmIndex::into_idx(Const::I8(1), asm);
        for _ in 0..10 {
            curr = IntoAsmIndex::into_idx(asm.biop(curr, curr, op), asm);
        }
        asm[curr]
            .clone()
            .typecheck(asm.sig(vec![], Type::Void), &[], asm)
            .unwrap();
        asm[curr].clone()
    }
    let mut asm = Assembly::default();
    test_binop(&mut asm, BinOp::Add);
    test_binop(&mut asm, BinOp::Sub);
    test_binop(&mut asm, BinOp::Mul);
    test_binop(&mut asm, BinOp::Rem);
}
pub trait IntoAsmIndex<Target> {
    fn into_idx(self, asm: &mut Assembly) -> Target;
}
impl<T> IntoAsmIndex<T> for T {
    fn into_idx(self, _: &mut Assembly) -> T {
        self
    }
}
impl IntoAsmIndex<StringIdx> for &str {
    fn into_idx(self, asm: &mut Assembly) -> StringIdx {
        asm.alloc_string(self)
    }
}
impl IntoAsmIndex<StringIdx> for IString {
    fn into_idx(self, asm: &mut Assembly) -> StringIdx {
        asm.alloc_string(self)
    }
}
impl IntoAsmIndex<StringIdx> for String {
    fn into_idx(self, asm: &mut Assembly) -> StringIdx {
        asm.alloc_string(self)
    }
}
impl IntoAsmIndex<TypeIdx> for Type {
    fn into_idx(self, asm: &mut Assembly) -> TypeIdx {
        asm.alloc_type(self)
    }
}
impl IntoAsmIndex<TypeIdx> for ClassRefIdx {
    fn into_idx(self, asm: &mut Assembly) -> TypeIdx {
        asm.alloc_type(Type::ClassRef(self))
    }
}
impl IntoAsmIndex<NodeIdx> for CILNode {
    fn into_idx(self, asm: &mut Assembly) -> NodeIdx {
        asm.alloc_node(self)
    }
}
impl IntoAsmIndex<NodeIdx> for Const {
    fn into_idx(self, asm: &mut Assembly) -> NodeIdx {
        asm.alloc_node(self)
    }
}
pub trait IntoIntType {
    fn int_type() -> Int;
}
pub trait IntoCillyType {
    fn cilly_type() -> Type;
}
impl<T: IntoIntType> IntoCillyType for T {
    fn cilly_type() -> Type {
        T::int_type().into()
    }
}
impl IntoIntType for usize {
    fn int_type() -> Int {
        Int::USize
    }
}
macro_rules! into_asm_index_closure {
    ($Target:ident) => {
        impl<'a, N: 'static + IntoAsmIndex<$Target>, F: FnOnce(&mut Assembly) -> N>
            IntoAsmIndex<$Target> for F
        {
            fn into_idx(self, asm: &mut Assembly) -> $Target {
                self(asm).into_idx(asm)
            }
        }
    };
}
into_asm_index_closure! {NodeIdx}
into_asm_index_closure! {RootIdx}
into_asm_index_closure! {StringIdx}

#[test]
fn add_macro() {
    let sum = binop!(
        CILNode::LdLoc(0),
        |asm| { asm.alloc_node(CILNode::LdLoc(0)) },
        crate::BinOp::Add
    );
}
