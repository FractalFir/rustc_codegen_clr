use serde::{Deserialize, Serialize};

use crate::{ClassRef, ClassRefIdx, Float, Int};

use super::Type;
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SIMDElem {
    Int(Int),
    Float(Float),
}
impl SIMDElem {
    fn bits(&self) -> u8 {
        match self {
            SIMDElem::Int(int) => int.bits(),
            SIMDElem::Float(float) => float.bits(),
        }
    }
}
impl Into<Type> for SIMDElem {
    fn into(self) -> Type {
        match self {
            SIMDElem::Int(int) => Type::Int(int),
            SIMDElem::Float(float) => Type::Float(float),
        }
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct SIMDVector {
    elem: SIMDElem,
    count: u8,
}
impl SIMDVector {
    pub fn new(elem: SIMDElem, count: u8) -> Self {
        let res = Self { elem, count };
        let bits = res.bits();
        assert!(
            bits == 64 || bits == 128 || bits == 256 || bits == 512,
            "A vec with {count} {elem:?} has the size of {bits}, which is not supported."
        );
        res
    }

    pub fn name(&self) -> String {
        match self.elem {
            SIMDElem::Int(elem) => {
                format!("simd{elem}_{count}", elem = elem.name(), count = self.count)
            }
            SIMDElem::Float(elem) => {
                format!("simd{elem}_{count}", elem = elem.name(), count = self.count)
            }
        }
    }

    pub fn bits(&self) -> u16 {
        self.elem.bits() as u16 * self.count as u16
    }

    pub fn elem(&self) -> SIMDElem {
        self.elem
    }

    pub fn class(&self, asm: &mut crate::Assembly) -> ClassRefIdx {
        let elem = self.elem().into();
        let asm_name = asm.alloc_string("System.Runtime.Intrinsics");
        let name = asm.alloc_string(format!("System.Runtime.Intrinsics.Vector{}", self.bits()));
        asm.alloc_class_ref(ClassRef::new(name, Some(asm_name), true, vec![elem].into()))
    }
    pub fn extension_class(&self, asm: &mut crate::Assembly) -> ClassRefIdx {
        let asm_name = asm.alloc_string("System.Runtime.Intrinsics");
        let name = asm.alloc_string(format!("System.Runtime.Intrinsics.Vector{}", self.bits()));
        asm.alloc_class_ref(ClassRef::new(name, Some(asm_name), false, vec![].into()))
    }

    pub fn count(&self) -> u8 {
        self.count
    }
}
impl TryInto<SIMDElem> for Type {
    type Error = ();

    fn try_into(self) -> Result<SIMDElem, Self::Error> {
        match self {
            Type::Int(int) => Ok(SIMDElem::Int(int)),
            Type::Float(float) => Ok(SIMDElem::Float(float)),
            _ => Err(()),
        }
    }
}
