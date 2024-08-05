use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use super::{
    bimap::{BiMapIndex, IntoBiMapIndex},
    Assembly, ClassRef, ClassRefIdx, Float, FnSig, Int, SigIdx,
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
    pub fn deref<'a, 'b: 'a>(&'a self, asm: &'b Assembly) -> &Self {
        match self {
            Type::Ptr(inner) | Type::Ref(inner) => asm.get_type(*inner),
            _ => panic!(),
        }
    }

    pub(crate) fn from_v1(tpe: &crate::Type, asm: &mut Assembly) -> Type {
        match tpe {
            crate::Type::Void => Self::Void,
            crate::Type::Bool => Self::Bool,
            crate::Type::F16 => Float::F16.into(),
            crate::Type::F32 => Float::F32.into(),
            crate::Type::F64 => Float::F64.into(),
            crate::Type::U8 => Int::U8.into(),
            crate::Type::U16 => Int::U16.into(),
            crate::Type::U32 => Int::U32.into(),
            crate::Type::U64 => Int::U64.into(),
            crate::Type::U128 => Int::U128.into(),
            crate::Type::USize => Int::USize.into(),
            crate::Type::I8 => Int::I8.into(),
            crate::Type::I16 => Int::I16.into(),
            crate::Type::I32 => Int::I32.into(),
            crate::Type::I64 => Int::I64.into(),
            crate::Type::I128 => Int::I128.into(),
            crate::Type::ISize => Int::ISize.into(),
            crate::Type::DotnetType(dotnet_type) => {
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
            crate::Type::ManagedReference(inner) => {
                let inner = Self::from_v1(inner, asm);
                asm.nref(inner)
            }
            crate::Type::Foreign => Self::Void,
            crate::Type::GenericArg(arg) => Self::PlatformGeneric(*arg, GenericKind::TypeGeneric),
            crate::Type::CallGenericArg(arg) => {
                Self::PlatformGeneric(*arg, GenericKind::CallGeneric)
            }
            crate::Type::DotnetChar => Self::PlatformChar,
            crate::Type::DelegatePtr(sig) => {
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
}
