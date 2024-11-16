use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};
use simd::SIMDVector;

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    Assembly, ClassRefIdx, Float, Int, SigIdx,
};

pub mod float;
pub mod int;
pub mod simd;
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TypeIdx(BiMapIndex);
impl IntoBiMapIndex for TypeIdx {
    fn from_index(val: BiMapIndex) -> Self {
        Self(val)
    }

    fn as_bimap_index(&self) -> BiMapIndex {
        self.0
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Type {
    Ptr(TypeIdx),
    Ref(TypeIdx),
    Int(Int),
    ClassRef(ClassRefIdx),
    Float(Float),
    PlatformString,
    PlatformChar,
    PlatformGeneric(u32, GenericKind),
    PlatformObject,
    Bool,
    Void,
    PlatformArray { elem: TypeIdx, dims: NonZeroU8 },
    FnPtr(SigIdx),
    SIMDVector(SIMDVector),
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum GenericKind {
    MethodGeneric,
    CallGeneric,
    TypeGeneric,
}
impl Type {
    pub fn iter_class_refs<'a, 'asm: 'a>(
        &'a self,
        asm: &'asm Assembly,
    ) -> impl Iterator<Item = ClassRefIdx> + 'a {
        let tmp: Box<dyn Iterator<Item = ClassRefIdx>> = match self {
            Type::PlatformArray { elem: inner, .. } | Type::Ptr(inner) | Type::Ref(inner) => {
                asm[*inner].iter_class_refs::<'a, 'asm>(asm)
            }
            Type::Int(_)
            | Type::Float(_)
            | Type::PlatformString
            | Type::PlatformChar
            | Type::PlatformGeneric(_, _)
            | Type::PlatformObject
            | Type::Bool
            | Type::Void
            | Type::SIMDVector(_) => Box::new(std::iter::empty()),
            Type::FnPtr(sig) => Box::new(
                asm[*sig]
                    .iter_types()
                    .flat_map(|tpe| tpe.iter_class_refs(asm).collect::<Box<_>>()),
            ),
            Type::ClassRef(cref) => Box::new(std::iter::once(*cref)),
        };
        tmp
    }
    #[must_use]
    pub fn deref<'a, 'b: 'a>(&'a self, asm: &'b Assembly) -> &'a Self {
        match self {
            Type::Ptr(inner) | Type::Ref(inner) => &asm[*inner],
            _ => panic!(),
        }
    }
    /// Returns a mangled ASCI representation of this type.
    /// ```
    /// # use cilly::*;
    /// # use cilly::v2::Int;
    /// # let asm = cilly::v2::Assembly::default();
    /// assert_eq!(Type::PlatformString.mangle(&asm),"st");
    /// assert_eq!(Type::Int(Int::I128).mangle(&asm),"i16");
    /// ```
    #[must_use]
    pub fn mangle(&self, asm: &Assembly) -> String {
        match self {
            Type::SIMDVector(val) => val.name(),
            Type::Ptr(inner) => format!("p{}", asm[*inner].mangle(asm)),
            Type::Ref(inner) => format!("r{}", asm[*inner].mangle(asm)),
            Type::Int(int) => match int {
                Int::U8 => "u1".into(),
                Int::U16 => "u2".into(),
                Int::U32 => "u4".into(),
                Int::U64 => "u8".into(),
                Int::U128 => "u16".into(),
                Int::USize => "us".into(),
                Int::I8 => "i1".into(),
                Int::I16 => "i2".into(),
                Int::I32 => "i4".into(),
                Int::I64 => "i8".into(),
                Int::I128 => "i16".into(),
                Int::ISize => "is".into(),
            },
            Type::ClassRef(cref) => {
                let cref = asm.class_ref(*cref);
                let asm_name = match cref.asm() {
                    Some(asm_name) => format!(
                        "{len}{asm_name}",
                        len = asm[asm_name].len(),
                        asm_name = &asm[asm_name]
                    ),
                    None => "n".into(),
                };
                let name = &asm[cref.name()];
                format!("{asm_name}{len}{name}", len = name.len())
            }
            Type::Float(float) => match float {
                Float::F16 => "f2".into(),
                Float::F32 => "f4".into(),
                Float::F64 => "f8".into(),
                Float::F128 => "f16".into(),
            },
            Type::PlatformString => "st".into(),
            Type::PlatformChar => "c".into(),
            Type::PlatformGeneric(_, _) => todo!(),
            Type::PlatformObject => "o".into(),
            Type::Bool => "b".into(),
            Type::Void => "v".into(),
            Type::PlatformArray { elem, dims } => format!(
                "a{dims}{elem}",
                elem = asm[*elem].mangle(asm),
                dims = dims.get()
            ),
            Type::FnPtr(sig) => {
                let sig = &asm[*sig];
                let argc = sig.inputs().len();
                let output = sig.output().mangle(asm);
                let inputs = sig
                    .inputs()
                    .iter()
                    .map(|input| input.mangle(asm))
                    .collect::<String>();
                format!("{argc}{inputs}{output}")
            }
        }
    }
    #[must_use]
    /// If this type is a class reference, returns that class reference.
    /// ```
    /// # use cilly::*;
    /// # use cilly::v2::{ClassRef};
    /// # let mut asm = cilly::v2::Assembly::default();
    /// let uint_128 = ClassRef::uint_128(&mut asm);
    /// assert_eq!(Type::ClassRef(uint_128).as_class_ref(),Some(uint_128));
    /// assert_eq!(Type::Int(Int::U8).as_class_ref(),None);
    /// ```
    pub fn as_class_ref(&self) -> Option<ClassRefIdx> {
        if let Self::ClassRef(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    /// If this type is a pointer(*T) or a reference(&T), returns the pointed type(T).
    /// ```
    /// # use cilly::*;
    /// # let mut asm = cilly::v2::Assembly::default();
    /// # let u8_tpe = asm.alloc_type(Type::Int(Int::U8));
    /// assert_eq!(asm.nptr(u8_tpe).pointed_to(),Some(u8_tpe));
    /// assert_eq!(asm.nref(u8_tpe).pointed_to(),Some(u8_tpe));
    /// assert_eq!(Type::Int(Int::U8).pointed_to(),None);
    /// ```
    pub fn pointed_to(&self) -> Option<TypeIdx> {
        match self {
            Type::Ptr(type_idx) | Type::Ref(type_idx) => Some(*type_idx),
            _ => None,
        }
    }
    /// Checks if this can be assigned to another type.
    /// ```
    /// # use cilly::*;
    /// # use cilly::v2::{ClassRef,Int};
    /// # let mut asm = cilly::v2::Assembly::default();
    /// // You can assign a string to an object.
    /// let ps = Type::PlatformString;
    /// let obj = Type::PlatformObject;
    /// // All non-valuetype classes can be assigned to an object.
    /// assert!(ps.is_assignable_to(obj,&asm));
    /// // Valuetype, so can't be directly assigned to an object(it needs to be boxed first).
    /// assert!(!Type::ClassRef(ClassRef::int_128(&mut asm)).is_assignable_to(obj,&asm));
    /// // But you can't assign an object to a string.
    /// assert!(!obj.is_assignable_to(ps,&asm));
    /// // Types are always assignable to themselves.
    /// assert!(Type::Bool.is_assignable_to(Type::Bool,&asm));
    /// // A class ref to int_128 is assignable to Int::I128
    /// assert!(Type::Int(Int::I128).is_assignable_to(Type::ClassRef(ClassRef::int_128(&mut asm)),&asm));
    /// // A class ref to uint_128 is assignable to Int::U128
    /// assert!(Type::Int(Int::U128).is_assignable_to(Type::ClassRef(ClassRef::uint_128(&mut asm)),&asm));
    /// // You can assign a *T to a &T, but not the other way round.
    /// # let refu8 = asm.nref(Int::U8);
    /// # let ptru8 = asm.nptr(Int::U8);
    /// assert!(ptru8.is_assignable_to(refu8,&asm));
    /// assert!(!refu8.is_assignable_to(ptru8,&asm));
    /// //     Ignores partial matches.
    /// # let u128_name = asm.alloc_string("System.UInt128");
    /// # let i128_name = asm.alloc_string("System.Int128");
    /// # let system_runtime = Some(asm.alloc_string("System.Runtime"));
    /// # let string_name = asm.alloc_string("System.String");
    /// // Has the right name and is in the right assembly, but the valuetype is not right.
    /// assert!(!Type::Int(Int::U128).is_assignable_to(Type::ClassRef(asm.alloc_class_ref(ClassRef::new(u128_name, system_runtime, false, [].into()))),&asm));
    /// assert!(!Type::Int(Int::I128).is_assignable_to(Type::ClassRef(asm.alloc_class_ref(ClassRef::new(i128_name, system_runtime, false, [].into()))),&asm));
    /// assert!(!Type::PlatformString.is_assignable_to(Type::ClassRef(asm.alloc_class_ref(ClassRef::new(string_name, system_runtime, true, [].into()))),&asm));
    /// // Has the right assembly, valuetype, but the wrong name
    /// assert!(!Type::Int(Int::I128).is_assignable_to(Type::ClassRef(asm.alloc_class_ref(ClassRef::new(string_name, system_runtime, true, [].into()))),&asm));
    /// assert!(!Type::Int(Int::U128).is_assignable_to(Type::ClassRef(asm.alloc_class_ref(ClassRef::new(string_name, system_runtime, true, [].into()))),&asm));
    /// assert!(!Type::PlatformString.is_assignable_to(Type::ClassRef(asm.alloc_class_ref(ClassRef::new(u128_name, system_runtime, false, [].into()))),&asm));
    /// ```
    pub fn is_assignable_to(&self, to: Type, asm: &Assembly) -> bool {
        if *self == to {
            return true;
        }
        match (*self, to) {
            (Type::PlatformString, Type::PlatformObject) => true,
            (Type::ClassRef(cref), Type::PlatformObject) => {
                let cref = asm.class_ref(cref);
                !cref.is_valuetype()
            }
            (Type::ClassRef(cref), Type::PlatformString)
            | (Type::PlatformString, Type::ClassRef(cref)) => {
                let cref = asm.class_ref(cref);
                !cref.is_valuetype()
                    && cref.asm().map(|s| asm[s].as_ref()) == Some("System.Runtime")
                    && &asm[cref.name()] == "System.String"
            }
            (Type::ClassRef(cref), Type::Int(Int::I128))
            | (Type::Int(Int::I128), Type::ClassRef(cref)) => {
                let cref = asm.class_ref(cref);
                cref.is_valuetype()
                    && cref.asm().map(|s| asm[s].as_ref()) == Some("System.Runtime")
                    && &asm[cref.name()] == "System.Int128"
            }
            (Type::ClassRef(cref), Type::Int(Int::U128))
            | (Type::Int(Int::U128), Type::ClassRef(cref)) => {
                let cref = asm.class_ref(cref);
                cref.is_valuetype()
                    && cref.asm().map(|s| asm[s].as_ref()) == Some("System.Runtime")
                    && &asm[cref.name()] == "System.UInt128"
            }
            (Type::Int(Int::U16 | Int::I16), Type::PlatformChar) => true,
            (Type::Ptr(ptr), Type::Ref(rf)) => ptr == rf,
            // TODO: check generics propely?
            (_, Type::PlatformGeneric(_, _)) => true,
            _ => false,
        }
    }
    /// If this type is an int, return that int.
    /// ```
    /// # use cilly::v2::Int;
    /// # use cilly::*;
    /// let tpe = Type::PlatformString;
    /// // Not an int, so this returns none.
    /// assert_eq!(tpe.as_int(),None);
    /// let tpe = Type::Int(Int::ISize);
    /// // An int, so this returns Some.
    /// assert_eq!(tpe.as_int(),Some(Int::ISize));
    /// ```
    pub fn as_int(&self) -> Option<Int> {
        if let Self::Int(v) = self {
            Some(*v)
        } else {
            None
        }
    }
    /// If this type is an [`SIMDVector`], return that SIMDVector.
    /// ```
    /// # use cilly::v2::tpe::simd::{SIMDElem,SIMDVector};
    /// # use cilly::*;
    /// let vec = SIMDVector::new(Int::U64.into(),4);
    /// assert_eq!(Type::SIMDVector(vec).as_simdvector(),Some(&vec));
    /// assert_eq!(Type::Int(Int::U64).as_simdvector(),None);
    /// ```
    pub fn as_simdvector(&self) -> Option<&SIMDVector> {
        if let Self::SIMDVector(v) = self {
            Some(v)
        } else {
            None
        }
    }
}