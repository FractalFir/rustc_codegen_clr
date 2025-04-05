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
            SIMDElem::Int(int) => int.bits().unwrap_or(64), // Assumes a 64 bit target
            SIMDElem::Float(float) => float.bits(),
        }
    }
}
impl From<SIMDElem> for Type {
    fn from(val: SIMDElem) -> Self {
        match val {
            SIMDElem::Int(int) => Type::Int(int),
            SIMDElem::Float(float) => Type::Float(float),
        }
    }
}
impl From<Int> for SIMDElem {
    fn from(val: Int) -> Self {
        SIMDElem::Int(val)
    }
}
impl From<Float> for SIMDElem {
    fn from(val: Float) -> Self {
        SIMDElem::Float(val)
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
    /// Returns a short name descirbing this vector.
    /// ```
    /// # use cilly::tpe::simd::{SIMDElem,SIMDVector};
    /// # use cilly::Int;
    /// assert_eq!(SIMDVector::new(Int::U64.into(),4).name(),"simdu64_4");
    /// assert_eq!(SIMDVector::new(Int::U8.into(),32).name(),"simdu8_32");
    /// ```
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
    /// Returns the size of this SIMD vector in bits.
    /// ```
    /// # use cilly::tpe::simd::{SIMDElem,SIMDVector};
    /// # use cilly::{Float,Int};
    /// assert_eq!(SIMDVector::new(Float::F64.into(),4).bits(), 4 * Float::F64.bits() as u16);
    /// assert_eq!(SIMDVector::new(Int::U64.into(),4).bits(), 4 * Int::U64.bits().unwrap() as u16);
    /// ```
    pub fn bits(&self) -> u16 {
        self.elem.bits() as u16 * self.count() as u16
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
    /// Returns the ammount of elements in this [`SIMDVec`].
    /// ```
    /// # use cilly::tpe::simd::{SIMDElem,SIMDVector};
    /// # use cilly::{Float,Int};
    /// assert_eq!(SIMDVector::new(Float::F64.into(),4).count(), 4);
    /// assert_eq!(SIMDVector::new(Float::F64.into(),2).count(), 2);
    /// assert_eq!(SIMDVector::new(Int::U64.into(),8).count(),8);
    /// ```
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
