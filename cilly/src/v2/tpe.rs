use std::num::NonZeroU8;

use serde::{Deserialize, Serialize};

use super::{bimap::HashWrapper, Assembly, ClassIdx, ClassRef, Float, FnSig, Int, SigIdx};

#[derive(Hash, PartialEq, Eq, Clone, Default, Copy, Debug, Serialize, Deserialize)]
pub struct TypeIdx(u64);
impl HashWrapper for TypeIdx {
    fn from_hash(val: u64) -> Self {
        Self(val)
    }
}
#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Type {
    Ptr(TypeIdx),
    Ref(TypeIdx),
    Int(Int),
    ClassRef(ClassIdx),
    Float(Float),
    PlatformChar,
    PlarformGeneric(u32, GenericKind),
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
    pub fn deref<'a, 'b: 'a>(&'a self, asm: &'b Assembly) -> &Self {
        match self {
            Type::Ptr(inner) | Type::Ref(inner) => asm.type_from_id(*inner),
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
                let cref = ClassRef::from_v1(dotnet_type, asm);
                let cref = asm.class_idx(cref);
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
            crate::Type::GenericArg(arg) => Self::PlarformGeneric(*arg, GenericKind::TypeGeneric),
            crate::Type::CallGenericArg(arg) => {
                Self::PlarformGeneric(*arg, GenericKind::CallGeneric)
            }
            crate::Type::DotnetChar => Self::PlatformChar,
            crate::Type::DelegatePtr(sig) => {
                let sig = FnSig::from_v1(sig, asm);
                Self::FnPtr(asm.sig_idx(sig))
            }
            crate::Type::MethodGenericArg(_) => todo!(),
            crate::Type::ManagedArray { element, dims } => {
                let element = Type::from_v1(element, asm);
                let element = asm.type_idx(element);
                Self::PlatformArray {
                    elem: element,
                    dims: *dims,
                }
            }
        }
    }
}
