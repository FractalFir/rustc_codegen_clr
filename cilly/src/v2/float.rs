use serde::{Deserialize, Serialize};

use super::{
    cilnode::MethodKind,
    hashable::{HashableF32, HashableF64},
    Assembly, CILNode, ClassRef, Const, MethodRef, NodeIdx, Type,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Float {
    F16,
    F32,
    F64,
    F128,
}
impl Float {
    /// Returns a constant 0 of this float.
    pub fn zero(&self) -> Const {
        match self {
            Float::F16 => todo!(),
            Float::F32 => Const::F32(HashableF32(0.0)),
            Float::F64 => Const::F64(HashableF64(0.0)),
            Float::F128 => todo!(),
        }
    }
    /// Checks if this float is NaN
    pub fn is_nan(&self, val: NodeIdx, asm: &mut Assembly) -> NodeIdx {
        let is_nan = asm.alloc_string("IsNaN");
        let sig = asm.sig([Type::Float(*self)], Type::Bool);
        match self {
            Float::F16 => {
                let half = ClassRef::half(asm);
                let mref = asm.alloc_methodref(MethodRef::new(
                    half,
                    is_nan,
                    sig,
                    MethodKind::Static,
                    [].into(),
                ));
                asm.alloc_node(CILNode::Call(Box::new((mref, [val].into()))))
            }
            Float::F32 => {
                let single = ClassRef::single(asm);
                let mref = asm.alloc_methodref(MethodRef::new(
                    single,
                    is_nan,
                    sig,
                    MethodKind::Static,
                    [].into(),
                ));
                asm.alloc_node(CILNode::Call(Box::new((mref, [val].into()))))
            }
            Float::F64 => {
                let double = ClassRef::single(asm);
                let mref = asm.alloc_methodref(MethodRef::new(
                    double,
                    is_nan,
                    sig,
                    MethodKind::Static,
                    [].into(),
                ));
                asm.alloc_node(CILNode::Call(Box::new((mref, [val].into()))))
            }
            Float::F128 => todo!(),
        }
    }
    /// Returns a short name of the float
    pub fn name(&self) -> &str {
        match self {
            Float::F16 => "f16",
            Float::F32 => "f32",
            Float::F64 => "f64",
            Float::F128 => "f128",
        }
    }
}
impl From<Float> for Type {
    fn from(value: Float) -> Self {
        Type::Float(value)
    }
}
