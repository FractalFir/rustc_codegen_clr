use serde::{Deserialize, Serialize};

use crate::{DotnetTypeRef, FnSig, IString};
#[derive(Serialize, Deserialize, PartialEq, Clone, Eq, Hash, Debug)]
pub enum Type {
    /// Void type
    Void,
    /// Boolean type
    Bool,
    // Floating-point types
    F16,
    F32,
    F64,
    // Unsigned intiegers
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
    // Signed intiegers
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    /// A refernece to a .NET type
    DotnetType(Box<DotnetTypeRef>),
    // Pointer to a type
    Ptr(Box<Self>),
    /// A managed reference `&`. IS NOT EQUIVALENT TO RUST `&`!
    ManagedReference(Box<Self>),
    // Speical type marking an unresoved type. This is a work around some issues with corelib types. Nothing can ever interact directly with this type.
    Unresolved,
    /// Foregin type. Will never be interacted with directly
    Foreign,
    /// Generic argument
    GenericArg(u32),
    CallGenericArg(u32),
    DotnetChar,
    /// Rust `FnDefs`
    FnDef(IString),
    DelegatePtr(Box<FnSig>),
    /// Generic argument of a method
    MethodGenericArg(i32),
    ManagedArray {
        element: Box<Self>,
        dims: std::num::NonZeroU8,
    },
}
impl Type {
    #[must_use]
    pub fn map_generic(&self, generics: &[Type]) -> Option<Type> {
        match self {
            Self::GenericArg(arg) => generics.get(*arg as usize).cloned(),
            Self::DotnetType(dref) => {
                let mut dref = dref.clone();
                let dref_generics: Option<Vec<_>> = dref
                    .generics()
                    .iter()
                    .map(|gtype| gtype.map_generic(generics))
                    .collect();
                dref.set_generics(dref_generics?);
                Some(Self::DotnetType(dref))
            }
            _ => Some(self.clone()),
        }
    }
    #[must_use]
    pub fn ref_to(&self) -> Self {
        match self {
            Self::DotnetType(dotnet) => todo!("Can't create reference to type {dotnet:?}"),
            _ => Self::Ptr(self.clone().into()),
        }
    }
    #[must_use]
    pub fn metadata(&self) -> Self {
        match self {
            Self::DotnetType(dotnet) => match dotnet.name_path() {
                "PtrComponents" => Type::USize,
                _ => Type::Void,
            },
            _ => Self::Void,
        }
    }
    #[must_use]
    pub fn as_dotnet(&self) -> Option<DotnetTypeRef> {
        match self {
            Self::DotnetType(inner) => Some(inner.as_ref().clone()),
            _ => None,
        }
    }
    #[must_use]
    pub fn dotnet_refs(&self) -> Option<DotnetTypeRef> {
        match self {
            Self::DotnetType(inner) => Some(inner.as_ref().clone()),
            Self::Ptr(inner) | Self::ManagedReference(inner) => inner.dotnet_refs(),
            _ => None,
        }
    }

    #[must_use]
    /// If this type is a pointer to a unction type, return its signature.
    pub fn as_delegate_ptr(&self) -> Option<&crate::fn_sig::FnSig> {
        if let Self::DelegatePtr(v) = self {
            Some(v)
        } else {
            None
        }
    }
    /// Checks if a type can be operated on by CIL numeric instructions.
    pub fn is_primitive_numeric(&self) -> bool {
        match self {
            Self::I8
            | Self::I16
            | Self::I32
            | Self::I64
            | Self::ISize
            | Self::U8
            | Self::U16
            | Self::U32
            | Self::U64
            | Self::USize => true,
            Self::Bool => true,
            Self::F32 | Self::F64 => true,
            Self::Ptr(_) => true,
            // 128 bit ints are NOT primitve CIL types!
            Self::I128 | Type::U128 => true,
            _ => false,
        }
    }
}
impl From<DotnetTypeRef> for Type {
    fn from(value: DotnetTypeRef) -> Self {
        Self::DotnetType(Box::new(value))
    }
}