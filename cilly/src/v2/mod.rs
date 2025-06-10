//#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
// This lint includes tests for some bizzare reason, so ignoring it seems like the best course of action
#![allow(clippy::missing_panics_doc)]
//#![warn(missing_docs)]
pub use super::bimap::Interned;
use std::path::Path;

pub use crate::Access;
pub use asm::{Assembly, IlasmFlavour};
pub use basic_block::BasicBlock;
pub use bimap::BiMap;
pub use cilnode::{BinOp, CILNode};
pub use cilroot::{BranchCond, CILRoot};
pub use class::{ClassDef, ClassRef};
pub use cst::Const;
pub use field::{FieldDesc, StaticFieldDesc};
pub use fnsig::FnSig;
pub use iter::{CILIter, CILIterElem};
pub use method::{MethodDef, MethodDefIdx, MethodImpl, MethodRef};

pub use tpe::float::Float;
pub use tpe::int::Int;
pub use tpe::Type;

use crate::IString;
/// Assembly
pub mod asm;
/// Linker-related code
pub mod asm_link;
/// Basic block - the building block of control-flow
pub mod basic_block;
/// Interning code
pub mod bimap;
/// Builtin intrinsics
pub mod builtins;
/// Code exporting C source files
pub mod c_exporter;
/// Exports modules to IR builders. Used for quickly implementing intrinsics
pub mod cillyir_exporter;
pub mod cilnode;
pub mod cilroot;
/// Definitons of a value / byref type
pub mod class;
/// IR constant
pub mod cst;
/// IR field
pub mod field;
/// IR functions signature
pub mod fnsig;
/// Defines hashable and equable floating point types. All NaNs are compared by bits, and -0.0 != 0.0.
pub mod hashable;
/// Exports IR to .NET bytecode
pub mod il_exporter;
/// IR iterator
pub mod iter;
/// Exports IR to JVM bytecode
pub mod java_exporter;
pub mod macros;
/// IR functions
pub mod method;
/// IR function builder
pub mod method_builder;
/// IR optimization functions
pub mod opt;
/// IR type repr
pub mod tpe;
/// IR typechecker
pub mod typecheck;
#[test]
fn types() {
    let mut asm = Assembly::default();
    let tpe = asm.nptr(Type::Int(Int::U8));
    let tpe = asm.nref(tpe);
    assert_eq!(*tpe.deref(&asm).deref(&asm), Type::Int(Int::U8));
}
#[test]
fn nodes() {
    let mut asm = Assembly::default();
    let add = asm.biop(Const::I8(2), Const::I8(1), BinOp::Add);
    let mut iter = CILIter::new(asm[add].clone(), &asm);
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
/// IL exporter
pub trait Exporter {
    /// Export error
    type Error: std::fmt::Debug;
    /// # Errors
    /// Returns an error if the export process failed.
    fn export(&mut self, asm: &Assembly, target: &Path) -> Result<(), Self::Error>;
}

#[test]
fn test_binops() {
    fn test_binop(asm: &mut Assembly, op: BinOp) -> CILNode {
        let mut curr: Interned<CILNode> = IntoAsmIndex::into_idx(Const::I8(1), asm);
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
impl IntoAsmIndex<Interned<IString>> for &str {
    fn into_idx(self, asm: &mut Assembly) -> Interned<IString> {
        asm.alloc_string(self)
    }
}
impl IntoAsmIndex<Interned<IString>> for IString {
    fn into_idx(self, asm: &mut Assembly) -> Interned<IString> {
        asm.alloc_string(self)
    }
}
impl IntoAsmIndex<Interned<IString>> for String {
    fn into_idx(self, asm: &mut Assembly) -> Interned<IString> {
        asm.alloc_string(self)
    }
}
impl IntoAsmIndex<Interned<Type>> for Type {
    fn into_idx(self, asm: &mut Assembly) -> Interned<Type> {
        asm.alloc_type(self)
    }
}
impl IntoAsmIndex<Interned<Type>> for Int {
    fn into_idx(self, asm: &mut Assembly) -> Interned<Type> {
        asm.alloc_type(self)
    }
}
impl IntoAsmIndex<Interned<Type>> for Interned<ClassRef> {
    fn into_idx(self, asm: &mut Assembly) -> Interned<Type> {
        asm.alloc_type(Type::ClassRef(self))
    }
}
impl IntoAsmIndex<Interned<CILNode>> for CILNode {
    fn into_idx(self, asm: &mut Assembly) -> Interned<CILNode> {
        asm.alloc_node(self)
    }
}
impl IntoAsmIndex<Interned<CILNode>> for Const {
    fn into_idx(self, asm: &mut Assembly) -> Interned<CILNode> {
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
    ($Target:ty) => {
        impl<'a, N: 'static + IntoAsmIndex<$Target>, F: FnOnce(&mut Assembly) -> N>
            IntoAsmIndex<$Target> for F
        {
            fn into_idx(self, asm: &mut Assembly) -> $Target {
                self(asm).into_idx(asm)
            }
        }
    };
}
into_asm_index_closure! {Interned<CILNode>}
into_asm_index_closure! {Interned<CILRoot>}
into_asm_index_closure! {Interned<IString>}

#[test]
fn add_macro() {
    let _sum = crate::binop!(
        CILNode::LdLoc(0),
        |asm| { asm.alloc_node(CILNode::LdLoc(0)) },
        crate::BinOp::Add
    );
}

pub fn branch_cond_to_name(
    target: u32,
    sub_target: u32,
    has_handler: bool,
    is_handler: bool,
) -> String {
    if sub_target == 0 {
        format!("bb{}", target)
    } else if is_handler {
        format!("h{}_{}", target, sub_target)
    } else if has_handler {
        format!("jp{}_{}", target, sub_target)
    }
    // If the handler was removed, we can just jump directly to our target
    else {
        format!("bb{}", sub_target)
    }
}
