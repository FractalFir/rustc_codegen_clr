use serde::{Deserialize, Serialize};

use crate::{DotnetTypeRef, FnSig};
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

    /// Foregin type. Will never be interacted with directly
    Foreign,
    /// Generic argument
    GenericArg(u32),
    CallGenericArg(u32),
    DotnetChar,
    /// Rust `FnDefs`
    DelegatePtr(Box<FnSig>),
    /// Generic argument of a method
    MethodGenericArg(i32),
    ManagedArray {
        element: Box<Self>,
        dims: std::num::NonZeroU8,
    },
}
impl Type {
    /// If this is a reference to a dotnet type, return that type. Will not work with pointers/references.
    #[must_use]
    pub fn as_dotnet(&self) -> Option<DotnetTypeRef> {
        match self {
            Self::DotnetType(inner) => Some(inner.as_ref().clone()),
            _ => None,
        }
    }
    /// If this is a reference to a dotnet type, return that type. Works with pointers/references.
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
    pub const fn as_delegate_ptr(&self) -> Option<&crate::fn_sig::FnSig> {
        if let Self::DelegatePtr(v) = self {
            Some(v)
        } else {
            None
        }
    }
    #[must_use]
    /// Checks if a type can be operated on by CIL numeric instructions.
    pub const fn is_primitive_numeric(&self) -> bool {
        // match_same_arms disabled, since we want to document the variants explicitly.
        #[allow(clippy::match_same_arms)]
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
            Self::I128 | Self::U128 => true,
            _ => false,
        }
    }
}
impl From<DotnetTypeRef> for Type {
    fn from(value: DotnetTypeRef) -> Self {
        Self::DotnetType(Box::new(value))
    }
}
#[test]
fn type_repr_size_ok() {
    // Type should not grow without a good reason.
    assert!(std::mem::size_of::<Type>() <= 16);
}
#[macro_export]
macro_rules! ptr {
    ($tpe:expr) => {
        Type::Ptr(Box::new($tpe))
    };
}
