use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    Assembly, ClassRefIdx, Float, Int, SigIdx,
};

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
                asm.get_type(*inner).iter_class_refs::<'a, 'asm>(asm)
            }
            Type::Int(_)
            | Type::Float(_)
            | Type::PlatformString
            | Type::PlatformChar
            | Type::PlatformGeneric(_, _)
            | Type::PlatformObject
            | Type::Bool
            | Type::Void => Box::new(std::iter::empty()),
            Type::FnPtr(sig) => Box::new(
                asm.get_sig(*sig)
                    .iter_types()
                    .flat_map(|tpe| tpe.iter_class_refs(asm).collect::<Box<_>>()),
            ),
            Type::ClassRef(cref) => Box::new(std::iter::once(*cref)),
        };
        tmp
    }
    #[must_use]
    pub fn deref<'a, 'b: 'a>(&'a self, asm: &'b Assembly) -> &Self {
        match self {
            Type::Ptr(inner) | Type::Ref(inner) => asm.get_type(*inner),
            _ => panic!(),
        }
    }
    /*
        pub fn from_v1(tpe: &crate::Type, asm: &mut Assembly) -> Type {
            match tpe {
                // We turn Foregin into void.
                crate::Type::Void | crate::Type::Foreign => Self::Void,
                crate::Type::Bool => Self::Bool,
                crate::Type::F16 => Float::F16.into(),
                crate::Type::Float(Float::F32) => Float::F32.into(),
                crate::Type::Float(Float::F64) => Float::F64.into(),
                crate::Type::Float(Float::F128) => Float::F128.into(),
                crate::Type::Int(Int::U8) => Int::U8.into(),
                crate::Type::Int(Int::U16) => Int::U16.into(),
                crate::Type::Int(Int::U32) => Int::U32.into(),
                crate::Type::Int(Int::U64) => Int::U64.into(),
                crate::Type::Int(Int::U128) => Int::U128.into(),
                crate::Type::Int(Int::USize) => Int::USize.into(),
                crate::Type::Int(Int::I8) => Int::I8.into(),
                crate::Type::Int(Int::I16) => Int::I16.into(),
                crate::Type::Int(Int::I32) => Int::I32.into(),
                crate::Type::Int(Int::I64) => Int::I64.into(),
                crate::Type::Int(Int::I128) => Int::I128.into(),
                crate::Type::Int(Int::ISize) => Int::ISize.into(),
                crate::Type::ClassRef(dotnet_type) => {
                    #[allow(clippy::single_match)]
                    if dotnet_type.asm() == Some("System.Runtime") {
                        match dotnet_type.name_path() {
                            "System.String" => return Type::PlatformString,
                            "System.Object" => return Type::PlatformObject,
                            _ => (),
                        }
                    }
                    let cref = ClassRef::from_v1(dotnet_type, asm);
                    let cref = asm.alloc_class_ref(cref);
                    cref.into()
                }
                crate::Type::Ptr(inner) => {
                    let inner = Self::from_v1(inner, asm);
                    asm.nptr(inner)
                }
                crate::Type::Ref(inner) => {
                    let inner = Self::from_v1(inner, asm);
                    asm.nref(inner)
                }

                crate::Type::GenericArg(arg) => Self::PlatformGeneric(*arg, GenericKind::TypeGeneric),
                crate::Type::CallGenericArg(arg) => {
                    Self::PlatformGeneric(*arg, GenericKind::CallGeneric)
                }
                crate::Type::DotnetChar => Self::PlatformChar,
                crate::Type::FnPtr(sig) => {
                    let sig = FnSig::from_v1(sig, asm);
                    Self::FnPtr(asm.alloc_sig(sig))
                }
                crate::Type::MethodGenericArg(_) => todo!(),
                crate::Type::ManagedArray { element, dims } => {
                    let element = Type::from_v1(element, asm);
                    let element = asm.alloc_type(element);
                    Self::PlatformArray {
                        elem: element,
                        dims: *dims,
                    }
                }
            }
        }
    */
    pub fn mangle(&self, asm: &Assembly) -> String {
        match self {
            Type::Ptr(inner) => format!("p{}", asm.get_type(*inner).mangle(asm)),
            Type::Ref(inner) => format!("r{}", asm.get_type(*inner).mangle(asm)),
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
                match cref.asm() {
                    Some(asm_name) => format!(
                        "{len}{asm_name}",
                        len = asm.get_string(asm_name).len(),
                        asm_name = asm.get_string(asm_name)
                    ),
                    None => "n".into(),
                }
            }
            Type::Float(float) => match float {
                Float::F16 => "f2".into(),
                Float::F32 => "f4".into(),
                Float::F64 => "f8".into(),
                Float::F128 => "f16".into(),
            },
            Type::PlatformString => "s".into(),
            Type::PlatformChar => "c".into(),
            Type::PlatformGeneric(_, _) => todo!(),
            Type::PlatformObject => "o".into(),
            Type::Bool => "b".into(),
            Type::Void => "v".into(),
            Type::PlatformArray { elem, dims } => format!(
                "a{dims}{elem}",
                elem = asm.get_type(*elem).mangle(asm),
                dims = dims.get()
            ),
            Type::FnPtr(sig) => {
                let sig = asm.get_sig(*sig);
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

    pub fn as_class_ref(&self) -> Option<ClassRefIdx> {
        if let Self::ClassRef(v) = self {
            Some(*v)
        } else {
            None
        }
    }
}
