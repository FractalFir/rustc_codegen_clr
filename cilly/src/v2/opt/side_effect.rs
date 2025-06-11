use std::collections::HashMap;

use crate::{bimap::Interned, cilnode::IsPure, Assembly, CILNode};
#[derive(Default)]
pub struct SideEffectInfoCache {
    side_effects: HashMap<Interned<CILNode>, bool>,
}
impl SideEffectInfoCache {
    /// Checks if a node may have side effects(if dupilcating it and poping the result would change the way a program runs).
    /// This also includes mutating values trough pointers in any way, shape or form.
    #[allow(clippy::match_same_arms)]
    pub fn has_side_effects(&mut self, node: Interned<CILNode>, asm: &Assembly) -> bool {
        if let Some(side_effect) = self.side_effects.get(&node) {
            return *side_effect;
        }
        let side_effect = match asm.get_node(node) {
            CILNode::LdTypeToken(_)
            | CILNode::LdFtn(_)
            | CILNode::Const(_)
            | CILNode::SizeOf(_) => false, // Constant, can't have side effects
            CILNode::BinOp(lhs, rhs, _) => {
                self.has_side_effects(*lhs, asm) || self.has_side_effects(*rhs, asm)
            }
            CILNode::UnOp(arg, _) => self.has_side_effects(*arg, asm), // UnOp, only has side effects if its arg has side effects
            CILNode::LdLoc(_) | CILNode::LdArg(_) => false, // Reading a variable has no side effects
            CILNode::LdLocA(_) | CILNode::LdArgA(_) => false, // Getting the address of something has no side effects.
            CILNode::Call(info) => {
                let (_, args, pure) = info.as_ref();
                // Impure call, may have side effects
                if *pure == IsPure::NOT {
                    return true;
                }
                // Check all args
                args.iter().any(|arg| self.has_side_effects(*arg, asm))
            }
            CILNode::RefToPtr(input)
            | CILNode::IntCast { input, .. }
            | CILNode::FloatCast { input, .. }
            | CILNode::PtrCast(input, _) => self.has_side_effects(*input, asm), // Casts don't have side effects, unless their input has one.
            CILNode::LdFieldAddress { addr, .. }
            | CILNode::LdField { addr, .. }
            | CILNode::LdInd { addr, .. } => self.has_side_effects(*addr, asm), // Reading a pointer or a field never has side effects.
            CILNode::GetException => true, // This is a low-level, unsafe operation, which manipulates the runtime stack, and can't be preformed twice. It for sure has side effects.
            CILNode::UnboxAny { object, .. }
            | CILNode::IsInst(object, _)
            | CILNode::CheckedCast(object, _) => {
                self.has_side_effects(*object, asm) // Class checks / casts / unboxes have no side effects.
            }
            CILNode::CallI(_) => true, // Indidrect calls may have side effects
            CILNode::LocAllocAlgined { .. } | CILNode::LocAlloc { .. } => true, // Allocation has side effects
            CILNode::LdStaticField(_) | CILNode::LdStaticFieldAddress(_) => false, // Loading static fields has no side effects.
            CILNode::LdLen(arr) => self.has_side_effects(*arr, asm), // Loading a length only has side effects if the index has array.
            CILNode::LdElelemRef { array, index } => {
                self.has_side_effects(*array, asm) || self.has_side_effects(*index, asm)
                // Indexing only has side effects if the index or array address has side effects.
            }
        };
        self.side_effects.insert(node, side_effect);
        side_effect
    }
}
#[test]
fn const_no_side_effect() {
    use crate::{
        hashable::{HashableF32, HashableF64},
        Const,
    };
    let consts = [
        true.into(),
        false.into(),
        Const::F32(HashableF32(std::f32::consts::PI)),
        Const::F64(HashableF64(std::f64::consts::PI)),
        Const::I8(5),
        Const::U8(5),
        Const::I16(5),
        Const::U16(5),
        Const::I32(5),
        Const::U32(5),
        Const::I64(5),
        Const::U64(5),
        //Const::I128(5),
        //Const::U128(5),
    ];
    let mut asm = Assembly::default();
    let mut cache = SideEffectInfoCache::default();
    for cst in consts {
        let node = asm.alloc_node(cst);
        assert!(!cache.has_side_effects(node, &asm));
        let node = asm.biop(cst, cst, crate::BinOp::Add);
        assert!(!cache.has_side_effects(node, &asm));
        let node = asm.biop(CILNode::LocAlloc { size: node }, cst, crate::BinOp::Add);
        assert!(cache.has_side_effects(node, &asm));
        let node = asm.biop(cst, CILNode::LocAlloc { size: node }, crate::BinOp::Add);
        assert!(cache.has_side_effects(node, &asm));
    }
}
#[test]
fn select_no_side_effects() {
    use crate::{Int, Type};
    let mut asm = Assembly::default();
    let a = crate::cil_node::V1Node::V2(asm.alloc_node(1_usize));
    let b = crate::cil_node::V1Node::V2(asm.alloc_node(2_usize));
    let predictate = crate::cil_node::V1Node::LDLoc(0);
    let v1 = crate::cil_node::V1Node::select(Type::Int(Int::USize), a, b, predictate, &mut asm);
    let v2 = CILNode::from_v1(&v1, &mut asm);
    let v2 = asm.alloc_node(v2);
    let mut cache = SideEffectInfoCache::default();
    assert!(!cache.has_side_effects(v2, &asm), "v2:{:?}", asm[v2]);
}
